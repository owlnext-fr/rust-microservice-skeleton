use std::path::PathBuf;

use super::configuration::ConfigState;

use anyhow::{bail, Result};

/// generic function to get the current upload directory path of the application.
pub fn upload_dir(config: &ConfigState) -> Result<PathBuf> {
    let upload_dir_path = config.get_string("UPLOAD_DIR");

    if upload_dir_path.is_err() {
        bail!("Cannot find APP_UPLOAD_DIR in env !");
    }

    Ok(PathBuf::from(upload_dir_path.unwrap()))
}

/// generic function to get the current system-file lock directory path of the application.
pub fn lock_dir() -> PathBuf {
    let mut lock_dir_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    lock_dir_path.push("storage");
    lock_dir_path.push("locks");

    lock_dir_path
}

/// generic function to get public/private certificate dir path.
pub fn certificate_dir() -> PathBuf {
    let mut path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("storage");
    path.push("jwt");

    path
}
