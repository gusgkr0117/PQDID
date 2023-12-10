use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
    database::{establish_connection, models::DidDocuments, read_doc, read_pubkey},
    pqc_sign::{
        types::{PublicKey, Signature},
        verify_sig,
    },
};

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
    pub did: i64,
    pub public_key: PublicKey,
}

#[derive(Serialize, Deserialize)]
pub struct IssueCertRequest {
    pub did: i64,
    pub issuer_did: i64,
    /// This string must be a json format
    pub json_type: serde_json::Value,
    pub signature: Signature,
}

impl IssueCertRequest {
    pub async fn verify(&self) -> Result<bool> {
        let mut db_conn = establish_connection().await;
        let public_key = read_pubkey(&mut db_conn, self.issuer_did).await?;
        let raw_data = bincode::serialize(&self.json_type)?;
        verify_sig(raw_data, self.signature.clone(), public_key)
    }
}

#[derive(Serialize, Deserialize)]
pub struct RegisterCertRequest {
    pub did: i64,
    pub user_did: i64,
    pub cert_did: i64,
    /// This is private data
    /// * This is used for verification only
    /// * The data will not be stored in the database
    pub cert_info: serde_json::Value,
    pub issuer_signature: Signature,
    pub user_signature: Signature,
}

impl RegisterCertRequest {
    pub async fn verify(&self) -> Result<bool> {
        let mut db_conn = establish_connection().await;
        let issued_doc = read_doc(&mut db_conn, self.cert_did).await?;
        let issuer_pubkey = read_pubkey(&mut db_conn, issued_doc.user_did.unwrap()).await?;
        let user_pubkey = read_pubkey(&mut db_conn, self.user_did).await?;
        let raw_data = bincode::serialize(&self.cert_info)?;
        // TODO: verifying the user's signature and check the cert_info format
        verify_sig(raw_data, self.issuer_signature.clone(), issuer_pubkey)
    }
}