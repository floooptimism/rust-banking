use std::collections::HashMap;
use crate::banking::bank_facade::BankFacade;
use crate::banking::types::BankID;


use super::bank::{Bank, TransferFee};

pub struct BankAssociation {
    banks: HashMap<BankID, Bank>,
    bank_increment_id: BankID,
}

impl BankAssociation {

    pub fn new() -> BankAssociation {
        BankAssociation {
            banks: HashMap::new(),
            bank_increment_id: 0,
        }
    }

    pub fn create_bank(&mut self, transfer_fee: TransferFee) -> BankID {
        let bank_id = self.bank_increment_id;
        let new_bank = Bank::new(bank_id, transfer_fee);
        self.banks.insert(bank_id, new_bank);
        self.bank_increment_id += 1;
        bank_id
    }

    pub fn get_bank(&self, bank_id: BankID) -> Option<&Bank> {
        match self.banks.get_key_value(&bank_id) {
            Some((_id, bank)) => Some(bank),
            None => None
        }
    }

    pub fn get_bank_facade(&mut self, bank_id: BankID) -> Option<BankFacade> {
        match self.banks.get_mut(&bank_id) {
            Some(bank) => Some(BankFacade::new(bank)),
            None => None
        }
    }

    pub fn get_bank_list(&self) -> Vec<BankID> {
        let mut bank_list: Vec<BankID> = Vec::new();

        for bank_key in self.banks.keys(){
            bank_list.push(*bank_key);
        }

        bank_list
    }
}
