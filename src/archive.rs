use std::error::Error;
use std::fs::{File, ReadDir};
use std::io::{ErrorKind, Read, Write};
use std::path::{Path, PathBuf};
use std::{fs, process};

use zip::write::FileOptions;
use zip::{ZipArchive, ZipWriter};

/// Extracts the extension file and copies it's contents to [output_dir].
pub fn extract_aix(aix_path: &Path, output_dir: &Path) -> PathBuf {
    let output_dir = ensure_dir(output_dir);

    let aix = open_file(aix_path, false);

    let mut archive = match ZipArchive::new(aix) {
        Ok(zip) => zip,
        Err(err) => {
            eprintln!(
                "Something went wrong while trying to open the extension file {}",
                aix_path.display()
            );
            eprintln!("{:?}", err);
            process::exit(1)
        }
    };

    // Extract the aix file
    if let Err(err) = archive.extract(output_dir) {
        eprintln!(
            "Something went wrong while trying to open the extension file {}",
            aix_path.display()
        );
        eprintln!("{:?}", err);
        process::exit(1)
    }

    output_dir.join(base_dir_from_aix(archive))
}

pub fn pack_aix(base_dir: &Path, output: &Path) {
    let base_name = base_dir.file_name().unwrap().to_str().unwrap();

    let aix_path = output.join(format!("{}.x.aix", base_name));
    fs::remove_file(aix_path.as_path());

    let aix = open_file(aix_path.as_path(), true);

    let mut zip_writer = ZipWriter::new(aix);

    let paths = fs::read_dir(base_dir).unwrap();

    write_to_archive(&mut zip_writer, paths, base_name).unwrap();
}

fn write_to_archive(
    zip_writer: &mut ZipWriter<File>,
    paths: ReadDir,
    in_zip_parent_dir: &str,
) -> Result<(), Box<dyn Error>> {
    zip_writer.add_directory(in_zip_parent_dir, FileOptions::default())?;

    for path in paths {
        let path = path?.path();

        if path.is_dir() {
            let dir_name = path.file_name().unwrap().to_str().unwrap();

            let paths = fs::read_dir(&path)?;

            write_to_archive(
                zip_writer,
                paths,
                &*format!("{}/{}", in_zip_parent_dir, dir_name),
            )?
        } else {
            let file = open_file(path.as_path(), false);
            let contents = contents(file)?;

            let path_in_archive = format!(
                "{}/{}",
                in_zip_parent_dir,
                path.file_name().unwrap().to_str().unwrap()
            );

            zip_writer.start_file(path_in_archive, FileOptions::default())?;
            zip_writer.write_all(contents.as_slice())?;
        }
    }

    Ok(())
}

fn contents(mut file: File) -> Result<Vec<u8>, Box<dyn Error>> {
    let metadata = file.metadata()?;

    let mut contents: Vec<u8> = Vec::with_capacity(metadata.len() as usize + 1);

    file.read_to_end(&mut contents)?;

    Ok(contents)
}

/// Creates the build directory if it doesn't already exists.
fn ensure_dir(path: &Path) -> &Path {
    // Create the directory if it doesn't already exists.
    if !path.exists() {
        match fs::create_dir_all(path) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Something went wrong");
                eprintln!("{}", err);
                process::exit(1);
            }
        };
    }

    // Exit if `path` is not a path to a directory.
    if !path.is_dir() {
        eprintln!("{} is not a directory", path.display());
        process::exit(1);
    }

    path
}

/// Opens and returns the file from [path].
fn open_file(path: &Path, create_if_req: bool) -> File {
    match File::open(path) {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => {
                if create_if_req {
                    match File::create(path) {
                        Ok(file) => file,
                        Err(err) => {
                            eprintln!("Unable to create file {}; reason: {}", path.display(), err);
                            process::exit(2);
                        }
                    }
                } else {
                    eprintln!("Extension file {} not found", path.display());
                    process::exit(2);
                }
            }
            ErrorKind::PermissionDenied => {
                eprintln!(
                    "Permission denied; unable to access extension file {}",
                    path.display()
                );
                process::exit(2);
            }
            _ => {
                eprintln!(
                    "Something went wrong while trying to open the extension file {}",
                    path.display()
                );
                eprintln!("{}", err);
                process::exit(2);
            }
        },
    }
}

/// Returns the base dir of the extension. This is same as the extension's package.
fn base_dir_from_aix(archive: ZipArchive<File>) -> PathBuf {
    let ext_props_path = archive
        .file_names()
        .into_iter()
        .find(|x| x.ends_with(".properties"))
        .unwrap();

    Path::new(ext_props_path).parent().unwrap().to_owned()
}
