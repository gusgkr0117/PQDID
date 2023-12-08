use std::env;

use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha512};

use crate::{
    database::{create_user, establish_connection, models::Users, verify_user},
    did_protocol::protocol::create_wallet,
    error::Error,
    jwt_auth::{JwtClaim, JwtMiddleware},
    pqc_sign::types::PublicKey,
    types::ApiResult,
};
use actix_web::{cookie::time::Duration as ActixWebDuration, HttpResponse};
use actix_web::{cookie::Cookie, post, web, Responder, Result};

#[derive(Deserialize)]
struct JoinInfo {
    pub user_id: String,
    pub passwd: String,
    pub username: String,
    pub public_key: PublicKey,
}

/// Create a new user and create a new did wallet
#[post("/join")]
async fn join(user_info: web::Json<JoinInfo>) -> Result<impl Responder, Error> {
    let mut db_conn = establish_connection().await;
    let mut hasher = Sha512::new();
    hasher.update(user_info.passwd.as_bytes());
    let passwd_hash: String = hex::encode_upper(hasher.finalize());

    // TODO: check if the user_id is actually not duplicated

    // Create a new wallet
    let response = create_wallet(user_info.public_key.clone()).await?;
    let created_did = response.did;

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
