use crate::{did_protocol::protocol::get_pubkey_from_did, peers::check_peers};
use crate::types::ApiResult;
use crate::error::Error;
use actix_web::{post, get, web, Responder};
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct VerifyCertInfo {
    cert_did: String,
    cert_info: String,
}

#[derive(Serialize)]
pub struct VerifyResult {
    pub value: bool,
}

impl VerifyResult {
    pub fn new(value: bool) -> Self {
        VerifyResult {
            value
        }
    }
}

#[post("/verify_cert")]
pub async fn verify_cert(
    view_cert_info: web::Json<VerifyCertInfo>,
) -> Result<impl Responder, Error> {
    println!("cert_did : {}", view_cert_info.cert_did.as_str());
    let cert_did = u64::from_str_radix(view_cert_info.cert_did.as_str(), 16)? as i64;
    let res : VerifyResult = match get_pubkey_from_did(cert_did).await {
        Ok(_) => VerifyResult::new(true),
        Err(_) => VerifyResult::new(false),
    };
    Ok(web::Json(ApiResult::ok::<VerifyResult>(Some(res))))
}

#[derive(Serialize)]
struct DidStatusResult {
    value: Vec<bool>,
}

#[get("/did_status")]
pub async fn did_status() -> Result<impl Responder, Error> {
    let status_list = check_peers().await?;
    Ok(web::Json(ApiResult::ok(Some(status_list))))
}
