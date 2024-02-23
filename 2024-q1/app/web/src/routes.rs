use std::env;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use tokio_postgres::GenericClient;

use crate::{
    db::{FuncError, PostgressPoolConnection},
    error::AppError,
};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct AccountBalance {
    #[serde(rename = "total")]
    pub total: i32,
    #[serde(rename = "limite")]
    pub limit: i32,
    #[serde(rename = "data_extrato")]
    pub date: String,
}
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Transactions {
    #[serde(rename = "tipo")]
    pub operation: String,
    #[serde(rename = "valor")]
    pub amount: i32,
    #[serde(rename = "descricao")]
    pub description: String,
    #[serde(rename = "realizada_em")]
    pub created_at: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ClientStatement {
    #[serde(rename = "saldo")]
    pub balance: AccountBalance,
    #[serde(rename = "ultimas_transacoes")]
    pub lastest_transactions: Vec<Transactions>,
}

pub async fn get_client_statement(
    Path(id): Path<i16>,
    State(pool): State<PostgressPoolConnection>,
) -> Result<impl IntoResponse, AppError> {
    let conn = pool.get().await.unwrap();
    let result = conn
        .query_one(
            "SELECT get_client_balance_and_transactions FROM get_client_balance_and_transactions($1::smallint)",
            &[&id],
        )
        .await
        .unwrap();

    let response = result.get::<_, serde_json::Value>(0);

    // // TODO: Refactor
    match response.get("error") {
        Some(err) => match err.as_str() {
            Some(err_str) => match err_str {
                "client_not_found" => return Err(AppError::ClientNotFound),
                _ => {}
            },
            None => {}
        },
        None => {}
    }

    Ok((StatusCode::OK, Json(response)))
}

pub async fn create_transaction(
    Path(id): Path<i16>,
    State(pool): State<PostgressPoolConnection>,
    Json(transaction): Json<Transactions>,
) -> Result<impl IntoResponse, AppError> {
    if transaction.description.len() > 10 || transaction.description.len() == 0 {
        return Err(AppError::TransactionInvalid);
    }

    if transaction.operation != "d" && transaction.operation != "c" {
        return Err(AppError::TransactionInvalid);
    }

    let conn = pool.get().await.unwrap();

    let result = conn
        .query_one(
            "SELECT insert_transaction($1::SMALLINT, $2::INTEGER, $3::CHAR, $4::VARCHAR)",
            &[
                &id,
                &transaction.amount,
                &transaction.operation,
                &transaction.description,
            ],
        )
        .await
        .unwrap();

    let response = result.get::<_, serde_json::Value>(0);

    // TODO: Refactor
    match response.get("error") {
        Some(err) => match err.as_str() {
            Some(err_str) => match err_str {
                "client_not_found" => return Err(AppError::ClientNotFound),
                "not_enough_limit" | "invalid_operation" => {
                    return Err(AppError::TransactionInvalid);
                }
                _ => {}
            },
            None => {}
        },
        None => {}
    }

    Ok((StatusCode::OK, Json(response)))
}
