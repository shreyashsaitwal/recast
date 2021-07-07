use std::env::{self, consts};
use std::path::PathBuf;

/// Returns the Rush's data directory.
pub fn get_rush_data_dir() -> Option<PathBuf> {
    let os = consts::OS;

    if os == "windows" {
        let user = env::var("UserProfile").unwrap();
        let path = PathBuf::from(user)
            .join("AppData")
            .join("Roaming")
            .join("rush");
        Some(path)
    } else if os == "macos" {
        let home = env::var("HOME").unwrap();
        let path = PathBuf::from(home)
            .join("Library")
            .join("Application Support");
        Some(path)
    } else if os == "linux" {
        let home = env::var("HOME").unwrap();
        let path = PathBuf::from(home);
        Some(path)
    } else {
        None
    }
}
