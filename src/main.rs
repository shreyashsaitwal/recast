use std::path::{Path, PathBuf};
use std::process;

use ansi_term::Color::{Blue, Cyan, Green, Red, Yellow};
use archive::ArchiveType;
use structopt::clap::AppSettings::{ColorAlways, ColoredHelp};
use structopt::StructOpt;

mod archive;
mod dexer;
mod jetifier;
mod util;

#[derive(StructOpt, Debug)]
#[structopt(name = "recast", setting(ColoredHelp), setting(ColorAlways))]
/// Recast makes your old extensions compatible with the latest versions of
/// Kodular and MIT AI2 by migrating them to new AndroidX libraries.
struct Options {
    /// Path to an extension file (.aix) or a directory containing multiple
    /// extension files.
    #[structopt(parse(from_os_str), long, short)]
    input: PathBuf,

    /// Path to a directory where the new recasted extension file(s) should be
    /// stored.
    #[structopt(parse(from_os_str), long, short, default_value = ".")]
    output: PathBuf,
}

fn recast(file_path: &Path, output_dir: &Path) {
    let file_name = file_path.file_name().unwrap().to_str().unwrap();

    println!("{} '{}'", Cyan.paint("  processing"), file_name);

    let archive_type = if file_name.ends_with(".aia") {
        ArchiveType::Aia
    } else {
        ArchiveType::Aix
    };

    let base_dirs = archive::extract_file(file_path, &archive_type);

    for el in base_dirs {
        let needs_jetification = jetifier::jetify(el.as_path());

        if needs_jetification {
            dexer::dex(el.as_path());

            if archive_type == ArchiveType::Aix {
                archive::pack_dir(el.as_path(), output_dir, ".x.aix");
                println!(
                    "    {} Generated '{}.x.aix'",
                    Green.paint("complete"),
                    output_dir.join(el.file_name().unwrap()).to_str().unwrap()
                )
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

    if archive_type == ArchiveType::Aia {
        let file_basename = file_path.file_stem().unwrap();
        let dir_path = util::data_dir().join("temp").join(file_name);

        archive::pack_dir(&dir_path, output_dir, "_x.aia");
        println!(
            "    {} Generated '{}_x.aia'",
            Green.paint("complete"),
            output_dir.join(file_basename).to_str().unwrap()
        )
    }
    println!("");
}

fn main() {
    #[cfg(target_os = "windows")]
    ansi_term::enable_ansi_support().unwrap_or(());

    let opts: Options = Options::from_args();
    let output_dir = opts.output;
    let input = opts.input;

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
