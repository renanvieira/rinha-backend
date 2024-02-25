use std::env;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use tokio_postgres::{GenericClient, NoTls};

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
    // let conn = pool.get().await.unwrap();
    let (client, connection) = tokio_postgres::connect(&env::var("DB_DSN").unwrap(), NoTls)
        .await
        .unwrap();

    tokio::task::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("COnnection error: {}", e);
        }
    });

    let result = client
        .query_one(
            "SELECT get_client_balance_and_transactions FROM get_client_balance_and_transactions($1::smallint)",
            &[&id],
        )
        .await
        .unwrap();

    let response = result.get::<_, serde_json::Value>(0);

    if let Some(err) = response.get("error").and_then(|e| e.as_str()) {
        return match err {
            "client_not_found" => Err(AppError::ClientNotFound),
            _ => Err(AppError::ClientNotFound),
        };
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

    // let conn = pool.get().await.unwrap();

    let (client, connection) = tokio_postgres::connect(&env::var("DB_DSN").unwrap(), NoTls)
        .await
        .unwrap();

    tokio::task::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("COnnection error: {}", e);
        }
    });

    let result = client
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

    if let Some(err) = response.get("error").and_then(|e| e.as_str()) {
        return match err {
            "client_not_found" => Err(AppError::ClientNotFound),
            "not_enough_limit" | "invalid_operation" => Err(AppError::TransactionInvalid),
            _ => Err(AppError::TransactionInvalid),
        };
    }

    Ok((StatusCode::OK, Json(response)))
}
