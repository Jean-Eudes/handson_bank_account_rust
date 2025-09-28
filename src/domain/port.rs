use mockall::automock;
use crate::domain::bank_account::{BankAccount, Transaction};

#[automock]
pub trait BankAccountPort {
    fn save_account(&mut self, bankAccount: BankAccount);
    fn load(&self, accountNumber: &String) -> Option<BankAccount>;
}