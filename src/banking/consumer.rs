use super::types::ID;

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
    id: ID,
    balance: i64
}

impl Account {
    pub fn new(person: Person, id: u64) -> Account {
        Account {
            person: person,
            id: id,
            balance: 0
        }
    }

    pub fn id(&self) -> ID {
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
    bank_id: ID,
    account_id: ID
}

impl ATMCard {
    pub fn id(&self) -> ID {
        self.id
    }

    pub fn bank_id(&self) -> ID {
        self.bank_id
    }

    pub fn account_id(&self) -> ID {
        self.account_id
    }
}