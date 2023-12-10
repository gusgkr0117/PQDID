//! Functions to read the environment variables
use anyhow::Result;
use hex::FromHex;
use std::env;

use crate::pqc_sign::types::PublicKey;

pub fn get_peers() -> Result<Vec<String>> {
    let mut peer_list: Vec<String> = Vec::new();
    for i in 1..5 {
        let env_str = format!("DID_REMOTE_{}", i);
        let value = env::var(env_str)?;
        peer_list.push(value);
    }
    Ok(peer_list)
}

pub fn get_peer_keys() -> Result<Vec<PublicKey>> {
    let mut peer_key_list: Vec<PublicKey> = Vec::new();
    for i in 1..5 {
        let env_str = format!("DID_PUBKEY_{}", i);
        let value = <PublicKey>::from_hex(env::var(env_str)?)?;
        peer_key_list.push(value);
    }
    Ok(peer_key_list)
}
