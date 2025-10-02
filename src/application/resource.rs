use crate::domain::bank_account::BankAccount;
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct BankAccountInput {
    initial_amount: i64,
    account_id: String,
}

#[derive(Deserialize)]
pub struct DepositAndWithdrawInput {
    amount: i64,
}

#[derive(Serialize, Debug)]
pub struct BankAccountResource {
    initial_amount: i64,
    account_id: String,
    balance: i64,
}

pub async fn create_account(
    State(state): State<AppState>,
    Json(payload): Json<BankAccountInput>,
) -> StatusCode {
    todo!()
}

pub async fn deposit(
    State(state): State<AppState>,
    Path(account_number): Path<String>,
    Json(payload): Json<DepositAndWithdrawInput>,
) -> Result<(StatusCode, Json<BankAccountResource>), StatusCode> {
    todo!()
}

pub async fn withdraw(
    State(state): State<AppState>,
    Path(account_number): Path<String>,
    Json(payload): Json<DepositAndWithdrawInput>,
) -> Result<(StatusCode, Json<BankAccountResource>), StatusCode> {
    todo!()
}

pub async fn fetch(
    State(state): State<AppState>,
    Path(account_number): Path<String>,
) -> Result<(StatusCode, Json<BankAccountResource>), StatusCode> {
    todo!()
}