use hyper::{body::Buf, server::conn::http1};
use std::{convert::Infallible, env, i128::MAX};
use validator::Validate;

use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::{body::Bytes, Method, Request, Response, StatusCode};

use crate::model::Transactions;

static REGEX_CLIENT_ID: &'static str = r"^/clientes/(\d+)/(.*)";
static MAX_AMOUNT: i64 = i32::MAX as i64;
static OP_MESSAGE: &'static str = "Invalid OPeration";
static DESC_MESSAGE: &'static str = "Invalid Description";
static AMOUNT_MESSAGE: &'static str = "Invalid Amount";

fn full<T: Into<Bytes>>(body: T) -> BoxBody<Bytes, Infallible> {
    Full::new(body.into()).boxed()
}

pub async fn get_client_overview(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, Infallible>>, hyper::Error> {
    let url_regex = regex::Regex::new(REGEX_CLIENT_ID).unwrap();
    let mut resource = req.uri().path();
    let mut id: Option<i16> = None;

    if url_regex.is_match(req.uri().path()) {
        let captures = url_regex.captures(req.uri().path()).unwrap();

        id = Some(captures.get(1).unwrap().as_str().parse::<i16>().unwrap());
        resource = captures.get(2).unwrap().as_str();
    }

    match (req.method(), resource) {
        (&Method::GET, "/healthcheck") => Ok(Response::builder()
            .status(StatusCode::OK)
            .body(Empty::new().boxed())
            .unwrap()),
        (&Method::GET, "extrato") => {
            let (client, conn) =
                tokio_postgres::connect(&env::var("DB_DSN").unwrap(), tokio_postgres::NoTls)
                    .await
                    .unwrap();

            tokio::spawn(async move {
                if let Err(e) = conn.await {
                    eprintln!("connection error: {}", e);
                }
            });

            let overview_result = client
                .query_one(
                    "SELECT get_client_balance_and_transactions($1::smallint)",
                    &[&id],
                )
                .await;

            let overview = match overview_result {
                Ok(r) => r,
                Err(e) => panic!("Error: {}", e),
            };

            let response = overview.get::<_, serde_json::Value>(0);

            // // TODO: Refactor
            match response.get("error") {
                Some(err) => match err.as_str() {
                    Some(err_str) => match err_str {
                        "client_not_found" => {
                            let body = Full::new(Bytes::from("Client not found"));

                            return Ok(Response::builder()
                                .status(StatusCode::NOT_FOUND)
                                .body(body.boxed())
                                .unwrap());
                        }
                        _ => {}
                    },
                    None => {}
                },
                None => {}
            }

            let body = Bytes::from(serde_json::to_string(&response).unwrap());

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(Full::new(body).boxed())
                .unwrap())
        }
        (&Method::POST, "transacoes") => {
            let raw_body = req.into_body().collect().await.unwrap().aggregate();
            let body_serialized: Transactions = serde_json::from_reader(raw_body.reader()).unwrap();

            let body = match body_serialized.validate() {
                Ok(_) => body_serialized,
                Err(e) => {
                    return Ok(Response::builder()
                        .status(StatusCode::UNPROCESSABLE_ENTITY)
                        .body(full(format!("Invalid Request: {}", e)))
                        .unwrap());
                }
            };

            let (client, conn) =
                tokio_postgres::connect(&env::var("DB_DSN").unwrap(), tokio_postgres::NoTls)
                    .await
                    .unwrap();

            tokio::spawn(async move {
                if let Err(e) = conn.await {
                    eprintln!("connection error: {}", e);
                }
            });

            let result = client
                .query_one(
                    "SELECT insert_transaction($1::SMALLINT, $2::INTEGER, $3::CHAR, $4::VARCHAR)",
                    &[&id, &body.amount, &body.operation, &body.description],
                )
                .await
                .unwrap();

            let response = result.get::<_, serde_json::Value>(0);

            // TODO: Refactor
            match response.get("error") {
                Some(err) => match err.as_str() {
                    Some(err_str) => match err_str {
                        "client_not_found" => {
                            let body = Full::new(Bytes::from("Client not found"));

                            return Ok(Response::builder()
                                .status(StatusCode::NOT_FOUND)
                                .body(body.boxed())
                                .unwrap());
                        }
                        "not_enough_limit" | "invalid_operation" => {
                            return Ok(Response::builder()
                                .status(StatusCode::UNPROCESSABLE_ENTITY)
                                .body(Empty::new().boxed())
                                .unwrap())
                        }
                        _ => {}
                    },
                    None => {}
                },
                None => {}
            }

            let body = Bytes::from(serde_json::to_string(&response).unwrap());

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(Full::new(body).boxed())
                .unwrap())
        }
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Full::new(Bytes::from("Route Not found")).boxed())
            .unwrap()),
    }
}
