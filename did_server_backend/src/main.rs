mod config;
mod consensus;
mod database;
mod did_protocol;
mod error;
mod jwt_auth;
mod peers;
mod pqc_sign;
mod routes;
mod types;

use std::env;

use actix_cors::Cors;
#[warn(missing_docs)]
use actix_web::{App, HttpServer};
use routes::{
    auth::{join, login, logout, keygen},
    issuer::{approve_cert, issue_cert},
    owner::{register_cert, request_cert, get_did},
    verifier::{verify_cert, did_status},
};
use tokio;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let local_ip = env::var("LOCAL_IP").unwrap();
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default().allow_any_origin().allowed_methods(vec!["GET", "POST"]).allowed_headers(vec!["Content-Type"]).max_age(3600))
            .service(keygen)
            .service(join)
            .service(login)
            .service(logout)
            .service(issue_cert)
            .service(approve_cert)
            .service(request_cert)
            .service(register_cert)
            .service(verify_cert)
            .service(get_did)
            .service(did_status)
    })
    .bind((local_ip, 8000))?
    .run()
    .await
}
