use std::path::{Path, PathBuf};
use std::process::Command;

use crate::util;

const NO_REF_STR: &str = "WARNING: [Main] No references were rewritten.";

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

    let output = Command::new(jetifer_path()).args(&args).output().unwrap();
    let output_as_str = String::from_utf8(output.stdout).unwrap();

    if output_as_str.contains(NO_REF_STR) {
        println!("You don't need to jetify your extension.");
        false
    } else {
        true
    }
}

/// Returns the path to the jetifier standalone script.
fn jetifer_path() -> PathBuf {
    let bin_dir = util::get_rush_data_dir()
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
