use std::env;

use chrono::{Duration, Utc};
use hex::FromHex;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha512};

use crate::{
    database::{create_user, establish_connection, models::Users, verify_user},
    did_protocol::protocol::create_wallet,
    error::Error,
    jwt_auth::{JwtClaim, JwtMiddleware},
    pqc_sign::{types::{PublicKey, SecretKey}, gen_keypair, self},
    types::ApiResult,
};
use actix_web::{cookie::time::Duration as ActixWebDuration, HttpResponse};
use actix_web::{cookie::Cookie, get, post, web, Responder, Result};

#[derive(Deserialize)]
struct SigningRequest {
    pub message: String,
    pub secret_key: String,
}

#[derive(Serialize)]
struct SigningResult {
    pub signature: String,
}

#[post("/signing")]
pub async fn signing(signing_req : web::Json<SigningRequest>) -> Result<impl Responder, Error> {
    let msg = signing_req.message.as_bytes();
    let secret_key = SecretKey::from_hex(signing_req.secret_key.clone())?;
    let signature = pqc_sign::signing(msg.to_vec(), secret_key)?;
    Ok(web::Json(ApiResult::ok(Some(hex::encode_upper(signature.value)))))
}

#[derive(Serialize)]
struct KeygenResult {
    pub public_key: String,
    pub secret_key: String,
}

#[get("/keygen")]
pub async fn keygen() -> Result<impl Responder, Error> {
    let (pk, sk) = gen_keypair()?;
    let result = KeygenResult{
        public_key: hex::encode_upper(pk.value),
        secret_key: hex::encode_upper(sk.value),
    };

    Ok(web::Json(ApiResult::ok(Some(result))))
}

#[derive(Deserialize)]
struct JoinInfo {
    pub user_id: String,
    pub passwd: String,
    pub username: String,
    pub public_key: String,
}

/// Create a new user and create a new did wallet
#[post("/join")]
async fn join(user_info: web::Json<JoinInfo>) -> Result<impl Responder, Error> {
    println!("Join api called");
    let mut db_conn = establish_connection().await;
    println!("db connected");
    let mut hasher = Sha512::new();
    hasher.update(user_info.passwd.as_bytes());
    let passwd_hash: String = hex::encode_upper(hasher.finalize());
    let pubkey = PublicKey::from_hex(user_info.public_key.as_str())?;
    println!("join publickey : {}", hex::encode_upper(pubkey.value));

    // TODO: check if the user_id is actually not duplicated

    // Create a new wallet
    let new_did = create_wallet(pubkey).await?;
    println!("new_did : {:#08X}", new_did);
    let created_did = new_did;

    let new_user = Users::new(
        user_info.user_id.clone(),
        passwd_hash,
        user_info.username.clone(),
        created_did,
    );

    create_user(&mut db_conn, new_user).await?;
    Ok(web::Json(ApiResult::ok::<usize>(None)))
}

#[derive(Deserialize)]
struct LoginInfo {
    pub user_id: String,
    pub passwd: String,
}

#[derive(Serialize)]
struct LoginResult {
    pub token: String,
}

/// Login as existing user
#[post("/login")]
async fn login(login_info: web::Json<LoginInfo>) -> Result<impl Responder, Error> {
    let mut db_conn = establish_connection().await;

    if !verify_user(
        &mut db_conn,
        login_info.user_id.clone(),
        login_info.passwd.clone(),
    )
    .await?
    {
        return Ok(web::Json(ApiResult::err("wrong id or password")));
    }

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(60)).timestamp() as usize;
    let claims = JwtClaim {
        sub: login_info.user_id.clone(),
        exp,
        iat,
    };

    let jwt_secret = env::var("JWT_SECRET").unwrap();
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .unwrap();

    let result = LoginResult { token };

    Ok(web::Json(ApiResult::ok(Some(result))))
}

#[post("/logout")]
async fn logout(_: JwtMiddleware) -> impl Responder {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({"result" : true}))
}
