use std::path::{Path, PathBuf};
use std::process;

use ansi_term::Color::{Blue, Cyan, Green, Red, Yellow};
use structopt::clap::AppSettings::{ColorAlways, ColoredHelp};
use structopt::StructOpt;

use archive::ArchiveType;

mod archive;
mod dexer;
mod jetifier;
mod util;

#[derive(StructOpt, Debug)]
#[structopt(name = "recast", setting(ColoredHelp), setting(ColorAlways))]
/// Recast makes your old extensions compatible with the latest versions of
/// Kodular and MIT AI2 by migrating them to new AndroidX libraries.
struct Options {
    /// Path to an individual AIA or AIX file or a directory containing many of
    /// them.
    #[structopt(parse(from_os_str), long, short)]
    input: PathBuf,

    /// Path to a directory where the new recasted AIA or AIX file(s) should be
    /// stored. Defaults to the current working directory.
    #[structopt(parse(from_os_str), long, short, default_value = ".")]
    output: PathBuf,
}

fn recast(input: &Path, output_dir: &Path) {
    let file_name = input.file_name().unwrap().to_str().unwrap();

    let archive_type = if file_name.ends_with(".aia") {
        println!("        {} Picked AIA [{}]", Blue.paint("info"), file_name);
        ArchiveType::Aia
    } else {
        ArchiveType::Aix
    };

    // A vec containing paths to all the extension base dirs in an AIA file. If
    // [input] is an AIX, it contains only one base dir.
    let base_dirs = archive::extract_file(input, &archive_type);
    let mut count = 0;

    for el in base_dirs {
        println!(
            "  {} {}",
            Cyan.paint("processing"),
            el.file_name().unwrap().to_str().unwrap()
        );

        let needs_jetification = jetifier::jetify(el.as_path());

        if needs_jetification {
            dexer::dex(el.as_path());

            if archive_type == ArchiveType::Aix {
                archive::pack_dir(el.as_path(), output_dir, ".x.aix");
                println!(
                    "    {} Generated {}.x.aix",
                    Green.paint("complete"),
                    output_dir.join(el.file_name().unwrap()).to_str().unwrap()
                );
            } else {
                count += 1;
            }
        } else if archive_type == ArchiveType::Aix {
            println!(
                "        {} No references to support libraries found",
                Blue.paint("info")
            );
            println!(
                "     {} This extension is already compatible with Kodular; no need to recast it",
                Yellow.paint("skipped")
            );
        }
    }

    // Finally, if the input was an AIA pack all of it's contents if required.
    if archive_type == ArchiveType::Aia {
        // If count is 0 then it means none of the extensions were recasted.
        if count > 0 {
            let file_basename = input.file_stem().unwrap();
            let dir_path = util::data_dir().join("temp").join(file_name);

            archive::pack_dir(&dir_path, output_dir, "_x.aia");
            println!(
                "    {} Generated {}_x.aia",
                Green.paint("complete"),
                output_dir.join(file_basename).to_str().unwrap()
            );
        } else {
            println!(
                "     {} All the extensions in this AIA are already compatible with Kodular",
                Yellow.paint("skipped")
            );
        }
    }

    println!();
}

fn main() {
    // Enable ANSI support on Windows 10 ignoring any error.
    #[cfg(target_os = "windows")]
    ansi_term::enable_ansi_support().unwrap_or(());

    let opts: Options = Options::from_args();
    let output_dir = opts.output;
    let input = opts.input;

    // Create the output dir if it doesn't already exists.
    if !output_dir.exists() {
        if let Err(err) = std::fs::create_dir_all(&output_dir) {
            eprintln!(
                "       {} Unable to create output directory {}. Reason: {}",
                Red.paint("error"),
                output_dir.to_str().unwrap(),
                err.to_string()
            );
            process::exit(1);
        }
    }

    // If input is a dir, collect every file that ends with either `.aia` and
    // `.aix` and recast it one-by-one, else proceed to recast the input only.
    if input.is_dir() {
        let entities = input.read_dir().unwrap();

        let archives = entities.filter(|x| {
            let name = x.as_ref().unwrap().file_name();
            let name = name.to_str().unwrap();
            name.ends_with(".aix") || name.ends_with(".aia")
        });

        for el in archives {
            let path = el.unwrap().path();
            recast(path.as_path(), output_dir.as_path());
        }
    } else {
        recast(input.as_path(), output_dir.as_path());
    }
}
