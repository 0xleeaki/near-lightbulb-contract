use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::U128;
use near_sdk::{
    env, ext_contract, near_bindgen, AccountId, Balance, BorshStorageKey, Promise,
};

mod account;
mod errors;
mod token_receiver;
mod utils;

use crate::account::Account;
use crate::errors::{ERR_ACC_NOT_REGISTERED, ERR_ILLEGAL_FEE, ERR_MIN_NEAR, ERR_OWNER};
use crate::utils::{FEE_DIVISOR, GAS_FOR_FT_TRANSFER, PRIZE_AMOUNT};

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
    owner_id: AccountId,
    accepted_token: AccountId,
    fee: u128,
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
    /* ========== INITTIALIZE =============== */
    #[init]
    pub fn new(accepted_token: AccountId, status: bool) -> Self {
        let this = Self {
            owner_id: env::predecessor_account_id(),
            accepted_token: accepted_token.clone().into(),
            fee: 0,
            status: status,
            accounts: LookupMap::new(StorageKey::Accounts),
        };
        this
    }

    /* ========== ASSERT FUNCTIONS ========== */

    pub(crate) fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "{}",
            ERR_OWNER
        );
    }

    /* ========== RESTRICTED FUNCTIONS ========== */

    pub fn set_owner(&mut self, owner_id: AccountId) {
        self.assert_owner();
        self.owner_id = owner_id;
    }

    pub fn get_owner(&self) -> AccountId {
        self.owner_id.clone()
    }

    pub fn set_fee(&mut self, fee: u128) {
        self.assert_owner();
        assert!(fee <= FEE_DIVISOR, "{}", ERR_ILLEGAL_FEE);
        self.fee = fee;
    }

    /* ========== PUBLIC FUNCTIONS ========== */

    pub(crate) fn internal_save_account(&mut self, account: Account) {
        self.accounts.insert(&account.account_id, &account);
    }

    pub(crate) fn unwrap_account(&self, account_id: &AccountId) -> Account {
        self.get_account(account_id).expect(ERR_ACC_NOT_REGISTERED)
    }

    pub fn get_account(&self, account_id: &AccountId) -> Option<Account> {
        self.accounts.get(account_id)
    }

    pub fn exits_account(&self, account_id: &AccountId) -> bool {
        self.accounts.contains_key(account_id)
    }

    #[payable]
    pub fn toggle(&mut self) {
        if self.status {
            let amount = if self.fee != 0 {
                PRIZE_AMOUNT * self.fee / (FEE_DIVISOR)
            } else {
                PRIZE_AMOUNT
            };
            Promise::new(env::predecessor_account_id()).transfer(amount);
        } else {
            let amount = env::attached_deposit();
            assert!(amount > PRIZE_AMOUNT, "{}", ERR_MIN_NEAR);
        }
        self.status = !self.status;
    }

    pub(crate) fn deposit(&mut self, sender_id: &AccountId, amount: Balance) {
        let is_exits = self.exits_account(sender_id);
        if is_exits {
            let mut account = self.unwrap_account(sender_id);
            account.deposit(amount);
            self.internal_save_account(account);
        } else {
            let new_account = Account {
                account_id: sender_id.clone(),
                amount: amount,
            };
            self.accounts.insert(&sender_id, &new_account);
        }
    }

    pub fn withdraw(&mut self, amount: U128) {
        let mut account = self.unwrap_account(&env::predecessor_account_id());
        account.withdraw(amount.into());
        self.internal_save_account(account);
        ext_ft::ft_transfer(
            env::predecessor_account_id(),
            amount.into(),
            None,
            self.accepted_token.clone(),
            1,
            GAS_FOR_FT_TRANSFER,
        );
    }

    /* ========== VIEWS FUNCTIONS ========== */

    pub fn get_status(&self) -> bool {
        self.status
    }
}
