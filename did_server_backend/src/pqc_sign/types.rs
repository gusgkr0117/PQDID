//! Parameters and types for SQISign lvl1 implementation
use anyhow::{anyhow, bail};
use hex::FromHex;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

use crate::error::Error;
pub const CRYPTO_SECRETKEYBYTES: usize = 782;
pub const CRYPTO_PUBLICKEYBYTES: usize = 64;
pub const CRYPTO_BYTES: usize = 177;

#[derive(Serialize, Deserialize, Clone)]
pub struct PublicKey {
    #[serde(with = "BigArray")]
    pub value: [u8; CRYPTO_PUBLICKEYBYTES],
}

impl PublicKey {
    pub fn new() -> Self {
        PublicKey {
            value: [0u8; CRYPTO_PUBLICKEYBYTES],
        }
    }

    pub fn from_vec(value: Vec<u8>) -> Result<Self, Error> {
        if value.len() != CRYPTO_PUBLICKEYBYTES {
            return Err(anyhow!("The public key size is incorrect").into());
        }

        let mut result = PublicKey::new();
        for i in 0..CRYPTO_PUBLICKEYBYTES {
            result.value[i] = value[i];
        }
        Ok(result)
    }
}

impl FromHex for PublicKey {
    type Error = anyhow::Error;
    fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, Self::Error> {
        Ok(PublicKey {
            value: <[u8; CRYPTO_PUBLICKEYBYTES]>::from_hex(hex)?,
        })
    }
}

impl PartialEq for PublicKey {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..CRYPTO_PUBLICKEYBYTES {
            if self.value[i] != other.value[i] {
                return false;
            }
        }
        true
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SecretKey {
    #[serde(with = "BigArray")]
    pub value: [u8; CRYPTO_SECRETKEYBYTES],
}

impl SecretKey {
    pub fn new() -> Self {
        SecretKey {
            value: [0u8; CRYPTO_SECRETKEYBYTES],
        }
    }
}

impl FromHex for SecretKey {
    type Error = anyhow::Error;
    fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, Self::Error> {
        let tmp = <Vec<u8>>::from_hex(hex)?;
        if tmp.len() != CRYPTO_SECRETKEYBYTES {
            bail!("Incorrect format for SecretKey");
        }

        Ok(SecretKey {
            value: tmp
                .try_into()
                .expect("Unreachable : Incorrect format for SecretKey"),
        })
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Signature {
    #[serde(with = "BigArray")]
    pub value: [u8; CRYPTO_BYTES + 32],
}

impl Signature {
    pub fn new() -> Self {
        Signature {
            value: [0u8; CRYPTO_BYTES + 32],
        }
    }

    pub fn from_vec(value: Vec<u8>) -> Result<Self, Error> {
        if value.len() != CRYPTO_BYTES + 32 {
            return Err(anyhow!("The signature size is incorrect").into());
        }

        let mut result = Signature::new();
        for i in 0..CRYPTO_BYTES + 32 {
            result.value[i] = value[i];
        }
        Ok(result)
    }
}

impl FromHex for Signature {
    type Error = anyhow::Error;
    fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, Self::Error> {
        let tmp = <Vec<u8>>::from_hex(hex)?;
        if tmp.len() != CRYPTO_BYTES + 32 {
            bail!("Incorrect format for SecretKey");
        }

        Ok(Signature {
            value: tmp
                .try_into()
                .expect("Unreachable : Incorrect format for Signature"),
        })
    }
}
