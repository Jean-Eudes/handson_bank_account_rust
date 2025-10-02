use crate::domain::bank_account::BankAccount;
use crate::domain::port::BankAccountPort;
use std::collections::HashMap;
use std::sync::Mutex;

pub struct BankAccountAdapter {
    accounts: Mutex<HashMap<String, BankAccount>>,
}

impl BankAccountAdapter {
    pub fn new() -> Self {
        BankAccountAdapter {
            accounts: Mutex::new(HashMap::new()),
        }
    }
}

impl BankAccountPort for BankAccountAdapter {
    fn save_account(&self, bank_account: &BankAccount) {
        self.accounts.lock().unwrap().insert(bank_account.account_number().to_string(), bank_account.clone());
    }

    fn load(&self, account_number: &str) -> Option<BankAccount> {
        let lock = self.accounts.lock().unwrap();
        let account_result = lock.get(account_number);
        account_result.cloned()
    }
}
#[allow(unused_imports)]
#[cfg(test)]
mod test {
    use axum_test::util::new_random_port;
    use crate::domain::bank_account::BankAccount;
    use crate::domain::port::BankAccountPort;
    use crate::infrastructure::repository::BankAccountAdapter;

    #[cfg(feature = "infra1")]
    #[test]
    fn should_save_account() {
        let account = BankAccount::create_new_account(String::from("A001"), 200);

        let repository = BankAccountAdapter::new();

        repository.save_account(&account);
        let lock = repository.accounts.lock().unwrap();
        assert!(lock.contains_key("A001"));
        assert_eq!(lock.get("A001").unwrap().initial_amount(), 200);
        assert!(lock.get("A001").unwrap().transactions().is_empty());
    }
    #[cfg(feature = "infra1")]
    #[test]
    fn should_load_account() {
        let repository = BankAccountAdapter::new();
        let account = BankAccount::create_new_account(String::from("A001"), 200);
        let mut lock = repository.accounts.lock().unwrap();
        lock.insert(String::from("A001"), account.clone());
        drop(lock);
        let result = repository.load("A001");

        assert_eq!(result, Some(account))
    }
}
