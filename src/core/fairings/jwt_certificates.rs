use std::{fs, path::PathBuf, process::Command};

use rocket::{
    fairing::{Fairing, Info, Kind, Result},
    Build, Rocket,
};

use crate::{console_error, console_warning, core::jwt};

#[derive(Default)]
pub struct JWTCertificatesFairing {}

#[rocket::async_trait]
impl Fairing for JWTCertificatesFairing {
    fn info(&self) -> Info {
        Info {
            name: "Generates PEM certificates for JWT",
            kind: Kind::Ignite,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> Result {
        let jwt_sec_dir = jwt::get_certificate_dir();

        if !jwt_sec_dir.exists() {
            let cr_result = fs::create_dir(jwt_sec_dir.clone());

            if cr_result.is_err() {
                console_error!(
                    "Cannot create secured JWT token certificates directory, aborting launch..."
                );
                return Err(rocket);
            }
        }

        let private_key_path: PathBuf = jwt::get_private_certificate_path();
        let public_key_path: PathBuf = jwt::get_public_certificate_path();

        let mut needs_generation: bool = false;

        if !public_key_path.exists() && !private_key_path.exists() {
            needs_generation = true;
        }

        if public_key_path.exists() && !private_key_path.exists() {
            console_warning!("Something is wrong with certificates, they will be generated.");
            fs::remove_file(&public_key_path).unwrap();
            needs_generation = true;
        }

        if !public_key_path.exists() && private_key_path.exists() {
            console_warning!("Something is wrong with certificates, they will be generated.");
            fs::remove_file(&private_key_path).unwrap();
            needs_generation = true;
        }

        if needs_generation {
            // TODO refactor
            let mut child = Command::new("sh");

            let certs_creation = child
                .arg("-c")
                .arg(format!("cd {} && openssl genrsa -out private.pem 2048 && openssl rsa -in private.pem -outform PEM -pubout -out public.pem", jwt_sec_dir.as_os_str().to_string_lossy()))
                .output();

            if certs_creation.is_err() {
                console_error!("Cannot create secured JWT token certificates, aborting launch...");
                return Err(rocket);
            }
        }

        Ok(rocket)
    }
}
