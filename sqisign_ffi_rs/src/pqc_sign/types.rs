//! Parameters and types for SQISign lvl1 implementation
use anyhow::bail;
use hex::{FromHex, ToHex};
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;
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
}

impl FromHex for PublicKey {
    type Error = anyhow::Error;
    fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, Self::Error> {
        Ok(PublicKey {
            value: <[u8; CRYPTO_PUBLICKEYBYTES]>::from_hex(hex)?,
        })
    }
}

impl ToHex for PublicKey {
    fn encode_hex<T: std::iter::FromIterator<char>>(&self) -> T {
        self.value.encode_hex::<T>()
    }

    fn encode_hex_upper<T: std::iter::FromIterator<char>>(&self) -> T {
        self.value.encode_hex_upper::<T>()
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

impl ToHex for SecretKey {
    fn encode_hex<T: std::iter::FromIterator<char>>(&self) -> T {
        self.value.encode_hex::<T>()
    }

    fn encode_hex_upper<T: std::iter::FromIterator<char>>(&self) -> T {
        self.value.encode_hex_upper::<T>()
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
