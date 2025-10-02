use crate::domain::bank_account::BankAccount;
use mockall::automock;

#[automock]
pub trait BankAccountPort: Send + Sync {
    fn save_account(&self, bank_account: &BankAccount);
    fn load(&self, account_number: &String) -> Option<BankAccount>;
}