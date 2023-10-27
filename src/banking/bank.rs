use std::collections::HashMap;
use self::errors::{BankError, TransactionError};

use super::consumer::{Account, ATMCard, Person};
use super::types::ID;

// * ERRORS

pub mod errors {
    pub enum TransactionError {
        NotEnoughBalance,
    }
    
    pub enum BankError {
        InvalidAccount,
        Transaction(TransactionError)
    }
}
// * ERRORS


pub enum BankTransaction {
    InternalTransfer {
        from: ID,
        to: ID,
        amount: u64
    },
    ExternalTransfer {
        from: ID,
        to: ID,
        target_bank: ID,
        amount: u64
    },
    Withdraw {
        account_id: ID,
        amount: u64
    },
    Deposit {
        account_id: ID,
        amount: u64
    }
}

pub struct TransferFee {
    pub internal: u64,
    pub external: u64
}

pub struct Bank {
    id: ID,
    accounts: HashMap<ID, Account>,
    cards: HashMap<ID, ATMCard>,
    transfer_fee: TransferFee,
    accounts_id_increment: u64,
    cards_id_increment: u64
}

impl Bank {
    pub fn new(id: ID, transfer_fee: TransferFee) -> Bank {
        Bank {
            id,
            accounts: HashMap::new(),
            cards: HashMap::new(),
            transfer_fee: transfer_fee,
            accounts_id_increment: 0,
            cards_id_increment: 0
        }
    }
    
    pub fn id(&self) -> ID {
        self.id
    }

    pub fn new_account(&mut self, person: Person) -> ID {
        let account_id = self.accounts_id_increment;
        self.accounts_id_increment += 1;

        let new_acc: Account = Account::new(person, account_id);
        self.accounts.insert(account_id, new_acc);

        account_id
    }

    pub fn get_account(&mut self, account_id: ID) -> Option<&mut Account> {
        match self.accounts.get_mut(&account_id) {
            Some(acc) => Some(acc),
            None => None
        }
    }

    pub fn is_account_exist(&self, account_id: ID) -> bool {
        match self.accounts.get_key_value(&account_id) {
            Some((_,_)) => true,
            None => false
        }
    }


    pub fn process_transaction(&mut self, transaction: BankTransaction) -> Result<(), BankError> {
        match transaction {
            BankTransaction::InternalTransfer { from, to, amount } => self.transfer_from_to(from, to, amount),
            _ => Ok(())
        }
    }

    fn transfer_from_to(&mut self, from: ID, to: ID, amount: u64) -> Result<(), BankError> {
        if !self.is_account_exist(from) | !self.is_account_exist(to) {
            return Err(BankError::InvalidAccount)
        }

        let transfer_fee = self.transfer_fee.internal;
        let total_charge: i64 = (transfer_fee + amount) as i64;

        // * Charge source for transfer and amount
        let source = self.get_account(from).unwrap();
        if source.get_balance() < total_charge { 
            println!("Failed transfer from accounts {from} to {to} with amount {amount} : Not Enough Balance");
            return Err(BankError::Transaction(TransactionError::NotEnoughBalance))
        }
        source.subtract(amount + transfer_fee);
        
        // * Transfer amount to target
        let target = self.get_account(to).unwrap();
        target.add(amount);

        Ok(())
        
    }
    
    
}