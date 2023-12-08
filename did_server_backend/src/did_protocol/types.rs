use std::time::SystemTime;

use hex::FromHex;
use serde::{Deserialize, Serialize};

use crate::{
    error::Error,
    pqc_sign::types::{PublicKey, Signature},
};

pub struct DidWallet {
    pub did: i64,
    pub public_key: PublicKey,
}

impl Into<DidDocuments> for DidWallet {
    fn into(self) -> DidDocuments {
        DidDocuments {
            did: self.did,
            user_did: None,
            doc_data: hex::encode(self.public_key.value),
            timestamp: SystemTime::now(),
            sig: None,
        }
    }
}

impl From<DidDocuments> for DidWallet {
    fn from(value: DidDocuments) -> Self {
        DidWallet {
            did: value.did,
            public_key: PublicKey::from_hex(value.doc_data).unwrap(),
        }
    }
}

pub struct DidIssuedCert {
    pub did: i64,
    pub issuer_did: i64,
    pub json_type: serde_json::Value,
    pub signature: Signature,
}

impl Into<DidDocuments> for DidIssuedCert {
    fn into(self) -> DidDocuments {
        DidDocuments {
            did: self.did,
            user_did: Some(self.issuer_did),
            doc_data: self.json_type.to_string(),
            timestamp: SystemTime::now(),
            sig: Some(self.signature.value.to_vec()),
        }
    }
}

impl TryFrom<DidDocuments> for DidIssuedCert {
    type Error = Error;
    fn try_from(value: DidDocuments) -> Result<Self, Self::Error> {
        Ok(DidIssuedCert {
            did: value.did,
            issuer_did: value.user_did.ok_or("no user did")?,
            json_type: serde_json::from_str(value.doc_data.as_str())?,
            signature: Signature::from_vec(value.sig.ok_or("no signature")?)?,
        })
    }
}

pub struct DidRegisteredCert {
    pub did: i64,
    pub user_did: i64,
    pub cert_did: i64,
    pub issuer_signature: Signature,
    pub signature: Signature,
}

impl Into<DidDocuments> for DidRegisteredCert {
    fn into(self) -> DidDocuments {
        DidDocuments {
            did: self.did,
            user_did: Some(self.user_did),
            doc_data: format!("{:#08X}", self.cert_did as u64)
                + "&"
                + hex::encode_upper(self.issuer_signature.value).as_str(),
            timestamp: SystemTime::now(),
            sig: Some(self.signature.value.to_vec()),
        }
    }
}

impl TryFrom<DidDocuments> for DidRegisteredCert {
    type Error = Error;
    fn try_from(value: DidDocuments) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.doc_data.split("&").collect();
        let cert_did: i64 = i64::from_str_radix(parts[0], 16)?;
        let issuer_signature = Signature::from_hex(parts[1])?;
        Ok(DidRegisteredCert {
            did: value.did,
            user_did: value.user_did.ok_or("no user did")?,
            cert_did,
            issuer_signature,
            signature: Signature::from_vec(value.sig.ok_or("no signature")?)?,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct DidDocuments {
    pub did: i64,
    pub user_did: Option<i64>,
    pub doc_data: String,
    pub timestamp: SystemTime,
    pub sig: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize)]
pub struct ReadDocRequest {
    pub request_did: i64,
}

#[derive(Serialize, Deserialize)]
pub struct ReadDocResponse {
    pub did_document: DidDocuments,
}

#[derive(Serialize, Deserialize)]
pub enum WriteType {
    CreateWallet,
    IssueCert,
    RegisterCert,
}

#[derive(Serialize, Deserialize)]
pub struct WriteDocRequest {
    pub write_type: WriteType,
    pub value: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateWalletRequest {
    pub public_key: PublicKey,
}

#[derive(Serialize, Deserialize)]
pub struct CreateWalletResponse {
    pub did: i64,
}

#[derive(Serialize, Deserialize)]
pub struct IssueCertRequest {
    pub issuer_did: i64,
    /// This string must be a json format
    pub json_type: serde_json::Value,
    pub signature: Signature,
}

#[derive(Serialize, Deserialize)]
pub struct IssueCertResponse {
    pub did: i64,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterCertRequest {
    pub user_did: i64,
    pub cert_did: i64,
    /// This is private data
    /// * This is used for verification only
    /// * The data will not be stored in the database
    pub cert_info: serde_json::Value,
    pub issuer_signature: Signature,
    pub user_signature: Signature,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterCertResponse {
    pub did: i64,
}
