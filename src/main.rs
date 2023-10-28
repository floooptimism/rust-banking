use banking::{bank_association::BankAssociation, consumer::Person};

use crate::banking::bank::TransferFee;

mod banking;


fn main() {
    let mut bank_association_1 : BankAssociation = BankAssociation::new();
    let new_bank_id = bank_association_1.create_bank(TransferFee {
        internal: 0,
        external: 25
    });
    let mut bank = bank_association_1.get_bank_facade(new_bank_id).unwrap();

    let jenny = Person::new("Jenny".to_string(), "Pearsons".to_string());
    let henry = Person::new("Henry".to_string(), "Waters".to_string());
    
    let jenny_id = bank.create_account(jenny, "jenny".to_string(), "jennypass".to_string());
    let henry_id = bank.create_account(henry, "henry".to_string(), "henrypass".to_string());

    bank.add_to_account(jenny_id, 1000); // * Add 1000 to jenny
    bank.add_to_account(henry_id, 100); // * Add 100 to henry
    
    let _ = bank.transfer_to(henry_id, jenny_id, 100);
    let _ = bank.transfer_to(jenny_id, henry_id, 500);

    let henry_balance = bank.get_balance(henry_id);

    assert_eq!(henry_balance, 500);

}