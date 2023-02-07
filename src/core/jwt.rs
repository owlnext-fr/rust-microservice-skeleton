use std::{collections::HashSet, fs, path::PathBuf};

use jwt_simple::prelude::*;
use serde::{Deserialize, Serialize};

use anyhow::{Context, Result};

use super::dirs;

/// A struct representing JWT claims for this API.
#[derive(Serialize, Deserialize, Debug)]
pub struct APIClaim {
    /// the user ID of this JWT token.
    pub user_id: i32,
    /// the user's roles.
    pub roles: Vec<String>,
    /// the user's username (or login).
    pub username: String,
}

/// Utility function to encode a JWT token, given claims, TTL and issuer for the token to generate.
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

/// Utility function to decode a given JWT token as a `&str` for a given issuer.
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

/// Utility function to get API's private key file.
pub fn get_private_certificate_path() -> PathBuf {
    let mut path: PathBuf = get_certificate_dir();
    path.push("private.pem");

    path
}

/// Utility function to get API's public key file.
pub fn get_public_certificate_path() -> PathBuf {
    let mut path: PathBuf = get_certificate_dir();
    path.push("public.pem");

    path
}

/// Utility function to get API's certificate directory path.
pub fn get_certificate_dir() -> PathBuf {
    dirs::certificate_dir()
}

/// PRIVATE - Utility function to gather private certificate file content
fn get_private_certificate_content() -> Result<String> {
    let content = fs::read_to_string(get_private_certificate_path())
        .with_context(|| "Cannot find private certificate file")?
        .parse()
        .with_context(|| "Cannot parse private certificate file")?;

    Ok(content)
}

/// PRIVATE - Utility function to gather public certificate file content
fn get_public_certificate_content() -> Result<String> {
    let content = fs::read_to_string(get_public_certificate_path())
        .with_context(|| "Cannot find public certificate file")?
        .parse()
        .with_context(|| "Cannot parse public certificate file")?;

    Ok(content)
}
