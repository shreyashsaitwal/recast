use std::path::{Path, PathBuf};
use std::process;
use std::process::Command;

use ansi_term::Color::Red;

use crate::util;

const NO_REF_WARN: &str = "WARNING: [Main] No references were rewritten.";

/// Jetifies the AndroidRuntime.jar, i.e., migrates any references to the support library packages
/// to their AndroidX equivalent.
pub fn jetify(base_dir: &Path) -> bool {
    let art_jar = base_dir.join("files").join("AndroidRuntime.jar");

    let args = [
        "-i",
        art_jar.to_str().unwrap(),
        "-o",
        art_jar.to_str().unwrap(),
    ];

    // Spawn the jetifier standalone and collect it's output
    let output = Command::new(jetifer_path()).args(&args).output().unwrap();

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

    // Convert output to string
    let output_as_str = String::from_utf8(output.stdout).unwrap();

    // If output contains `NO_REF_WARN`, it means that the extension classes has no references to
    // the support library. No need to further process th extension.
    !output_as_str.contains(NO_REF_WARN)
}

/// Returns the path to the platform specific jetifier standalone script.
fn jetifer_path() -> PathBuf {
    let bin_dir = util::rush_data_dir()
        .unwrap()
        .join("tools")
        .join("jetifier-standalone")
        .join("bin");

    if std::env::consts::OS == "windows" {
        bin_dir.join("jetifier-standalone.bat")
    } else {
        bin_dir.join("jetifier-standalone")
    }
}
