//! Functions to read the environment variables
use anyhow::{Context, Result};
use hex::FromHex;
use std::env;

use crate::pqc_sign::types::{PublicKey, SecretKey};

pub fn get_local_addr() -> Result<String> {
    env::var("DID_LOCAL").context("No Local IP")
}

pub fn get_pubkey() -> Result<PublicKey> {
    let pubkey_str = env::var("DID_PUBKEY")?;
    Ok(PublicKey::from_hex(pubkey_str)?)
}

pub fn get_seckey() -> Result<SecretKey> {
    let seckey_str = env::var("DID_SECKEY")?;
    Ok(SecretKey::from_hex(seckey_str)?)
}

pub fn get_peers() -> Result<Vec<String>> {
    let mut peer_list: Vec<String> = Vec::new();
    for i in 1..4 {
        let env_str = format!("DID_REMOTE_{}", i);
        let value = env::var(env_str)?;
        peer_list.push(value);
    }
    Ok(peer_list)
}

pub fn get_peer_keys() -> Result<Vec<PublicKey>> {
    let mut peer_key_list: Vec<PublicKey> = Vec::new();
    for i in 1..4 {
        let env_str = format!("DID_PUBKEY_{}", i);
        let value = <PublicKey>::from_hex(env::var(env_str)?)?;
        peer_key_list.push(value);
    }
    Ok(peer_key_list)
}
