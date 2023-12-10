use crate::{
    database::{
        self, create_precert, establish_connection, get_cert_info, get_user_did,
        models::PreCertificates,
    },
    did_protocol::{self, protocol::get_pubkey_from_did},
    error::Error,
    jwt_auth::JwtMiddleware,
    pqc_sign::{types::Signature, verify_sig},
    types::ApiResult,
};
use actix_web::{post, web, Responder};
use hex::FromHex;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct PrecertInfo {
    template: serde_json::Value,
    cert_name: String,
    signature: String,
}

#[derive(Serialize)]
struct PrecertResponse {
    cert_did: String,
}

#[post("/issue_cert")]
pub async fn issue_cert(
    precert_info: web::Json<PrecertInfo>,
    jwt_middle: JwtMiddleware,
) -> Result<impl Responder, Error> {
    let user_id = jwt_middle.user_id;
    let signature: Signature = Signature::from_hex(precert_info.signature.clone())?;

    // Get corresponding user did
    let mut db_conn = establish_connection().await;
    let issuer_did = get_user_did(&mut db_conn, user_id.clone()).await?;

    // Issueing the certificate to did network
    let new_did =
        did_protocol::protocol::issue_cert(issuer_did, precert_info.template.clone(), signature)
            .await?;

    // Create a new precertificate
    let precert = PreCertificates {
        did: new_did,
        issuer_id: user_id,
        template: precert_info.template.to_string(),
        cert_name: precert_info.cert_name.clone(),
    };

    create_precert(&mut db_conn, precert).await?;

    let response = PrecertResponse {
        cert_did: format!("{:#08X}", new_did),
    };
    Ok(web::Json(ApiResult::ok(Some(response))))
}

#[derive(Deserialize)]
struct ApproveCertInfo {
    cert_id: i32,
    signature: String,
}

#[post("/approve_cert")]
pub async fn approve_cert(
    approve_cert_info: web::Json<ApproveCertInfo>,
    jwt_middle: JwtMiddleware,
) -> Result<impl Responder, Error> {
    let user_id = jwt_middle.user_id;
    let signature: Signature = Signature::from_hex(approve_cert_info.signature.clone())?;

    // Get corresponding issuer public key from issuer did
    let mut db_conn = establish_connection().await;
    let issuer_did = get_user_did(&mut db_conn, user_id.clone()).await?;
    let issuer_pubkey = get_pubkey_from_did(issuer_did).await?;

    // Get certificate data
    let cert_info = match get_cert_info(&mut db_conn, approve_cert_info.cert_id).await? {
        Some(x) => x,
        None => {
            return Ok(web::Json(ApiResult::err(
                "There is no certificate information",
            )));
        }
    };

    // Verify the signature
    if !verify_sig(cert_info.into_bytes(), signature.clone(), issuer_pubkey)? {
        return Ok(web::Json(ApiResult::err("The signature is not valid")));
    }

    // Approve the certificate
    database::approve_cert(&mut db_conn, approve_cert_info.cert_id, signature).await?;

    Ok(web::Json(ApiResult::ok::<usize>(None)))
}
