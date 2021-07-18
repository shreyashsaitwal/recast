use std::path::{Path, PathBuf};

use crate::util;

const NO_REF_WARN: &str = "WARNING: [Main] No references were rewritten.";

/// Jetifies the AndroidRuntime.jar, i.e., migrates any references to the support
/// library packages to their AndroidX equivalent.
pub fn jetify(ext_base_dir: &Path) -> bool {
    let art_jar = ext_base_dir.join("files").join("AndroidRuntime.jar");

    let args = [
        "-i",
        art_jar.to_str().unwrap(),
        "-o",
        art_jar.to_str().unwrap(),
    ];

    // Spawn the jetifier standalone and collect it's output
    let output = util::spawn_process(jetifer_path().as_path(), &args);

    // Convert output to string
    let output_as_str = String::from_utf8(output.stdout).unwrap();

    // If output contains `NO_REF_WARN`, it means that the extension classes has
    // no references to the support library. No need to further process the
    //  extension.
    !output_as_str.contains(NO_REF_WARN)
}

/// Returns the path to the platform specific jetifier standalone script.
fn jetifer_path() -> PathBuf {
    let bin_dir = util::data_dir()
        .join("tools")
        .join("jetifier-standalone")
        .join("bin");

    if std::env::consts::OS == "windows" {
        bin_dir.join("jetifier-standalone.bat")
    } else {
        bin_dir.join("jetifier-standalone")
    }
}
