use validator::{Validate, ValidationError};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct AccountBalance {
    #[serde(rename = "total")]
    pub total: i32,
    #[serde(rename = "limite")]
    pub limit: i32,
    #[serde(rename = "data_extrato")]
    pub date: String,
}
#[derive(serde::Deserialize, serde::Serialize, Validate, Debug)]
pub struct Transactions {
    #[serde(rename = "tipo")]
    #[validate(custom = "validate_operation")]
    pub operation: String,
    #[serde(rename = "valor")]
    #[validate(range(min = 1, max = "i32::MAX"))]
    pub amount: i32,
    #[serde(rename = "descricao")]
    #[validate(length(min = 1, max = 10))]
    pub description: String,
    #[serde(rename = "realizada_em")]
    pub created_at: Option<String>,
}

fn validate_operation(operation: &str) -> Result<(), ValidationError> {
    match operation {
        "c" | "d" => Ok(()),
        _ => Err(ValidationError::new("invalid_operation")),
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ClientStatement {
    #[serde(rename = "saldo")]
    pub balance: AccountBalance,
    #[serde(rename = "ultimas_transacoes")]
    pub lastest_transactions: Vec<Transactions>,
}
