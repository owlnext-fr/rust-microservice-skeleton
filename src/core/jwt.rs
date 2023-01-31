use std::{collections::HashSet, fs, path::PathBuf};

use jwt_simple::prelude::*;
use serde::{Deserialize, Serialize};

use anyhow::{Context, Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct APIClaim {
    pub user_id: i32,
    pub roles: Vec<String>,
    pub username: String,
}

pub fn encode(claim: APIClaim, jwt_ttl: i64, issuer: &str) -> Result<String> {
    let key_pair = RS512KeyPair::from_pem(&get_private_certificate_content()?)
        .with_context(|| "Cannot acquire private key.")?;

    let claims = Claims::with_custom_claims(claim, Duration::from_secs(jwt_ttl.unsigned_abs()))
        .with_issuer(issuer)
        .with_subject("authorization");

    let token = key_pair
        .sign(claims)
        .with_context(|| "Cannot sign JWT claims")?;

    Ok(token)
}

pub fn decode(token: &str, issuer: &str) -> Result<JWTClaims<APIClaim>> {
    let public_key = RS512PublicKey::from_pem(&get_public_certificate_content()?)
        .with_context(|| "Cannot acquire public key.")?;

    let options = VerificationOptions {
        accept_future: true,
        allowed_issuers: Some(HashSet::from_strings(&[issuer])),
        ..Default::default()
    };

    let claims = public_key
        .verify_token::<APIClaim>(token, Some(options))
        .with_context(|| "Cannot verify token with public key")?;

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

fn get_private_certificate_content() -> Result<String> {
    let content = fs::read_to_string(get_private_certificate_path())
        .with_context(|| "Cannot find private certificate file")?
        .parse()
        .with_context(|| "Cannot parse private certificate file")?;

    Ok(content)
}

fn get_public_certificate_content() -> Result<String> {
    let content = fs::read_to_string(get_public_certificate_path())
        .with_context(|| "Cannot find public certificate file")?
        .parse()
        .with_context(|| "Cannot parse public certificate file")?;

    Ok(content)
}
