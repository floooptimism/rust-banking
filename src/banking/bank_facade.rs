use crate::banking::bank::{Bank, BankTransaction};
use crate::banking::bank::errors::BankError;
use crate::banking::types::AccountID;

use crate::banking::consumer::person::Person;

pub struct BankFacade<'a> {
    bank: &'a mut Bank
}

impl<'a> BankFacade<'a> {
    pub fn new(bank: &mut Bank) -> BankFacade {
        BankFacade {
            bank
        }
    }

    pub fn create_account(&mut self, person: Person, username: String, password: String) -> AccountID {
        self.bank.new_account(person, username, password)
    }

    pub fn transfer_to(&mut self, from: AccountID, to: AccountID, amount: u64) -> Result<(), BankError> {
        self.bank.process_transaction(BankTransaction::InternalTransfer {
            from, to, amount
        })
    }
    pub fn get_balance(&mut self, account_id: AccountID) -> i64 {
        self.bank.get_account(account_id).unwrap().get_balance()
    }

    pub fn add_to_account(&mut self, account_id: AccountID, amount: u64) {
        let account = self.bank.get_account(account_id).unwrap();
        account.add(amount);
    }

    pub fn subtract_from_account(&mut self, account_id: AccountID, amount:u64) {
        let account = self.bank.get_account(account_id).unwrap();
        account.subtract(amount);
    }


}