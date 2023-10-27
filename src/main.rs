use banking::{bank_association::BankAssociation, consumer::Person, bank::BankTransaction};

use crate::banking::bank::TransferFee;

mod banking;


fn main() {
    let mut bank_association_1 : BankAssociation = BankAssociation::new();
    let new_bank_id = bank_association_1.create_bank(TransferFee {
        internal: 0,
        external: 25
    });
    let bank = bank_association_1.get_bank_mut(new_bank_id).unwrap();


    let jenny = Person::new("Jenny".to_string(), "Pearsons".to_string());
    let henry = Person::new("Henry".to_string(), "Waters".to_string());
    
    let jenny_id = bank.new_account(jenny);
    let henry_id = bank.new_account(henry);

    bank.get_account(jenny_id).unwrap().add(1000); // * Add 1000 to jenny
    bank.get_account(henry_id).unwrap().add(100); // * Add 100 to henry
    
    let _ = bank.process_transaction(BankTransaction::InternalTransfer { from: henry_id, to: jenny_id, amount: 100 }); 
    let _ = bank.process_transaction(BankTransaction::InternalTransfer { from: jenny_id, to: henry_id, amount: 500 }); 

    let henry_balance = bank.get_account(henry_id).unwrap().get_balance(); 

    assert_eq!(henry_balance, 500);

}