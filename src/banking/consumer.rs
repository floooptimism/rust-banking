use crate::banking::bank::Bank;
use crate::banking::types::BankID;
use super::types::{AccountID, ID};

pub struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    pub fn new(first: String, last: String) -> Person {
        Person {
            first_name: first,
            last_name: last
        }
    }
}

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
        format!("{} {}", self.person.first_name, self.person.last_name)
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

pub struct ATMCard {
    id: ID,
    bank_id: BankID,
    account_id: AccountID
}

impl ATMCard {
    pub fn id(&self) -> ID {
        self.id
    }

    pub fn bank_id(&self) -> BankID {
        self.bank_id
    }

    pub fn account_id(&self) -> AccountID {
        self.account_id
    }
}