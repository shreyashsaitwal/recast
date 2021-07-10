use std::path::{Path, PathBuf};

use structopt::clap::AppSettings::ColorAlways;
use structopt::clap::AppSettings::ColoredHelp;
use structopt::StructOpt;

mod archive;
mod dexer;
mod jetifier;
mod util;

#[derive(StructOpt, Debug)]
#[structopt(name = "recast", setting(ColoredHelp), setting(ColorAlways))]
/// Recast makes your old extensions compatible with the latest versions of Kodular and MIT AI2 by
/// migrating them to new AndroidX libraries.
struct Options {
    /// Path to an extension file (.aix) or a directory containing multiple extension files.
    #[structopt(parse(from_os_str), long, short)]
    input: PathBuf,

    /// Path to a directory where the new recasted extension file(s) should be stored.
    #[structopt(parse(from_os_str), long, short)]
    output: PathBuf,
}

fn main() {
    let opts: Options = Options::from_args();
    let output_dir = opts.output;
    let input = opts.input;

    if input.is_dir() {
        let entities = input.read_dir().unwrap();

        let extensions = entities.filter(|x| {
            x.as_ref()
                .unwrap()
                .file_name()
                .to_str()
                .unwrap()
                .ends_with(".aix")
        });

        for aix in extensions {
            let path = aix.unwrap().path();
            println!("{:?}", path);
            process(path.as_path(), output_dir.as_path());
        }
    } else {
        process(input.as_path(), output_dir.as_path());
    }
}

fn process(aix_path: &Path, output_dir: &Path) {
    let base_dir = archive::extract_aix(aix_path);
    let needs_jetification = jetifier::jetify(base_dir.as_path());

    if needs_jetification {
        dexer::dex(base_dir.as_path());
        archive::pack_aix(base_dir.as_path(), output_dir);
    }
}
