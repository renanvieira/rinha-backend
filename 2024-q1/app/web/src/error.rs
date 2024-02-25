use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use postgres_types::{FromSql, Type};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub message: String,
}

impl ErrorResponse {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

#[derive(Debug)]
pub enum AppError {
    ClientNotFound,
    TransactionInvalid,
    InternalError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::ClientNotFound => (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new("Client nao encontrado".to_owned())),
            )
                .into_response(),
            AppError::TransactionInvalid => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(ErrorResponse::new("Transacao invalida".to_owned())),
            )
                .into_response(),
            AppError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Error".to_owned(),
            )
                .into_response(),
        }
    }
}
