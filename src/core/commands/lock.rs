use std::path::{Path, PathBuf};

use anyhow::{bail, Result};

pub trait OneAccessLock<'a> {
    fn try_aquire(key: &'a str) -> Result<Self>
    where
        Self: Sized;

    fn try_release(&self) -> Result<&Self>
    where
        Self: Sized;
}

pub struct FileLock<'a> {
    pub key: &'a str,
}

impl<'a> OneAccessLock<'a> for FileLock<'a> {
    fn try_aquire(key: &'a str) -> Result<FileLock<'a>> {
        let lock_file_path = get_lock_file_path(key);

        if Path::new(&lock_file_path).exists() {
            bail!("Lock already aquired !");
        }

        if let Err(_created) = create_lock_file(&lock_file_path) {
            bail!("Cannot write lock file");
        }

        Ok(Self { key })
    }

    fn try_release(&self) -> Result<&Self> {
        let lock_file_path = get_lock_file_path(self.key);

        if !Path::new(&lock_file_path).exists() {
            bail!("Lock file does not exists !");
        }

        if let Err(_deleted) = remove_lock_file(&lock_file_path) {
            bail!("Lock file cannot be deleted !");
        }

        Ok(self)
    }
}

fn create_lock_file(path: &PathBuf) -> Result<()> {
    Ok(())
}

fn remove_lock_file(path: &PathBuf) -> Result<()> {
    Ok(())
}

fn get_lock_file_path(key: &str) -> PathBuf {
    let formatted_key = generate_lock_key(key);

    let mut lock_possible_path = get_lock_dir();
    lock_possible_path.push(format!("{}.lock", formatted_key));

    lock_possible_path
}

fn get_lock_dir() -> PathBuf {
    let mut path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("storage");
    path.push("locks");

    path
}

fn generate_lock_key(content: &str) -> String {
    format!("{:x}", md5::compute(content))
}
