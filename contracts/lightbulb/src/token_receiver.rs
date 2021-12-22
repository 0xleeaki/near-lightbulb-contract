use near_sdk::json_types::U128;
use near_sdk::log;
use near_sdk::AccountId;
use near_sdk::PromiseOrValue;

use crate::*;

pub trait FungibleTokenReceiver {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128>;
}

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        log!("Account @{} transfer {}", sender_id, msg);
        if msg.is_empty() {
            log!("Deposit...");
            self.deposit(&sender_id, amount.into())
        }
        log!("Return...");
        PromiseOrValue::Value(U128(0))
    }
}
