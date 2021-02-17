use std::convert::From;

use actix_web::HttpResponse;
use diesel::result::Error as DBError;
use juniper::graphql_value;
use r2d2::Error as R2D2Error;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum ServiceError {
    #[error("Internal Server Error")]
    InternalServerError,

    #[error("BadRequest: {0}")]
    BadRequest(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Unable to connect to DB")]
    UnableToConnectToDb,
}

impl juniper::IntoFieldError for ServiceError {
    fn into_field_error(self) -> juniper::FieldError {
        match self {
            ServiceError::Unauthorized => juniper::FieldError::new(
                "Unauthorized",
                graphql_value!({
                    "type": "NO_ACCESS"
                }),
            ),
            ServiceError::BadRequest(s) => juniper::FieldError::new(
                s,
                graphql_value!({
                    "type": "BAD_REQUEST"
                }),
            ),
            ServiceError::InternalServerError => juniper::FieldError::new(
                "Internal Error",
                graphql_value!({
                    "type": "INTERNAL_ERROR"
                }),
            ),
            ServiceError::UnableToConnectToDb => juniper::FieldError::new(
                "Unable to connect to DB",
                graphql_value!({
                    "type": "DB_CONNECTION_ERROR"
                }),
            ),
        }
    }
}

impl actix_web::error::ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
            ServiceError::UnableToConnectToDb => HttpResponse::InternalServerError()
                .json("Unable to connect to DB, Please try later"),
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
        }
    }
}

impl From<DBError> for ServiceError {
    fn from(error: DBError) -> ServiceError {
        match error {
            DBError::DatabaseError(_kind, info) => {
                let message = info.details().unwrap_or_else(|| info.message()).to_string();
                ServiceError::BadRequest(message)
            }
            _ => ServiceError::InternalServerError,
        }
    }
}

impl From<R2D2Error> for ServiceError {
    fn from(_: R2D2Error) -> ServiceError {
        ServiceError::UnableToConnectToDb
    }
}

pub type ServiceResult<V> = std::result::Result<V, crate::errors::ServiceError>;
