use std::{fs, process};
use std::error::Error;
use std::fs::File;
use std::io::{ErrorKind, Read, Write};
use std::path::{Path, PathBuf};

use ansi_term::Color::Red;
use zip::{ZipArchive, ZipWriter};
use zip::write::FileOptions;

use crate::util;

#[derive(PartialEq)]
pub enum ArchiveType {
    Aia,
    Aix,
}

/// Extracts the given file and dumps the contents to a temporary directory.
/// Returns a vec of [PathBuf]s that corresponds to the base directories of
/// extensions.
pub fn extract_file(path: &Path, archive_type: &ArchiveType) -> Vec<PathBuf> {
    let file_name = path.file_name().unwrap();
    let output_dir = util::data_dir().join("temp").join(file_name);

    let file = open_file(path, false);

    let mut archive = match ZipArchive::new(file) {
        Ok(zip) => zip,
        Err(err) => {
            eprintln!(
                "       {} Unable to open {}. Reason: {}",
                Red.paint("error"),
                path.to_str().unwrap(),
                err.to_string()
            );
            process::exit(1);
        }
    };

    // Try to extract the archive.
    if let Err(err) = archive.extract(&output_dir) {
        eprintln!(
            "       {} Unable to extract {}. Reason: {}",
            Red.paint("error"),
            path.to_str().unwrap(),
            err.to_string()
        );
        process::exit(1);
    }

    match archive_type {
        ArchiveType::Aia => {
            let external_comps = output_dir.join("assets").join("external_comps");

            let base_dirs = match fs::read_dir(external_comps) {
                Ok(res) => res,
                Err(err) => {
                    eprintln!(
                        "       {} Unable to extract {}. Reason: {}",
                        Red.paint("error"),
                        path.to_str().unwrap(),
                        err.to_string()
                    );
                    process::exit(1);
                }
            };

            let mut res: Vec<PathBuf> = Vec::new();
            for el in base_dirs {
                let el = el.unwrap();
                res.push(el.path());
            }

            res
        }
        ArchiveType::Aix => [output_dir.join(ext_base_dir(archive))].into(),
    }
}

/// Packs the given [dir_path] as a ZIP archive with a custom [prefix].
pub fn pack_dir(dir_path: &Path, output_dir: &Path, prefix: &str) {
    let archive_basename = dir_path.file_stem().unwrap().to_str().unwrap();
    let out_path = output_dir.join(format!("{}{}", archive_basename, prefix));

    // If the ...x.aix/a already exists in the output dir, delete it.
    if out_path.exists() {
        if let Err(err) = fs::remove_file(out_path.as_path()) {
            eprintln!(
                "       {} Unable to create {}. Reason: {}",
                Red.paint("error"),
                out_path.as_path().to_str().unwrap(),
                err.to_string()
            );
            process::exit(1);
        }
    }

    let file = open_file(out_path.as_path(), true);

    // Initialize the zip writer
    let mut zip_writer = ZipWriter::new(file);

    // Create the AIX
    let result = if prefix == "_x.aia" {
        archive_from_path(
            &mut zip_writer,
            dir_path,
            dir_path.parent().unwrap(),
            &format!("{}.aia", archive_basename),
        )
    } else {
        archive_from_path(&mut zip_writer, dir_path, dir_path.parent().unwrap(), "")
    };

    if let Err(err) = result {
        eprintln!(
            "       {} Unable to create {}. Reason: {}",
            Red.paint("error"),
            out_path.as_path().to_str().unwrap(),
            err.to_string()
        );
        process::exit(1);
    };

    // Delete the base directory once the extension is packed
    fs::remove_dir_all(dir_path).unwrap();
}

/// Recursively generates an archive from the given [path].
fn archive_from_path(
    zip_writer: &mut ZipWriter<File>,
    path: &Path,
    output_dir: &Path,
    exclude_dir_name: &str,
) -> Result<(), Box<dyn Error>> {
    if path.is_dir() {
        // Relative path of the directory from the output directory.
        let path_rel = path.strip_prefix(output_dir)?.to_str().unwrap();

        // `\` must be replaced with `/` otherwise the builder won't be able to
        // locate files.
        let path_rel = path_rel.replace("\\", "/");

        if !path.file_name().unwrap().eq(exclude_dir_name) {
            zip_writer.add_directory(path_rel, FileOptions::default())?;
        }

        // List all the entities in this directory
        let entities = fs::read_dir(path)?;

        // Then recursively add all the entities to the archive
        for entity in entities {
            let entity_path = entity?.path();

            archive_from_path(
                zip_writer,
                entity_path.as_path(),
                output_dir,
                exclude_dir_name,
            )?;
        }
    } else {
        let file = open_file(path, false);
        let contents = contents_as_bytes(file)?;

        // Relative path of the file from the output directory.
        let mut path_rel = path
            .strip_prefix(output_dir)?
            .to_str()
            .unwrap()
            .replace("\\", "/");

        if !exclude_dir_name.is_empty() {
            path_rel = path_rel.replace(&format!("{}/", exclude_dir_name), "");
        }

        // Add this file in the archive
        zip_writer.start_file(path_rel, FileOptions::default())?;

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
                                "       {} Unable to create file {}. Reason: {}",
                                Red.paint("error"),
                                path.to_str().unwrap(),
                                err.to_string()
                            );
                            process::exit(1);
                        }
                    }
                } else {
                    eprintln!(
                        "       {} File {} not found",
                        Red.paint("error"),
                        path.to_str().unwrap()
                    );
                    process::exit(2);
                }
            }
            _ => {
                eprintln!(
                    "       {} Unable to open file {}. Reason: {}",
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
fn ext_base_dir(archive: ZipArchive<File>) -> PathBuf {
    let ext_props_path = archive
        .file_names()
        .into_iter()
        .find(|x| x.ends_with("classes.jar"))
        .unwrap();

    Path::new(ext_props_path).parent().unwrap().to_owned()
}
