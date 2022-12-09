use std::{
    fs,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};

use rocket::{
    fairing::{Fairing, Info, Kind, Result},
    Build, Rocket,
};

use crate::{console_error, console_warning};

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
        let mut jwt_sec_dir: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        jwt_sec_dir.push("storage");
        jwt_sec_dir.push("jwt");

        if !jwt_sec_dir.exists() {
            let cr_result = fs::create_dir(jwt_sec_dir.clone());

            if cr_result.is_err() {
                console_error!(
                    "Cannot create secured JWT token certificates directory, aborting launch..."
                );
                return Err(rocket);
            }
        }

        let mut private_key_path: PathBuf = jwt_sec_dir.clone();
        private_key_path.push("private.pem");

        let mut public_key_path: PathBuf = jwt_sec_dir.clone();
        public_key_path.push("public.pem");

        let mut needs_generation: bool = false;

        if !public_key_path.exists() && !private_key_path.exists() {
            needs_generation = true;
        }

        if public_key_path.exists() && !private_key_path.exists() {
            console_warning!("Something is wrong with certificates, they will be generated.");
            fs::remove_file(public_key_path.clone()).unwrap();
            needs_generation = true;
        }

        if !public_key_path.exists() && private_key_path.exists() {
            console_warning!("Something is wrong with certificates, they will be generated.");
            fs::remove_file(private_key_path.clone()).unwrap();
            needs_generation = true;
        }

        if needs_generation {
            // TODO refactor
            let mut child = Command::new("sh")
                .stdin(Stdio::piped())
                .stderr(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();

            let child_stdin = child.stdin.as_mut().unwrap();

            child_stdin
                .write_all(b"openssl genrsa -out private.pem 2048")
                .unwrap();
            child_stdin
                .write_all(b"openssl rsa -in private.pem -outform PEM -pubout -out public.pem")
                .unwrap();
        }

        Ok(rocket)
    }
}
