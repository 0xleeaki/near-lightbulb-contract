use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::U128;
use near_sdk::{
    env, log, ext_contract, near_bindgen, AccountId, Balance, BorshStorageKey, Gas, Promise,
};

mod account;
mod errors;
mod token_receiver;

use crate::account::Account;

const PRIZE_AMOUNT: u128 = 5_000_000_000_000_000_000_000_000; // 5 NEAR
const GAS_FOR_FT_TRANSFER: Gas = Gas(5_000_000_000_000);

#[ext_contract(ext_ft)]
trait FungibleToken {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
}

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKey {
    Accounts,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    accepted_token: AccountId,
    status: bool,
    accounts: LookupMap<AccountId, Account>,
}

impl Default for Contract {
    fn default() -> Self {
        env::panic(b"Contract is not initialized");
    }
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(accepted_token: AccountId, status: bool) -> Self {
        let this = Self {
            status: status,
            accepted_token: accepted_token.clone().into(),
            accounts: LookupMap::new(StorageKey::Accounts),
        };
        this
    }

    #[payable]
    pub fn toggle(&mut self) {
        if self.status {
            Promise::new(env::predecessor_account_id()).transfer(PRIZE_AMOUNT);
        } else {
            let amount = env::attached_deposit();
            assert!(
                amount > PRIZE_AMOUNT,
                "Required min 5 NEAR to turn on the light bulb"
            );
        }
        self.status = !self.status;
    }

    pub fn internal_get_account(&self, account_id: &AccountId) -> Option<Account> {
        self.accounts.get(account_id)
    }

    pub fn internal_unwrap_account(&self, account_id: &AccountId) -> Account {
        self.internal_get_account(account_id)
            .expect(errors::ERR_ACC_NOT_REGISTERED)
    }

    pub fn internal_exits_account(&self, account_id: &AccountId) -> bool {
        self.accounts.contains_key(account_id)
    }

    pub fn deposit(&mut self, sender_id: &AccountId, amount: Balance) {
        log!("Deposit 2...");
        let mut account = self.internal_unwrap_account(sender_id);
        let is_exits = self.internal_exits_account(sender_id);
        log!("is_exits {}", is_exits);
        if is_exits {
            account.deposit(amount);
        } else {
            let new_account = Account {
                account_id: sender_id.clone(),
                amount: amount
            };
            self.accounts.insert(&sender_id, &new_account);
        }
    }

    pub fn withdraw(&self, sender_id: &AccountId, amount: Balance) {
        let mut account = self.internal_unwrap_account(sender_id);
        account.withdraw(amount);
        ext_ft::ft_transfer(
            env::predecessor_account_id(),
            amount.into(),
            None,
            self.accepted_token.clone(),
            1,
            GAS_FOR_FT_TRANSFER,
        );
    }

    pub fn get_status(&self) -> bool {
        self.status
    }
}
