use crate::banking::types::{AccountID, BankID, ID};

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