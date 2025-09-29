use crate::domain::bank_account::BankAccount;
use crate::domain::port::BankAccountPort;
use serde::Serialize;
use std::collections::HashMap;

pub struct BankAccountAdapter {
    accounts: HashMap<String, BankAccount>
}

impl BankAccountAdapter {
    pub fn new() -> Self {
        BankAccountAdapter {
            accounts: HashMap::new(),
        }
    }
}

impl BankAccountPort for BankAccountAdapter {
    fn save_account(&mut self, bankAccount: BankAccount){
        self.accounts.insert(bankAccount.account_number().clone(), bankAccount);
    }

    fn load(&self, accountNumber: &String) -> Option<BankAccount> {
        let accountResult = self.accounts.get(accountNumber);
        accountResult.cloned()
    }
}

#[cfg(test)]
mod test {
    #[cfg(feature = "infra1")]
    #[test]
    fn should_save_account() {
        let account = BankAccount::create_new_account(String::from("A001"), 200);

        let mut repository = BankAccountAdapter::new();

        repository.save_account(account);
        assert!(repository.accounts.contains_key(&String::from("A001")));
        assert_eq!(repository.accounts.get(&String::from("A001")).unwrap().initial_amount(), 200);
        assert!(repository.accounts.get(&String::from("A001")).unwrap().transactions().is_empty());
    }
}
