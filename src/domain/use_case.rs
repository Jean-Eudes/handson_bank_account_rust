use crate::domain::bank_account::BankAccount;
use crate::domain::port::BankAccountPort;

pub struct BankAccountUseCase {
    bank_account_port: Box<dyn BankAccountPort>,
}

impl BankAccountUseCase {
    pub fn new(adapter: Box<dyn BankAccountPort>) -> BankAccountUseCase {
        BankAccountUseCase {
            bank_account_port: adapter,
        }
    }

    pub fn create(&self, account_number: String, initial_amount: i64) {
        todo!()
    }

    pub fn withdraw(&self, account_number: String, amount: i64) -> Option<BankAccount> {
        todo!()
    }

    pub fn deposit(&self, account_number: String, amount: i64) -> Option<BankAccount> {
        todo!()
    }

    pub fn fetch(&self, account_number: String) -> Option<BankAccount> {
        todo!()
    }
}

#[allow(unused_imports)]
#[cfg(test)]
mod test {
    use crate::domain::bank_account::{BankAccount, Transaction};
    use crate::domain::port::MockBankAccountPort;
    use crate::domain::use_case::BankAccountUseCase;
    use mockall::predicate;
    use mockall::predicate::eq;

    #[cfg(feature = "domain4")]
    #[test]
    fn should_create_account() {
        let mut port = MockBankAccountPort::new();
        let account = BankAccount::create_new_account(String::from("A0001"), 200);
        port.expect_save_account()
            .once()
            .with(eq(account))
            .return_const(());

        let user_case = BankAccountUseCase::new(Box::new(port));

        user_case.create(String::from("A0001"), 200);
    }

    #[cfg(feature = "domain4")]
    #[test]
    fn should_load_account() {
        let mut port = MockBankAccountPort::new();
        let account = BankAccount::create_new_account(String::from("A0001"), 200);
        port.expect_load()
            .once()
            .with(eq(String::from("A0001")))
            .return_const(account.clone());

        let user_case = BankAccountUseCase::new(Box::new(port));

        assert_eq!(user_case.fetch(String::from("A0001")), Some(account));
    }

    #[cfg(feature = "domain4")]
    #[test]
    fn should_deposit_in_account() {
        let mut port = MockBankAccountPort::new();
        let account = BankAccount::create_new_account(String::from("A0001"), 200);
        port.expect_load()
            .once()
            .with(eq(String::from("A0001")))
            .return_const(account.clone());
        port.expect_save_account()
            .once()
            .withf(|ac| {
                matches!(
                    ac.transactions().get(0).unwrap(),
                    Transaction::Deposit {
                        date: _date,
                        amount: 100
                    }
                )
            })
            .return_const(());
        let user_case = BankAccountUseCase::new(Box::new(port));

        let result = user_case.deposit(String::from("A0001"), 100);

        assert!(result.is_some());
    }

    #[cfg(feature = "domain4")]
    #[test]
    fn should_withdraw_from_account() {
        let mut port = MockBankAccountPort::new();
        let account = BankAccount::create_new_account(String::from("A0001"), 200);
        port.expect_load()
            .once()
            .with(eq(String::from("A0001")))
            .return_const(account.clone());
        port.expect_save_account()
            .once()
            .withf(|ac| {
                matches!(
                    ac.transactions().get(0).unwrap(),
                    Transaction::Withdraw {
                        date: _date,
                        amount: 100
                    }
                )
            })

            .return_const(());
        let user_case = BankAccountUseCase::new(Box::new(port));

        let result = user_case.withdraw(String::from("A0001"), 100);

        assert!(result.is_some());
    }
}
