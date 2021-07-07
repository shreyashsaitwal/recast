use std::path::{Path, PathBuf};
use std::process;
use std::process::Command;

use crate::util;

/// Replaces the old `classes.jar` with a DEXed version of the jetified `AndroidRuntime.jar`.
pub fn dex(base_dir: &Path) {
    let classes_jar = base_dir.join("classes.jar");
    remove_old_classes_jar(&*classes_jar);

    let art_jar = base_dir.join("files").join("AndroidRuntime.jar");
    let d8_path = d8_path();

    let args = [
        "-cp",
        d8_path.to_str().unwrap(),
        "com.android.tools.r8.D8",
        "--release",
        "--output",
        classes_jar.to_str().unwrap(),
        art_jar.to_str().unwrap(),
    ];

    let output = Command::new("java").args(&args).output().unwrap();

    if !output.status.success() {
        eprintln!("Something quite unexpected happened while trying to dex the extension:");
        eprintln!("{}", output.status);
        process::exit(1);
    }
}

fn d8_path() -> PathBuf {
    util::get_rush_data_dir()
        .unwrap()
        .join("tools")
        .join("other")
        .join("d8.jar")
}

fn remove_old_classes_jar(classes_jar: &Path) {
    if let Err(err) = std::fs::remove_file(classes_jar) {
        eprintln!("Something went wrong while trying to remove `classes.jar`:");
        eprintln!("{}", err);
        process::exit(1);
    }
}
