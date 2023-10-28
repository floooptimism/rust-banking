use crate::banking::types::AccountID;
use super::person::Person;
pub struct Account {
    person: Person,
    id: AccountID,
    balance: i64
}

impl Account {
    pub fn new(person: Person, id: AccountID) -> Account {
        Account {
            person: person,
            id: id,
            balance: 0
        }
    }

    pub fn id(&self) -> AccountID {
        self.id
    }

    pub fn get_balance(&self) -> i64 {
        self.balance
    }

    pub fn get_name(&self) -> String {
        format!("{} {}", self.person.first_name(), self.person.last_name())
    }

    pub fn add(&mut self, amount: u64) {
        let amount = amount as i64;
        self.balance = self.balance + amount;
    }

    pub fn subtract(&mut self, amount: u64) {
        let amount = amount as i64;
        self.balance = self.balance - amount;
    }
}