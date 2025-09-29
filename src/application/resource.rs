use crate::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct BankAccountInput {
    initial_amount: i64,
    account_id: String,
}

#[derive(Serialize, Debug)]
pub struct BankAccountResource {
    initial_amount: i64,
    account_id: String,
}

pub async fn create_account(State(state): State<AppState>, Json(payload): Json<BankAccountInput>) -> impl IntoResponse {
    state
        .use_case
        .lock()
        .unwrap()
        .create(payload.account_id, payload.initial_amount);
    StatusCode::CREATED
}

pub async fn fetch(
    State(state): State<AppState>,
    Path(account_number): Path<String>,
) -> impl IntoResponse {
    let foundBankAccount = state.use_case.lock().unwrap().fetch(account_number);
    match foundBankAccount {
        Some(account) => (
            StatusCode::OK,
            Json(BankAccountResource {
                account_id: String::from(account.account_number()),
                initial_amount: account.initial_amount(),
            }),
        )
            .into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

// Est ce vraiment utile vu la compléxité du test et les tests déjà fait du côté main
#[cfg(test)]
mod tests {
    use crate::application::resource::{
        create_account, fetch, BankAccountInput, BankAccountResource,
    };
    use crate::domain::bank_account::BankAccount;
    use crate::domain::port::MockBankAccountPort;
    use crate::domain::use_case::BankAccountUseCase;
    use crate::AppState;
    use axum::extract::{Path, State};
    use axum::http::StatusCode;
    use axum::response::IntoResponse;
    use axum::Json;
    use mockall::predicate::eq;
    use std::sync::{Arc, Mutex};

    #[tokio::test]
    async fn should_load_account() {
        let mut port = MockBankAccountPort::new();
        let account = BankAccount::create_new_account(String::from("A0001"), 200);
        port.expect_load()
            .once()
            .with(eq(String::from("A0001")))
            .return_const(account.clone());
        let useCase = BankAccountUseCase::new(Box::new(port));
        let state = AppState {
            use_case: Arc::new(Mutex::new(useCase)),
        };

        let result = fetch(State(state), Path(String::from("A0001")))
            .await
            .into_response();

        let expectedResponse = (
            StatusCode::OK,
            Json(BankAccountResource {
                initial_amount: 200,
                account_id: String::from("A0001"),
            }),
        )
            .into_response();
        assert_eq!(result.status(), expectedResponse.status());
    }

    #[tokio::test]
    async fn should_not_load_account_when_account_not_found() {
        let mut port = MockBankAccountPort::new();
        port.expect_load()
            .once()
            .with(eq(String::from("A0001")))
            .return_const(None);
        let useCase = BankAccountUseCase::new(Box::new(port));
        let state = AppState {
            use_case: Arc::new(Mutex::new(useCase)),
        };

        let result = fetch(State(state), Path(String::from("A0001")))
            .await
            .into_response();

        let expectedResponse = StatusCode::NOT_FOUND.into_response();
        assert_eq!(result.status(), expectedResponse.status());
    }

    #[tokio::test]
    async fn should_create_account() {
        let mut port = MockBankAccountPort::new();
        let account = BankAccount::create_new_account(String::from("A0001"), 200);
        port.expect_save_account()
            .once()
            .with(eq(account))
            .return_const(());
        let useCase = BankAccountUseCase::new(Box::new(port));
        let state = AppState {
            use_case: Arc::new(Mutex::new(useCase)),
        };

        let result = create_account(
            State(state),
            Json(BankAccountInput {
                account_id: String::from("A0001"),
                initial_amount: 200,
            }),
        )
        .await
        .into_response();

        let expectedResponse = StatusCode::CREATED.into_response();
        assert_eq!(result.status(), expectedResponse.status());
    }
}
