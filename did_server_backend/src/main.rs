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

#[warn(missing_docs)]
use actix_web::{App, HttpServer};
use routes::{
    auth::{join, login, logout},
    issuer::{approve_cert, issue_cert},
    owner::{register_cert, request_cert},
    verifier::verify_cert,
};
use tokio;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(join)
            .service(login)
            .service(logout)
            .service(issue_cert)
            .service(approve_cert)
            .service(request_cert)
            .service(register_cert)
            .service(verify_cert)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
