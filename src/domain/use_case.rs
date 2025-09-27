use crate::domain::bank_account::BankAccount;
use crate::domain::port::BankAccountPort;

struct BankAccountUseCase {
    bank_account_port: Box<dyn BankAccountPort>,
}

impl BankAccountUseCase {
    fn new(adapter: Box<dyn BankAccountPort>) -> BankAccountUseCase {
        BankAccountUseCase {
            bank_account_port: adapter,
        }
    }

    fn create(mut self, account_number: String, initial_amount: i64) {
        let account = BankAccount::create_new_account(account_number, initial_amount);
        self.bank_account_port.save_account(account)
    }

    fn fetch(self, account_number: String) -> Option<BankAccount> {
        self.bank_account_port.load(&account_number)
    }
}

#[cfg(test)]
mod test {
    use mockall::predicate::eq;
    use crate::domain::bank_account::BankAccount;
    use crate::domain::port::MockBankAccountPort;
    use crate::domain::use_case::BankAccountUseCase;

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

    #[cfg(feature = "domain5")]
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
}
