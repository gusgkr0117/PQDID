use crate::{
    database::{
        self, create_cert, establish_connection, get_cert_from_id, get_user_did,
        models::Certificates,
    },
    did_protocol::{self, types::RegisterCertRequest},
    error::Error,
    jwt_auth::JwtMiddleware,
    pqc_sign::types::Signature,
    types::ApiResult,
};
use actix_web::{post, web, Responder};
use hex::FromHex;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RequestCertInfo {
    user_info: serde_json::Value,
    precert_did: i64,
}

#[post("/request_cert")]
pub async fn request_cert(
    req_cert_info: web::Json<RequestCertInfo>,
    jwt_middle: JwtMiddleware,
) -> Result<impl Responder, Error> {
    let user_id = jwt_middle.user_id;
    // 01. Verify the template of the requested certificate
    // TODO

    // 02. Create a new certificate in Certificate table
    let mut db_conn = establish_connection().await;
    let precert = database::get_precert_from_did(&mut db_conn, req_cert_info.precert_did).await?;
    let new_cert = Certificates {
        id: rand::random(),
        did: None,
        user_id,
        issuer_id: precert.issuer_id,
        cert_did: req_cert_info.precert_did,
        cert_info: Some(req_cert_info.user_info.to_string()),
        issuer_sig: None,
        stat: 0,
    };

    create_cert(&mut db_conn, new_cert).await?;

    Ok(web::Json(ApiResult::ok::<usize>(None)))
}

#[derive(Deserialize)]
pub struct RegisterCertInfo {
    cert_id: i32,
    signature: String,
}

#[post("/register_cert")]
pub async fn register_cert(
    reg_cert_info: web::Json<RegisterCertInfo>,
    jwt_middle: JwtMiddleware,
) -> Result<impl Responder, Error> {
    let mut db_conn = establish_connection().await;
    let cert = get_cert_from_id(&mut db_conn, reg_cert_info.cert_id).await?;
    let cert_info = cert.cert_info.ok_or("no certificate information")?;
    let issuer_signature = Signature::from_vec(cert.issuer_sig.ok_or("no issuer's signature")?)?;
    let user_signature = Signature::from_hex(reg_cert_info.signature.as_str())?;

    // Make a RegisterCertRequest
    let user_id = jwt_middle.user_id;
    let user_did = get_user_did(&mut db_conn, user_id).await?;
    let cert_did = cert.cert_did;
    let new_did: i64 = rand::random();

    let reg_cert_request = RegisterCertRequest {
        did: new_did,
        user_did,
        cert_did,
        cert_info: serde_json::from_str(cert_info.as_str())?,
        issuer_signature,
        user_signature,
    };

    // Update did database
    did_protocol::protocol::register_cert(reg_cert_request).await?;

    // Update local database
    database::register_cert(&mut db_conn, reg_cert_info.cert_id, new_did).await?;

    Ok(web::Json(ApiResult::ok::<usize>(None)))
}

#[derive(Deserialize)]
struct GetUserDidRequest {
    user_id: String,
}

#[derive(Serialize)]
struct GetUserDidResponse {
    pub did: String,
}

#[post("/get_did")]
pub async fn get_did(get_user_did_req: web::Json<GetUserDidRequest>) -> Result<impl Responder, Error> {
    let mut db_conn = establish_connection().await;
    let did = database::get_user_did(&mut db_conn, get_user_did_req.user_id.clone()).await?;
    let result = GetUserDidResponse {
        did: format!("{:08X}", did),
    };
    Ok(web::Json(ApiResult::ok(Some(result))))
}