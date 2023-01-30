use std::path::{Path, PathBuf};

use anyhow::{bail, Result};
use rocket::tokio::fs::{remove_file, File};

#[async_trait]
pub trait OneAccessLock {
    async fn try_aquire(key: &str) -> Result<()>
    where
        Self: Sized;

    async fn try_release(key: &str) -> Result<()>
    where
        Self: Sized;
}

#[derive(Debug, Copy, Clone)]
pub struct FileLock {}

#[async_trait]
impl OneAccessLock for FileLock {
    async fn try_aquire(key: &str) -> Result<()> {
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

async fn create_lock_file(path: &PathBuf) -> Result<()> {
    File::create(path).await?;
    Ok(())
}

async fn remove_lock_file(path: &PathBuf) -> Result<()> {
    remove_file(path).await?;
    Ok(())
}

fn get_lock_file_path(key: &str) -> PathBuf {
    let formatted_key = generate_lock_key(key);

    let mut lock_possible_path = get_lock_dir();
    lock_possible_path.push(format!("{formatted_key}.lock"));

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
        let result = FileLock::try_aquire(key).await;
        assert_ok!(result);

        let result = FileLock::try_aquire(key).await;
        assert_err!(result);

        let result = FileLock::try_release(key).await;
        assert_ok!(result);
    }
}
