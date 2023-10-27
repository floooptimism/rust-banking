use banking::{bank_association::BankAssociation, consumer::Person, bank::BankTransaction};

use crate::banking::bank::TransferFee;

mod banking;


fn main() {
    let mut bank_ass : BankAssociation = BankAssociation::new();
    let new_bank_id = bank_ass.create_bank(TransferFee {
        internal: 0,
        external: 25
    });
    let bank = bank_ass.get_bank_mut(new_bank_id).unwrap();


    let jenny = Person::new("Jenny".to_string(), "Pearsons".to_string());
    let henry = Person::new("Henry".to_string(), "Waters".to_string());

    let jenny_acc = bank.new_account(jenny);
    let jenny_id = jenny_acc.id();
    jenny_acc.add(1000);

    let henry_acc = bank.new_account(henry);
    let henry_id = henry_acc.id();

    
    let _ = bank.process_transaction(BankTransaction::InternalTransfer { from: jenny_id, to: henry_id, amount: 500 });

    let henry_balance = bank.get_account(henry_id).unwrap().get_balance();

    assert_eq!(henry_balance, 500);

}