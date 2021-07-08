use std::fs::File;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::{fs, process};

use zip::ZipArchive;

/// Extracts the extension file and copies it's contents to [output_dir].
pub fn extract_aix(aix_path: &Path, output_dir: &Path) -> PathBuf {
    let output_dir = ensure_dir(output_dir);

    let aix = open_file(aix_path);

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

pub fn pack_aix() {
    // todo
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

/// Returns the base dir of the extension. This is same as the extension's package.
fn base_dir_from_aix(archive: ZipArchive<File>) -> PathBuf {
    let ext_props_path = archive
        .file_names()
        .into_iter()
        .find(|x| x.ends_with(".properties"))
        .unwrap();

    Path::new(ext_props_path).parent().unwrap().to_owned()
}
