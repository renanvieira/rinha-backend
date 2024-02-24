use std::{env, sync::Arc, time::Duration};

use postgres::{Client, NoTls};
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use rouille::{input::json_input, router, Response};
use scheduled_thread_pool::ScheduledThreadPool;

fn main() {
    let host = "0.0.0.0";
    let port = env::var("PORT").unwrap_or("9999".to_string());
    let db_url = env::var("DB_URL").unwrap_or("postgres://postgres:123@localhost/rinha".to_owned());
    let db_dsn = env::var("DB_DSN")
        .unwrap_or("host=localhost user=postgres password=123 dbname=rinha".to_owned());
    let pool_size: u32 = env::var("R2D2_MAX_POOL_SIZE")
        .unwrap_or("100".to_string())
        .parse()
        .unwrap_or(100);

    println!("Pool size: {}", pool_size);
    println!("Port: {}", port);
    println!("DB URL: {}", db_url);

    let thread_pool = ScheduledThreadPool::new(10);

    let pool_manager = PostgresConnectionManager::new(db_dsn.parse().unwrap(), NoTls);

    let start = std::time::Instant::now();
    let pool_result = Pool::builder()
        .thread_pool(Arc::new(thread_pool))
        .connection_timeout(Duration::from_secs(120))
        .max_size(pool_size)
        .build(pool_manager);
    let end = std::time::Instant::now();

    println!("Pool initialized: {:?}", end - start);

    let pool = match pool_result {
        Ok(p) => p,
        Err(e) => panic!("Error creating pool: {}", e),
    };

    rouille::start_server_with_pool(format!("{}:{}", host, port), None, move |request| {
        let result = router!(request,
            (GET) (/healthcheck) => {
                Response::text("OK")
            },
            (GET) (/clientes/{id:i16}/extrato) => {

                let result = {
                    let mut db = pool.get().unwrap();
                    let sql = "SELECT get_client_balance_and_transactions FROM get_client_balance_and_transactions($1::smallint)";
                     db.query_one(sql, &[&id]).unwrap()
                };

                let result_row: serde_json::Value = result.get(0);


                match result_row.get("error"){
                    Some(err) => match err.as_str(){
                        Some(err_msg) => match err_msg{
                            "client_not_found" => return Response::text("Invalid Request: client not found").with_status_code( 404),
                            _ => {}
                        },
                        None => todo!(),
                    },
                    None => {},
                }


                Response::json(&result_row)
            },
            (POST) (/clientes/{id:i16}/transacoes) => {
                let body = json_input::<serde_json::Value>(&request);


            let parsed_body = match body{
                Ok(b) => b,
                Err(e) => return Response::text(format!("Invalid Request: {}", e)).with_status_code( 422),
            };

            let description_result = parsed_body.get("descricao");
            let description = match description_result{
                Some(desc) => desc.to_string().trim_matches('\"').to_string(),
                None => return Response::text("Invalid Request: description field missing").with_status_code( 422),
            };

            if description.is_empty() || description.len() > 10 || description.len() == 0 {
                return Response::text("Invalid Request: description field invalid").with_status_code( 422);
            }

            let amount_result = parsed_body.get("valor");
            let amount = match amount_result{
                Some(amt) => match amt.as_i64(){
                    Some(int_amt) => int_amt as i32,
                    None => return Response::text("Invalid Request: amount field invalid").with_status_code( 422),
                },
                None => return Response::text("Invalid Request: amount field missing").with_status_code( 422),
            };

            let op_result = parsed_body.get("tipo");
            let operation : &str = match op_result{
                Some(op) => match op.as_str(){
                    Some(op_str) => match op_str{
                        "c" | "d" => op_str,
                        _ => return Response::text("Invalid Request: operation field invalid").with_status_code( 422),
                    },
                    None => return Response::text("Invalid Request: operation field invalid").with_status_code( 422),
                },
                None => return Response::text("Invalid Request: operation field missing").with_status_code( 422),
            };

            let result = {
                let sql = "SELECT insert_transaction($1::SMALLINT, $2::INTEGER, $3::CHAR, $4::VARCHAR)";
                let mut db = pool.get().unwrap();

                db.query_one(sql, &[&id,&amount,&operation, &description.to_string()]).unwrap()
            };

            let response :serde_json::Value= result.get(0);

            match response.get("error"){
                Some(err) => match err.as_str(){
                    Some(err_msg) => match err_msg{
                        "client_not_found" => return Response::text("Invalid Request: client not found").with_status_code( 404),
                        "not_enough_limit" | "invalid_operation" => return Response::text("Invalid Request").with_status_code( 422),
                        _ => {}
                    },
                    None => {},
                },
                None => {},
            }

            Response::json(&response)

            },
            _ => Response::empty_404()
        );

        result
    });
}
