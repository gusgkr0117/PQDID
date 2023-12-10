//! Functions to communicate with the did ledger nodes
use crate::{
    consensus::{protocol::{send_consensus_packet, send_consensus_packet_and_wait}, types::ProtocolType},
    pqc_sign::types::{PublicKey, Signature}, did_protocol::types::DidDocuments,
};
use anyhow::Result;

use super::types::{
    CreateWalletRequest, DidIssuedCert, DidRegisteredCert, DidWallet,
    IssueCertRequest, ReadDocRequest, ReadDocResponse, RegisterCertRequest, WriteDocRequest, WriteType,
};

pub async fn get_pubkey_from_did(request_did: i64) -> Result<PublicKey> {
    let read_doc_response = read_doc_request(request_did).await?;
    let did_wallet = DidWallet::from(read_doc_response);
    Ok(did_wallet.public_key)
}

pub async fn get_precert_from_did(request_did: i64) -> Result<serde_json::Value> {
    let read_doc_response = read_doc_request(request_did).await?;
    let did_issued_cert = DidIssuedCert::try_from(read_doc_response)?;
    Ok(did_issued_cert.json_type)
}

pub async fn get_cert_from_did(request_did: i64) -> Result<DidRegisteredCert> {
    let read_doc_response = read_doc_request(request_did).await?;
    let did_registered_cert = DidRegisteredCert::try_from(read_doc_response)?;
    Ok(did_registered_cert)
}

pub async fn check_doc_request(request_did: i64) -> Result<bool> {
    let request = ReadDocRequest { request_did };
    let raw_response = send_consensus_packet_and_wait(ProtocolType::Read, bincode::serialize(&request)?).await?;
    //send_consensus_packet(ProtocolType::Read, bincode::serialize(&request)?).await?;
    return Ok(true);
}

pub async fn read_doc_request(request_did: i64) -> Result<DidDocuments> {
    let request = ReadDocRequest { request_did };
    //send_consensus_packet(ProtocolType::Read, bincode::serialize(&request)?).await?;
    let raw_response =
        send_consensus_packet_and_wait(ProtocolType::Read, bincode::serialize(&request)?).await?;
    println!("read_doc_response : {:?}", raw_response);
    let result: DidDocuments = bincode::deserialize(&raw_response[8..])?;
    // let result : ReadDocResponse = bincode::deserialize(&vec![])?;
    Ok(result)
}

pub async fn create_wallet(public_key: PublicKey) -> Result<i64> {
    let new_did: i64 = rand::random();
    let request = CreateWalletRequest {did: new_did, public_key };
    let write_request = WriteDocRequest {write_type: WriteType::CreateWallet, value: bincode::serialize(&request)?};
    send_consensus_packet(ProtocolType::Write, bincode::serialize(&write_request)?).await?;
    Ok(new_did)
}

pub async fn issue_cert(
    issuer_did: i64,
    json_type: serde_json::Value,
    signature: Signature,
) -> Result<i64> {
    let new_did: i64 = rand::random();
    let request: IssueCertRequest = IssueCertRequest {
        did: new_did,
        issuer_did,
        json_type,
        signature,
    };
    let write_request = WriteDocRequest {write_type: WriteType::IssueCert, value: bincode::serialize(&request)?};
    send_consensus_packet(ProtocolType::Write, bincode::serialize(&write_request)?).await?;
    Ok(new_did)
}

pub async fn register_cert(request: RegisterCertRequest) -> Result<i64> {
    let new_did : i64 = rand::random();
    send_consensus_packet(ProtocolType::Write, bincode::serialize(&request)?).await?;
    Ok(new_did)
}
