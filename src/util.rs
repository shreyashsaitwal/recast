use std::env::{self, consts};
use std::error::Error;
use std::path::{Path, PathBuf};

/// Returns the Rush's data directory.
pub fn rush_data_dir() -> Option<PathBuf> {
    let os = consts::OS;

    if os == "windows" {
        let user = env::var("UserProfile").unwrap();
        let path = Path::new(&user)
            .join("AppData")
            .join("Roaming")
            .join("rush");

        Some(path)
    } else if os == "macos" {
        let home = env::var("HOME").unwrap();
        let path = Path::new(&home)
            .join("Library")
            .join("Application Support");
        
        Some(path)
    } else if os == "linux" {
        let home = env::var("HOME").unwrap();
        Some(PathBuf::from(home))
    } else {
        None
    }
}

/// Returns path to Recast's build dir
pub fn build_dir_path() -> Result<PathBuf, Box<dyn Error>> {
    // Get the base data directory using Rush's data directory
    let data_dir = rush_data_dir().unwrap();
    let data_dir = data_dir.parent().unwrap();

    let build_dir = data_dir.join("recast").join("temp");
    std::fs::create_dir_all(&build_dir)?;

    Ok(build_dir)
}
