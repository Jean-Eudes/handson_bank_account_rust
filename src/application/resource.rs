use crate::domain::bank_account::BankAccount;
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct BankAccountInput {
    initial_amount: i64,
    account_id: String,
}

#[derive(Deserialize)]
pub struct DepositAndWithdrawInput {
    amount: i64
}

#[derive(Serialize, Debug)]
pub struct BankAccountResource {
    initial_amount: i64,
    account_id: String,
    balance: i64
}

pub async fn create_account(State(state): State<AppState>, Json(payload): Json<BankAccountInput>) -> impl IntoResponse {
    state
        .use_case
        .create(payload.account_id, payload.initial_amount);
    StatusCode::CREATED
}

pub async fn deposit(State(state): State<AppState>, Path(account_number): Path<String>, Json(payload): Json<DepositAndWithdrawInput>)-> impl IntoResponse {
    state
        .use_case
        .deposit(account_number, payload.amount)
        .map(|account| to_response(account))
        .unwrap_or_else(|| StatusCode::NOT_FOUND.into_response())
}

pub async fn withdraw(State(state): State<AppState>, Path(account_number): Path<String>, Json(payload): Json<DepositAndWithdrawInput>) -> impl IntoResponse {
    state
        .use_case
        .withdraw(account_number, payload.amount)
        .map(|account| to_response(account))
        .unwrap_or_else(|| StatusCode::NOT_FOUND.into_response())
}

pub async fn fetch(
    State(state): State<AppState>,
    Path(account_number): Path<String>,
) -> impl IntoResponse {
    let found_bank_account = state
        .use_case
        .fetch(account_number);

    match found_bank_account {
        Some(account) => (
            StatusCode::OK,
            Json(BankAccountResource {
                account_id: String::from(account.account_number()),
                initial_amount: account.initial_amount(),
                balance: account.balance()
            }),
        ).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

fn to_response(account: BankAccount) -> Response {
    (StatusCode::OK,
     Json(BankAccountResource {
         account_id: String::from(account.account_number()),
         initial_amount: account.initial_amount(),
         balance: account.balance(),
     })).into_response()
}
