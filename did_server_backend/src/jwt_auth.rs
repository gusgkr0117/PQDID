use actix_web::error::ErrorUnauthorized;
use actix_web::http;
use actix_web::FromRequest;
use jsonwebtoken::decode;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::Validation;
use serde::{Deserialize, Serialize};
use std::env;
use std::future::{ready, Ready};

use crate::types::ApiResult;

#[derive(Deserialize, Serialize)]
pub struct JwtClaim {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

pub struct JwtMiddleware {
    pub user_id: String,
}

impl FromRequest for JwtMiddleware {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let jwt_secret = env::var("JWT_SECRET").unwrap();

        let token = req
            .cookie("token")
            .map(|c| c.value().to_string())
            .or_else(|| {
                req.headers()
                    .get(http::header::AUTHORIZATION)
                    .map(|h| h.to_str().unwrap().split_at(7).1.to_string())
            });

        if token.is_none() {
            let json_error = ApiResult::err("You are not logged in, please provide token");
            return ready(Err(ErrorUnauthorized(json_error)));
        }

        let claims = match decode::<JwtClaim>(
            &token.unwrap(),
            &DecodingKey::from_secret(jwt_secret.as_ref()),
            &Validation::default(),
        ) {
            Ok(c) => c.claims,
            Err(_) => {
                let json_error = ApiResult::err("Invalid token");
                return ready(Err(ErrorUnauthorized(json_error)));
            }
        };

        ready(Ok(JwtMiddleware {
            user_id: claims.sub,
        }))
    }
}
