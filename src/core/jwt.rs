use std::{collections::HashSet, error::Error, fs, path::PathBuf};

use jwt_simple::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct APIClaim {
    pub user_id: i32,
    pub roles: Vec<String>,
    pub username: String,
}

pub fn encode(claim: APIClaim, jwt_ttl: i64, issuer: &str) -> Result<String, Box<dyn Error>> {
    let key_pair = RS512KeyPair::from_pem(&get_private_certificate_content()?)?;

    let claims = Claims::with_custom_claims(claim, Duration::from_secs(jwt_ttl.unsigned_abs()))
        .with_issuer(issuer)
        .with_subject("authorization");

    let token = key_pair.sign(claims)?;

    Ok(token)
}

pub fn decode(token: &str, issuer: &str) -> Result<JWTClaims<APIClaim>, Box<dyn Error>> {
    let public_key = RS512PublicKey::from_pem(&get_public_certificate_content()?)?;

    let options = VerificationOptions {
        accept_future: true,
        allowed_issuers: Some(HashSet::from_strings(&[issuer])),
        ..Default::default()
    };

    let claims = public_key.verify_token::<APIClaim>(token, Some(options))?;

    Ok(claims)
}

pub fn get_private_certificate_path() -> PathBuf {
    let mut path: PathBuf = get_certificate_dir();
    path.push("private.pem");

    path
}

pub fn get_public_certificate_path() -> PathBuf {
    let mut path: PathBuf = get_certificate_dir();
    path.push("public.pem");

    path
}

pub fn get_certificate_dir() -> PathBuf {
    let mut path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("storage");
    path.push("jwt");

    path
}

fn get_private_certificate_content() -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(get_private_certificate_path())?.parse()?;

    Ok(content)
}

fn get_public_certificate_content() -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(get_public_certificate_path())?.parse()?;

    Ok(content)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn prelude() {}

    fn create_claim() -> APIClaim {
        APIClaim {
            user_id: 1,
            roles: vec!["ROLE_USER".into()],
            username: "test".into(),
        }
    }

    #[test]
    fn create_jwt() {
        prelude();
        let jwt_token = encode(create_claim(), 3600, "owlnext").unwrap();
        println!("{}", jwt_token);
    }

    #[test]
    fn validate_jwt() {
        prelude();
        let jwt_token = encode(create_claim(), 3600, "owlnext").unwrap();
        let claims_response = decode(&jwt_token, "owlnext");
        assert!(claims_response.is_ok());
        println!("{:#?}", claims_response.unwrap());
    }
}
