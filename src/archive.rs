use std::error::Error;
use std::fs::File;
use std::io::{ErrorKind, Read, Write};
use std::path::{Path, PathBuf};
use std::{fs, process};

use ansi_term::Color::Red;
use zip::write::FileOptions;
use zip::{ZipArchive, ZipWriter};

use crate::util;

/// Extracts the extension file and copies it's contents to [build_dir].
pub fn extract_aix(aix_path: &Path) -> PathBuf {
    let output_dir = util::data_dir().join("temp");

    let aix = open_file(aix_path, false);

    let mut archive = match ZipArchive::new(aix) {
        Ok(zip) => zip,
        Err(err) => {
            eprintln!(
                "     {} Unable to open {}. Reason: {}",
                Red.paint("error"),
                aix_path.to_str().unwrap(),
                err.to_string()
            );
            process::exit(1);
        }
    };

    // Extract the aix file
    if let Err(err) = archive.extract(&output_dir) {
        eprintln!(
            "     {} Unable to extract {}. Reason: {}",
            Red.paint("error"),
            aix_path.to_str().unwrap(),
            err.to_string()
        );
        process::exit(1);
    }

    output_dir.join(base_dir_from_aix(archive))
}

/// Packs the new jetified AIX in the given output directory.
pub fn pack_aix(base_dir: &Path, output_dir: &Path) {
    let org_name = base_dir.file_name().unwrap().to_str().unwrap();

    // x represents AndroidX...
    let x_aix_path = output_dir.join(format!("{}.x.aix", org_name));

    // If the ...x.aix already exists in the output dir, delete it.
    if x_aix_path.exists() {
        if let Err(err) = fs::remove_file(x_aix_path.as_path()) {
            eprintln!(
                "     {} Unable to create {}. Reason: {}",
                Red.paint("error"),
                x_aix_path.as_path().to_str().unwrap(),
                err.to_string()
            );
            process::exit(1);
        }
    }

    // Create the new ...x.aix
    let aix = open_file(x_aix_path.as_path(), true);

    // Initialize the zip writer
    let mut zip_writer = ZipWriter::new(aix);

    // Create the AIX
    let result = archive_from_path(&mut zip_writer, base_dir, base_dir.parent().unwrap());
    if let Err(err) = result {
        eprintln!(
            "     {} Unable to create {}. Reason: {}",
            Red.paint("error"),
            x_aix_path.as_path().to_str().unwrap(),
            err.to_string()
        );
        process::exit(1);
    };

    // Delete the base directory once the extension is packed
    fs::remove_dir_all(base_dir).unwrap();
}

/// Recursively generates an archive from the given [path]
fn archive_from_path(
    zip_writer: &mut ZipWriter<File>,
    path: &Path,
    output_dir: &Path,
) -> Result<(), Box<dyn Error>> {
    if path.is_dir() {
        // Relative path of the directory from the output directory.
        let path_rel = path.strip_prefix(output_dir)?.to_str().unwrap();

        // `\` must be replaced with `/` otherwise the builder won't be able to
        // locate files.
        zip_writer.add_directory(path_rel.replace("\\", "/"), FileOptions::default())?;

        // List all the entities in this directory
        let entities = fs::read_dir(path)?;

        // Then recursive add all the entities to the archive
        for entity in entities {
            let entity_path = entity?.path();

            archive_from_path(zip_writer, entity_path.as_path(), output_dir)?;
        }
    } else {
        let file = open_file(path, false);
        let contents = contents_as_bytes(file)?;

        // Relative path of the file from the output directory.
        let path_rel = path.strip_prefix(output_dir)?.to_str().unwrap();

        // Add this file in the archive
        zip_writer.start_file(path_rel.replace("\\", "/"), FileOptions::default())?;

        // Then write out it's contents
        zip_writer.write_all(contents.as_slice())?;
    }

    Ok(())
}

/// Reads the given file and returns it's contents as bytes.
fn contents_as_bytes(mut file: File) -> Result<Vec<u8>, Box<dyn Error>> {
    let metadata = file.metadata()?;

    let mut contents: Vec<u8> = Vec::with_capacity(metadata.len() as usize + 1);

    file.read_to_end(&mut contents)?;

    Ok(contents)
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
                            eprintln!(
                                "     {} Unable to create file {}. Reason: {}",
                                Red.paint("error"),
                                path.to_str().unwrap(),
                                err.to_string()
                            );
                            process::exit(1);
                        }
                    }
                } else {
                    eprintln!(
                        "     {} File {} not found",
                        Red.paint("error"),
                        path.to_str().unwrap()
                    );
                    process::exit(2);
                }
            }
            _ => {
                eprintln!(
                    "     {} Unable to open file {}. Reason: {}",
                    Red.paint("error"),
                    path.to_str().unwrap(),
                    err.to_string()
                );
                process::exit(1);
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
