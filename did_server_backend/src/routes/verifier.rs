use crate::types::ApiResult;
use crate::{did_protocol::protocol::get_cert_from_did, error::Error};
use actix_web::{post, web, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct VerifyCertInfo {
    cert_did: i64,
    cert_info: String,
}

#[post("/verify_cert")]
pub async fn verify_cert(
    view_cert_info: web::Json<VerifyCertInfo>,
) -> Result<impl Responder, Error> {
    let cert = get_cert_from_did(view_cert_info.cert_did).await?;
    Ok(web::Json(ApiResult::ok::<usize>(None)))
}
