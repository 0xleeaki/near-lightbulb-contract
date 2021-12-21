use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, self};
use near_sdk::{AccountId, Balance};

use crate::errors;

#[serde(crate = "near_sdk::serde")]
#[derive(BorshSerialize, BorshDeserialize, Serialize)]
pub struct Account {
    pub account_id: AccountId,
    pub amount: Balance,
}

impl Account {
    pub fn new(account_id: &AccountId, amount: Balance) -> Self {
        Account {
            amount: amount,
            account_id: account_id.clone(),
        }
    }

    pub fn get_balance(&self) -> Option<Balance> {
        Some(self.amount)
    }

    pub(crate) fn deposit(&mut self, amount: Balance) {
        self.amount = self.amount + amount;
    }

    pub(crate) fn withdraw(&mut self, amount: Balance) {
        assert!(
            self.amount >= amount,
            "{}",
            errors::ERR_INSUFFICIENT_BALANCE
        );
        self.amount = self.amount - amount;
    }
}
