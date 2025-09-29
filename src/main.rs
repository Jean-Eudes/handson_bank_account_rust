use crate::application::resource::{create_account, fetch};
use crate::domain::use_case::BankAccountUseCase;
use crate::infrastructure::repository::BankAccountAdapter;
use axum::routing::{get, post};
use axum::Router;
use std::sync::{Arc, Mutex};
use tokio::signal;
use tracing::info;

mod application;
mod domain;
mod infrastructure;

#[derive(Clone)]
pub struct AppState {
    use_case: Arc<Mutex<BankAccountUseCase>>,
}

#[tokio::main]
async fn main() {
    let adapter = BankAccountAdapter::new();
    let use_case = Mutex::new(BankAccountUseCase::new(Box::new(adapter)));
    let router = router(use_case);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap()
}

fn router(use_case: Mutex<BankAccountUseCase>) -> Router {
    Router::new()
        .route("/accounts", post(create_account))
        .route("/accounts/{account_number}", get(fetch))
        .with_state(AppState {
            use_case: Arc::new(use_case),
        })
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    tokio::select! {_ = ctrl_c => {info!("received ctrl + C")}}
}
#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use crate::domain::bank_account::BankAccount;
    use crate::domain::port::MockBankAccountPort;
    use crate::domain::use_case::BankAccountUseCase;
    use crate::router;
    use axum::http::StatusCode;
    use axum_test::expect_json::__private::serde_json::json;
    use axum_test::TestServer;
    use mockall::predicate::eq;
    use std::sync::Mutex;

    #[tokio::test]
    async fn it_works() {
        let mut port = MockBankAccountPort::new();
        let account = BankAccount::create_new_account(String::from("A0001"), 200);
        port.expect_load()
            .once()
            .with(eq(String::from("A0001")))
            .return_const(account.clone());

        let server = setUpServer(port);

        let response = server.get("/accounts/A0001").await;

        assert_eq!(
            response.text(),
            String::from("{\"initial_amount\":200,\"account_id\":\"A0001\"}")
        );
    }

    #[tokio::test]
    async fn should_not_load_account_when_account_not_found() {
        let mut port = MockBankAccountPort::new();
        port.expect_load()
            .once()
            .with(eq(String::from("A0001")))
            .return_const(None);
        let server = setUpServer(port);

        let response = server.get("/accounts/A0001").await;

        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn should_create_account() {
        let mut port = MockBankAccountPort::new();
        let account = BankAccount::create_new_account(String::from("A0001"), 200);
        port.expect_save_account()
            .once()
            .with(eq(account))
            .return_const(());
        let server = setUpServer(port);

        let response = server
            .post("/accounts")
            .json(&json!({"initial_amount":200,"account_id":"A0001"}))
            .await;

        assert_eq!(response.status_code(), StatusCode::CREATED);
    }

    fn setUpServer(port: MockBankAccountPort) -> TestServer {
        let useCase = BankAccountUseCase::new(Box::new(port));
        let server = TestServer::new(router(Mutex::new(useCase))).unwrap();
        server
    }
}
