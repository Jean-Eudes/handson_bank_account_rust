use crate::domain::bank_account::BankAccount;
use mockall::automock;

#[automock]
pub trait BankAccountPort: Send + Sync {
    fn save_account(&mut self, bankAccount: BankAccount);
    fn load(&self, accountNumber: &String) -> Option<BankAccount>;
}