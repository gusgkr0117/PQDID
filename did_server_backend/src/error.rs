use actix_web::{
    self,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use anyhow;
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum Error {
    #[display(fmt = "anyhow error")]
    AnyhowError,
    #[display(fmt = "web error")]
    ActixWebError,
    #[display(fmt = "diesel error")]
    DieselError,
    #[display(fmt = "json error")]
    JsonError,
    #[display(fmt = "string error")]
    StrError,
    #[display(fmt = "parse int error")]
    ParseIntError,
}

impl actix_web::error::ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            Error::AnyhowError => StatusCode::INTERNAL_SERVER_ERROR,
            Error::ActixWebError => StatusCode::INTERNAL_SERVER_ERROR,
            Error::DieselError => StatusCode::INTERNAL_SERVER_ERROR,
            Error::JsonError => StatusCode::INTERNAL_SERVER_ERROR,
            Error::StrError => StatusCode::INTERNAL_SERVER_ERROR,
            Error::ParseIntError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<actix_web::Error> for Error {
    fn from(_: actix_web::Error) -> Self {
        Error::ActixWebError
    }
}

impl From<anyhow::Error> for Error {
    fn from(_: anyhow::Error) -> Self {
        Error::AnyhowError
    }
}

impl From<diesel::result::Error> for Error {
    fn from(_: diesel::result::Error) -> Self {
        Error::DieselError
    }
}

impl From<serde_json::Error> for Error {
    fn from(_: serde_json::Error) -> Self {
        Error::JsonError
    }
}

impl From<&str> for Error {
    fn from(_: &str) -> Self {
        Error::StrError
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(_: std::num::ParseIntError) -> Self {
        Error::ParseIntError
    }
}
