use std::path::{Path, PathBuf};
use std::process;

use ansi_term::Color::Red;

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

    util::spawn_process("java", &args);
}

/// Returns the path to `d8.jar` stored in Recast's data directory.
fn d8_path() -> PathBuf {
    util::data_dir()
        .join("tools")
        .join("d8.jar")
}

/// Deletes old, non-jetified, `classes.jar`.
fn remove_old_classes_jar(classes_jar: &Path) {
    if let Err(err) = std::fs::remove_file(classes_jar) {
        eprintln!(
            "     {} Unable to delete {}. Reason: {}",
            Red.paint("error"),
            classes_jar.to_str().unwrap(),
            err.to_string()
        );
        process::exit(1);
    }
}
