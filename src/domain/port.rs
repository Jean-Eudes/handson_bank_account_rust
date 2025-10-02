use crate::domain::bank_account::BankAccount;
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait BankAccountPort: Send + Sync {
    fn save_account(&self, bank_account: &BankAccount);
    fn load(&self, account_number: &str) -> Option<BankAccount>;
}