//! Functions to communicate with the did ledger nodes
use crate::{
    consensus::{protocol::send_consensus_packet, types::ProtocolType},
    pqc_sign::types::{PublicKey, Signature},
};
use anyhow::Result;

use super::types::{
    CreateWalletRequest, CreateWalletResponse, DidIssuedCert, DidRegisteredCert, DidWallet,
    IssueCertRequest, IssueCertResponse, ReadDocRequest, ReadDocResponse, RegisterCertRequest,
    RegisterCertResponse,
};

pub async fn get_pubkey_from_did(request_did: i64) -> Result<PublicKey> {
    let read_doc_response = read_doc_request(request_did).await?;
    let did_wallet = DidWallet::from(read_doc_response.did_document);
    Ok(did_wallet.public_key)
}

pub async fn get_precert_from_did(request_did: i64) -> Result<serde_json::Value> {
    let read_doc_response = read_doc_request(request_did).await?;
    let did_issued_cert = DidIssuedCert::try_from(read_doc_response.did_document)?;
    Ok(did_issued_cert.json_type)
}

pub async fn get_cert_from_did(request_did: i64) -> Result<DidRegisteredCert> {
    let read_doc_response = read_doc_request(request_did).await?;
    let did_registered_cert = DidRegisteredCert::try_from(read_doc_response.did_document)?;
    Ok(did_registered_cert)
}

pub async fn read_doc_request(request_did: i64) -> Result<ReadDocResponse> {
    let request = ReadDocRequest { request_did };
    let raw_response =
        send_consensus_packet(ProtocolType::Read, bincode::serialize(&request)?).await?;
    let result: ReadDocResponse = bincode::deserialize(&raw_response)?;
    Ok(result)
}

pub async fn create_wallet(public_key: PublicKey) -> Result<CreateWalletResponse> {
    let request = CreateWalletRequest { public_key };
    let raw_response =
        send_consensus_packet(ProtocolType::Write, bincode::serialize(&request)?).await?;
    let result: CreateWalletResponse = bincode::deserialize(&raw_response)?;
    Ok(result)
}

pub async fn issue_cert(
    issuer_did: i64,
    json_type: serde_json::Value,
    signature: Signature,
) -> Result<IssueCertResponse> {
    let request: IssueCertRequest = IssueCertRequest {
        issuer_did,
        json_type,
        signature,
    };
    let raw_response =
        send_consensus_packet(ProtocolType::Write, bincode::serialize(&request)?).await?;
    let result: IssueCertResponse = bincode::deserialize(&raw_response)?;
    Ok(result)
}

pub async fn register_cert(request: RegisterCertRequest) -> Result<RegisterCertResponse> {
    let raw_response =
        send_consensus_packet(ProtocolType::Write, bincode::serialize(&request)?).await?;
    let result: RegisterCertResponse = bincode::deserialize(&raw_response)?;
    Ok(result)
}
