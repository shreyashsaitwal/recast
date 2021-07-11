use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::{env, process};

use ansi_term::Color::Red;

/// Returns the Recast's data directory.
pub fn data_dir() -> PathBuf {
    Path::new(&env::var("HOME").unwrap()).join(".recast")
}

/// Spawns the [program] and passes [args] to it. Exits if the process doesn't completes successfully.
pub fn spawn_process<I, S>(program: &Path, args: I) -> Output
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let output = Command::new(program).args(args).output().unwrap();

    if !output.status.success() {
        if !output.stderr.is_empty() {
            eprintln!(
                "     {} {}",
                Red.paint("error"),
                String::from_utf8(output.stderr)
                    .unwrap()
                    .replace("\n", "\n        ")
            );
        }

        if !output.stdout.is_empty() {
            eprintln!(
                "     {} {}",
                Red.paint("error"),
                String::from_utf8(output.stdout)
                    .unwrap()
                    .replace("\n", "\n        ")
            );
        }

        process::exit(1);
    }

    output
}
