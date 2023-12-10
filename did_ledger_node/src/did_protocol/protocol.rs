use anyhow::{bail, Result};
use log::info;
use std::hash::Hash;
use std::{collections::hash_map::DefaultHasher, hash::Hasher};

use crate::{
    consensus::{types::ProtocolType, Consensus},
    database::{
        create_wallet, establish_connection, issue_cert, read_doc, register_cert, DidIssuedCert,
        DidRegisteredCert, DidWallet,
    },
};

use super::types::{
    CreateWalletRequest, IssueCertRequest, ReadDocRequest,
    ReadDocResponse, RegisterCertRequest, WriteDocRequest, WriteType,
};

pub async fn did_run(local_addr: String) -> Result<()> {
    let consensus = Consensus::new(local_addr);
    consensus.run(did_protocol).await?;
    Ok(())
}

fn hash_data(data: &Vec<u8>) -> i64 {
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    hasher.finish() as i64
}

pub async fn did_protocol(proto_type: ProtocolType, data: Vec<u8>) -> Result<Vec<u8>> {
    match proto_type {
        // corresponds to read_doc_request in backend
        ProtocolType::Read => {
            let readdoc_req: ReadDocRequest = bincode::deserialize(&data)?;
            let mut db_conn = establish_connection().await;
            println!("requested did #{:#08X}",readdoc_req.request_did);
            let did_doc = read_doc(&mut db_conn, readdoc_req.request_did).await?;
            println!("Did document #{:#08X} is read", readdoc_req.request_did);
            let response = ReadDocResponse {
                did_document: did_doc,
            };
            return Ok(bincode::serialize(&response).unwrap());
        }
        ProtocolType::Write => {
            let writedoc_req: WriteDocRequest = bincode::deserialize(&data)?;
            match writedoc_req.write_type {
                // corresponds to create_wallet in backend
                WriteType::CreateWallet => {
                    println!("CreateWallet called");
                    let create_wallet_req: CreateWalletRequest =
                        bincode::deserialize(&writedoc_req.value)?;

                    println!("CreateWallet did : {}", create_wallet_req.did);

                    let mut db_conn = establish_connection().await;
                    let new_wallet = DidWallet {
                        did: create_wallet_req.did,
                        public_key: create_wallet_req.public_key.value.to_vec(),
                    };

                    create_wallet(&mut db_conn, new_wallet).await?;
                    info!("New wallet created #{:#08x}", create_wallet_req.did);

                    return Ok(vec![]);
                }
                // corresponds to issue_cert in backend
                WriteType::IssueCert => {
                    let issue_cert_req: IssueCertRequest =
                        bincode::deserialize(&writedoc_req.value)?;

                    if !issue_cert_req.verify().await? {
                        bail!("The issueing request is not valid")
                    }

                    let did_issued_cert = DidIssuedCert {
                        did: issue_cert_req.did,
                        issuer_did: issue_cert_req.issuer_did,
                        json_type: issue_cert_req.json_type,
                        signature: issue_cert_req.signature,
                    };

                    let mut db_conn = establish_connection().await;
                    issue_cert(&mut db_conn, did_issued_cert).await?;
                    info!("New certificate has been issued #{:#08x}", issue_cert_req.did);

                    return Ok(vec![]);
                }
                // corresponds to register_cert in backend
                WriteType::RegisterCert => {
                    let register_cert_req: RegisterCertRequest =
                        bincode::deserialize(&writedoc_req.value)?;

                    if !register_cert_req.verify().await? {
                        bail!("The registering cert request is not valid")
                    }

                    let did_registered_cert = DidRegisteredCert {
                        did: register_cert_req.did,
                        user_did: register_cert_req.user_did,
                        cert_did: register_cert_req.cert_did,
                        issuer_signature: register_cert_req.issuer_signature,
                        signature: register_cert_req.user_signature,
                    };

                    let mut db_conn = establish_connection().await;
                    register_cert(&mut db_conn, did_registered_cert).await?;
                    info!("New certificate is registered #{:#08x}", register_cert_req.did);
                    return Ok(vec![]);
                }
            }
        }
        _ => bail!("Invalid did message"),
    }
}
