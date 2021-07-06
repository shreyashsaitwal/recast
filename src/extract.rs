use std::fs::File;
use std::io::ErrorKind;
use std::path::Path;
use std::{fs, process};

use zip::ZipArchive;

/// Extracts the extension file and copies it's contents to [output_dir].
pub fn extract_aix(path: &Path, output_dir: &Path) {
    let output_dir = ensure_dir(output_dir);

    let aix = open_file(path);

    let mut archive = match ZipArchive::new(aix) {
        Ok(zip) => zip,
        Err(err) => {
            eprintln!(
                "Something went wrong while trying to open the extension file {}",
                path.display()
            );
            eprintln!("{:?}", err);
            process::exit(1)
        }
    };

    // Extract each file in the archive one by one
    for i in 0..archive.len() {
        let mut file = match archive.by_index(i) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Something went wrong:");
                eprintln!("{:?}", err);
                process::exit(1);
            }
        };

        // Ignore empty directories.
        if file.is_file() {
            let output = match file.enclosed_name() {
                Some(path) => {
                    // Sanitize the in-archive path of the file by replacing `/` with OS specific
                    // path separator.
                    let sanitized = match path.to_str() {
                        None => continue,
                        Some(p) => p.replace("/", &std::path::MAIN_SEPARATOR.to_string()),
                    };

                    // Join the path with the output directory and return
                    output_dir.join(sanitized)
                }
                None => continue,
            };

            // Create the parent directories before creating the final output file
            let parent = match output.parent() {
                None => continue,
                Some(path) => path,
            };
            ensure_dir(parent);

            let mut output = match File::create(output) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Something went wrong:");
                    eprintln!("{:?}", err);
                    process::exit(1);
                }
            };

            match std::io::copy(&mut file, &mut output) {
                Ok(_) => {}
                Err(err) => {
                    eprintln!(
                        "Something went wrong while trying to extract the contents of {}",
                        path.display()
                    );
                    eprintln!("{}", err);
                    process::exit(1);
                }
            }
        }
    }
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
fn open_file(path: &Path) -> File {
    match File::open(path) {
        Ok(file) => file,
        Err(err) => {
            match err.kind() {
                ErrorKind::NotFound => eprintln!("Extension file {} not found", path.display()),
                ErrorKind::PermissionDenied => eprintln!(
                    "Permission denied; unable to access extension file {}",
                    path.display()
                ),
                _ => {
                    eprintln!(
                        "Something went wrong while trying to open the extension file {}",
                        path.display()
                    );
                    eprintln!("{}", err);
                }
            };
            process::exit(2);
        }
    }
}
