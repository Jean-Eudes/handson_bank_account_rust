use chrono::{DateTime, Utc};

#[derive(Clone, PartialEq, Debug)]
pub struct BankAccount {
    account_number: String,
    initial_amount: i16,
    transactions: Vec<Transaction>,
}

impl BankAccount {
    fn create_new_account(account_number: String, initial_amount: i16) -> BankAccount {
        return BankAccount {
            account_number,
            initial_amount,
            transactions: Vec::new()
        };
    }

    fn balance(&self) -> i16 {
        self.initial_amount
    }

    fn deposit(&mut self, amount: i16) {
        let transaction: Transaction = Transaction::Deposit { amount: amount, date: Utc::now() };
        self.apply_transaction(transaction);
    }

    fn withdraw(&mut self, amount: i16) {
        let transaction: Transaction = Transaction::Withdraw { amount: amount, date: Utc::now() };
        self.apply_transaction(transaction);
    }

    fn apply_transaction(&mut self, transaction: Transaction) {
        self.initial_amount += transaction.amount();
        self.transactions.push(transaction);
    }

}

#[derive(Clone, PartialEq, Debug)]
pub enum Transaction {
    Deposit { amount: i16, date: DateTime<Utc>},
    Withdraw { amount: i16, date: DateTime<Utc>},
}

impl Transaction {
    fn amount(&self) -> i16 {
        match self {
            // Transaction::Deposit { amount, date } => *amount, // déréfencement car les var internes sont aussi par défaurt des référeence et il faut pour les types primitifs utiliser des valeurs et non des références
            Transaction::Deposit { amount, date } => amount * 1, // la multiplication par 1 va recréer un résultat qui sera une valeur et non une référence
            Transaction::Withdraw { amount, date } => amount * -1,
        }
    }
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

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

        // When / Then
        assert_eq!(account.balance(), 1_000);
    }

    #[cfg(feature = "domain2")]
    #[test]
    fn should_compute_transaction_deposit_amount() {
        // Given
        let transaction = Transaction::Deposit {
            amount: 1_000,
            date: Utc::now(),
        };

        // When & Then
        assert_eq!(transaction.amount(), 1_000);
    }

    #[cfg(feature = "domain2")]
    #[test]
    fn should_compute_transaction_withdraw_amount() {
        // Given
        let transaction = Transaction::Withdraw {
            amount: 1_000,
            date: Utc::now(),
        };

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
    fn should_withdraw_to_bank_account() {
        // Given
        let mut account = BankAccount::create_new_account("account_number".to_string(), 100);

        // When
        account.withdraw(500);

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
        account.withdraw(500);
        account.deposit(2000);

        // Then
        assert_eq!(account.balance(), 2_500);
    }
}
