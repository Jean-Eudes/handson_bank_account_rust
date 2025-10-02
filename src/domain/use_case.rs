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

    pub fn create(&mut self, account_number: String, initial_amount: i64) {
        let account = BankAccount::create_new_account(account_number, initial_amount);
        self.bank_account_port.save_account(&account)
    }

    pub fn withdraw(&mut self, account_number: String, amount: i64) -> Option<BankAccount> {
        let mut account = self.bank_account_port.load(&account_number)?;
        account.with_draw(amount);
        self.bank_account_port.save_account(&account);
        Some(account)
    }

    pub fn deposit(&mut self, account_number: String, amount: i64) -> Option<BankAccount> {
        let mut account = self.bank_account_port.load(&account_number)?;
        account.deposit(amount);
        self.bank_account_port.save_account(&account);
        Some(account)
    }

    pub fn fetch(&self, account_number: String) -> Option<BankAccount> {
        self.bank_account_port.load(&account_number)
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

        let mut user_case = BankAccountUseCase::new(Box::new(port));

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
            .withf(|ac| ac.balance() == 300)
            .return_const(());
        let mut user_case = BankAccountUseCase::new(Box::new(port));

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
            .withf(|ac| ac.balance() == 100)
            .return_const(());
        let mut user_case = BankAccountUseCase::new(Box::new(port));

        let result = user_case.withdraw(String::from("A0001"), 100);

        assert!(result.is_some());
    }
}
