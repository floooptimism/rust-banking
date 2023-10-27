use std::collections::HashMap;


use super::types::ID;
use super::bank::{Bank, TransferFee};

pub struct BankAssociation {
    banks: HashMap<ID, Bank>,
    bank_increment_id: ID,
}

impl BankAssociation {

    pub fn new() -> BankAssociation {
        BankAssociation {
            banks: HashMap::new(),
            bank_increment_id: 0,
        }
    }

    pub fn create_bank(&mut self, transfer_fee: TransferFee) -> ID {
        let bank_id = self.bank_increment_id;
        let new_bank = Bank::new(bank_id, transfer_fee);
        self.banks.insert(bank_id, new_bank);
        self.bank_increment_id += 1;
        bank_id
    }

    pub fn get_bank(&self, bank_id: ID) -> Option<&Bank> {
        match self.banks.get_key_value(&bank_id) {
            Some((_id, bank)) => Some(bank),
            None => None
        }
    }

    pub fn get_bank_mut(&mut self, bank_id: ID) -> Option<&mut Bank> {
        match self.banks.get_mut(&bank_id) {
            Some(bank) => Some(bank),
            None => None
        }
    }
    
}
