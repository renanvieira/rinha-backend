use deadpool::managed::Pool;
use deadpool_postgres::Manager;
use postgres_types::FromSql;
use serde::{Deserialize, Serialize};

// pub type PostgressPoolConnection = Pool<PostgresConnectionManager<NoTls>>;
pub type PostgressPoolConnection = Pool<Manager>;

#[derive(Debug, Serialize, Deserialize)]
pub struct FuncError {
    pub error: String,
}

impl<'a> FromSql<'a> for FuncError {
    fn from_sql(
        ty: &postgres_types::Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        let error = String::from_utf8(raw.to_vec()).unwrap();

        Ok(FuncError {
            error: serde_json::from_str(&error)?,
        })
    }

    fn accepts(ty: &postgres_types::Type) -> bool {
        true
    }
}
