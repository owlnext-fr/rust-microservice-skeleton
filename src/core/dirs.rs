use std::path::PathBuf;

use super::configuration::ConfigState;

use anyhow::{bail, Result};

pub fn upload_dir(config: &ConfigState) -> Result<PathBuf> {
    let upload_dir_path = config.get_string("UPLOAD_DIR");

    if upload_dir_path.is_err() {
        bail!("Cannot find APP_UPLOAD_DIR in env !");
    }

    Ok(PathBuf::from(upload_dir_path.unwrap()))
}
