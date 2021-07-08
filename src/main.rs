use std::path::PathBuf;

use structopt::clap::AppSettings::ColorAlways;
use structopt::StructOpt;

mod archive;
mod dexer;
mod jetifier;
mod util;

#[derive(StructOpt, Debug)]
#[structopt(name = "jetifier", setting(ColorAlways))]
struct Options {
    /// Path to AIX file.
    #[structopt(parse(from_os_str), long, short)]
    input: PathBuf,

    /// Path to a directory where the jetified AIX should be stored.
    #[structopt(parse(from_os_str), long, short)]
    output: PathBuf,
}

fn main() {
    let opts: Options = Options::from_args();

    let base_dir = archive::extract_aix(opts.input.as_path(), opts.output.as_path());

    let needs_jetification = jetifier::jetify(&base_dir);

    if needs_jetification {
        dexer::dex(&base_dir);
        archive::pack_aix(&base_dir, opts.output.as_path());
    }
}
