use std::collections::HashMap;
use super::consumer::{Account, ATMCard, Person};
use super::types::ID;

pub enum TransactionError {
    NotEnoughBalance,
}

pub enum BankError {
    InvalidAccount,
    Transaction(TransactionError)
}

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

    pub fn new_account(&mut self, person: Person) -> &mut Account {
        let account_id = self.accounts_id_increment;
        self.accounts_id_increment += 1;

        let mut new_acc: Account = Account::new(person, account_id);
        self.accounts.insert(account_id, new_acc);

        self.accounts.get_mut(&account_id).unwrap()
    }

    pub fn get_account(&mut self, account_id: ID) -> Option<&mut Account> {
        match self.accounts.get_mut(&account_id) {
            Some(acc) => Some(acc),
            None => None
        }
    }

    pub fn account_exist(&self, account_id: ID) -> bool {
        match self.accounts.get_key_value(&account_id) {
            Some((_,_)) => true,
            None => false
        }
    }


    pub fn process_transaction(&mut self, transaction: BankTransaction) -> Result<(), BankError> {
        match transaction {
            BankTransaction::InternalTransfer { from, to, amount } => self.internal_transfer(from, to, amount),
            _ => Ok(())
        }
    }

    fn internal_transfer(&mut self, from: ID, to: ID, amount: u64) -> Result<(), BankError> {
        if !self.account_exist(from) | !self.account_exist(to) {
            return Err(BankError::InvalidAccount)
        }

        if let Some(source) = self.get_account(from){
            if source.get_balance() < (amount as i64) { return Err(BankError::Transaction(TransactionError::NotEnoughBalance))}
            source.subtract(amount);
        }
        
        if let Some(target) = self.get_account(to) {
            target.add(amount);
        }
        Ok(())
        
    }
    
    
}