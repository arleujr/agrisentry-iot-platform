use thiserror::Error;
use actix_web::{HttpResponse, ResponseError};

#[derive(Error, Debug)]
pub enum GatewayError {
    #[error("Internal Database driver error state structural failure")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Network connection interface serialization or timeout failure")]
    NetworkError(#[from] reqwest::Error),
    #[error("Requested remote hardware signature was not identified")]
    HardwareNotFound,
}

impl ResponseError for GatewayError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GatewayError::HardwareNotFound => HttpResponse::NotFound().json(serde_json::json!({ "error": self.to_string() })),
            _ => HttpResponse::InternalServerError().json(serde_json::json!({ "error": "Internal infrastructure breakdown encountered" })),
        }
    }
}