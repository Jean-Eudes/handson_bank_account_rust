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
    state
        .use_case
        .create(payload.account_id, payload.initial_amount);
    StatusCode::CREATED
}

pub async fn deposit(
    State(state): State<AppState>,
    Path(account_number): Path<String>,
    Json(payload): Json<DepositAndWithdrawInput>,
) -> Result<(StatusCode, Json<BankAccountResource>), StatusCode> {
    state
        .use_case
        .deposit(account_number, payload.amount)
        .map(|account| (StatusCode::OK, to_response(account)))
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn withdraw(
    State(state): State<AppState>,
    Path(account_number): Path<String>,
    Json(payload): Json<DepositAndWithdrawInput>,
) -> Result<(StatusCode, Json<BankAccountResource>), StatusCode> {
    state
        .use_case
        .withdraw(account_number, payload.amount)
        .map(|account| (StatusCode::OK, to_response(account)))
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn fetch(
    State(state): State<AppState>,
    Path(account_number): Path<String>,
) -> Result<(StatusCode, Json<BankAccountResource>), StatusCode> {
    state
        .use_case
        .fetch(account_number)
        .map(|account| (StatusCode::OK, to_response(account)))
        .ok_or(StatusCode::NOT_FOUND)
}

fn to_response(account: BankAccount) -> Json<BankAccountResource> {
    Json(BankAccountResource {
        account_id: String::from(account.account_number()),
        initial_amount: account.initial_amount(),
        balance: account.balance(),
    })
}
