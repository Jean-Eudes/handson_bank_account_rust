use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Clone, Debug, PartialEq)]
pub struct BankAccount {
    account_number: String,
    initial_amount: i64,
    transactions: Vec<Transaction>
}

impl BankAccount {
    pub fn create_new_account(account_number: String, initial_amount: i64) -> BankAccount {
        BankAccount{
         account_number, initial_amount, transactions: vec![]
        }
    }
    pub fn balance(&self) -> i64 {
        self.transactions.iter()
            .fold(self.initial_amount, |acc, transaction| acc + transaction.amount())
    }

    pub fn deposit(&mut self, amount: i64) {
        self.transactions.push(Transaction::Deposit {amount, date: Utc::now()})
    }
    pub fn with_draw(&mut self, amount: i64) {
        self.transactions.push(Transaction::Withdraw {amount, date: Utc::now()})
    }

    pub fn account_number(&self) -> &String {
        &self.account_number
    }

    pub fn initial_amount(&self) -> i64 {
        self.initial_amount
    }

    pub fn transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Transaction {
    Deposit { amount: i64, date: DateTime<Utc> },
    Withdraw { amount: i64, date: DateTime<Utc> },
}

impl Transaction {
    pub fn amount(&self) -> i64 {
        match self {
            Transaction::Deposit{amount, ..} => *amount,
            Transaction::Withdraw{amount, ..} => -amount
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn truc() {
        let time = Utc::now();
    }

    #[cfg(feature = "domain1")]
    #[test]
    fn should_create_new_bank_account() {
        // Given / When
        let account = BankAccount::create_new_account("account_number".to_string(), 100);

        // Then
        assert_eq!(account.account_number, "account_number");
        assert_eq!(account.initial_amount, 100);
    }

    #[cfg(feature = "domain1")]
    #[test]
    fn should_get_balance() {
        // Given
        let account = BankAccount::create_new_account("account_number".to_string(), 1_000);

        // When & Then
        assert_eq!(account.balance(), 1_000);
    }

    #[cfg(feature = "domain2")]
    #[test]
    fn should_compute_transaction_deposit_amount() {
        // Given
        let transaction = Transaction::Deposit { amount: 1_000, date: Utc::now() };

        // When & Then
        assert_eq!(transaction.amount(), 1_000);
    }

    #[cfg(feature = "domain2")]
    #[test]
    fn should_compute_transaction_withdraw_amount() {
        // Given
        let transaction = Transaction::Withdraw { amount: 1_000, date: Utc::now() };

        // When & Then
        assert_eq!(transaction.amount(), -1_000);
    }

    #[cfg(feature = "domain2")]
    #[test]
    fn should_create_new_bank_account_with_transaction() {
        // Given / When
        let account = BankAccount::create_new_account("account_number".to_string(), 100);

        // Then
        assert_eq!(account.account_number, "account_number");
        assert_eq!(account.transactions.len(), 0);
        assert_eq!(account.initial_amount, 100);
    }

    #[cfg(feature = "domain3")]
    #[test]
    fn should_deposit_to_bank_account() {
        // Given
        let mut account = BankAccount::create_new_account("account_number".to_string(), 100);

        // When
        account.deposit(1000);

        // Then
        assert_eq!(
            matches!(
                account.transactions[0],
                Transaction::Deposit {
                    date: _date,
                    amount: 1000
                }
            ),
            true
        );
    }

    #[cfg(feature = "domain3")]
    #[test]
    fn should_with_draw_to_bank_account() {
        // Given
        let mut account = BankAccount::create_new_account("account_number".to_string(), 100);

        // When
        account.with_draw(500);

        // Then
        assert_eq!(
            matches!(
                account.transactions[0],
                Transaction::Withdraw {
                    date: _date,
                    amount: 500
                }
            ),
            true
        );
    }

    #[cfg(feature = "domain3")]
    #[test]
    fn should_compute_balance() {
        // Given
        let mut account = BankAccount::create_new_account("account_number".to_string(), 1000);

        // When
        account.with_draw(500);
        account.deposit(2000);

        // Then
        assert_eq!(account.balance(), 2_500);
    }
}
