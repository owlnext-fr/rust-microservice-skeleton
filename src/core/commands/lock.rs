use std::path::{Path, PathBuf};

use anyhow::{bail, Result};
use rocket::tokio::fs::{remove_file, File};

use crate::core::dirs;

/// A trait defining behaviour for a one-access lock.
/// A one-access lock is a lock that provides a lock for contextualized ressources given a key. Only one access at a time can be given to the ressource given the same key.
#[async_trait]
pub trait OneAccessLock {
    /// tries to acquire the lock on a given key.
    async fn try_acquire(key: &str) -> Result<()>
    where
        Self: Sized;

    /// tries to release a lock on a given key.
    async fn try_release(key: &str) -> Result<()>
    where
        Self: Sized;
}

/// representation of a system file lock.
#[derive(Debug, Copy, Clone)]
pub struct FileLock {}

#[async_trait]
impl OneAccessLock for FileLock {
    async fn try_acquire(key: &str) -> Result<()> {
        let lock_file_path = get_lock_file_path(key);

        if Path::new(&lock_file_path).exists() {
            bail!("Lock already aquired !");
        }

        if let Err(_created) = create_lock_file(&lock_file_path).await {
            bail!("Cannot write lock file");
        }

        Ok(())
    }

    async fn try_release(key: &str) -> Result<()> {
        let lock_file_path = get_lock_file_path(key);

        if !Path::new(&lock_file_path).exists() {
            bail!("Lock file does not exists !");
        }

        if let Err(_deleted) = remove_lock_file(&lock_file_path).await {
            bail!("Lock file cannot be deleted !");
        }

        Ok(())
    }
}

/// util function to create a system-file lock file.
async fn create_lock_file(path: &PathBuf) -> Result<()> {
    File::create(path).await?;
    Ok(())
}

/// util function to remove a system-file lock file.
async fn remove_lock_file(path: &PathBuf) -> Result<()> {
    remove_file(path).await?;
    Ok(())
}

/// util function to generate a lock file path.
fn get_lock_file_path(key: &str) -> PathBuf {
    let formatted_key = generate_lock_key(key);

    let mut lock_possible_path = get_lock_dir();
    lock_possible_path.push(format!("{formatted_key}.lock"));

    lock_possible_path
}

/// util function to get the base path of the lock files.
fn get_lock_dir() -> PathBuf {
    dirs::lock_dir()
}

/// util function to generate a simple lock key from a given context.
fn generate_lock_key(context: &str) -> String {
    format!("{:x}", md5::compute(context))
}

#[cfg(test)]
mod tests {
    use rocket::tokio;
    use tokio_test::{assert_err, assert_ok};

    use super::*;

    fn create_key<'a>() -> &'a str {
        "abcd"
    }

    #[tokio::test]
    async fn test_suite() {
        let key = create_key();
        let result = FileLock::try_acquire(key).await;
        assert_ok!(result);

        let result = FileLock::try_acquire(key).await;
        assert_err!(result);

        let result = FileLock::try_release(key).await;
        assert_ok!(result);
    }
}
