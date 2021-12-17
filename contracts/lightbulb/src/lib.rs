use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    status: bool,
    pub messages: HashMap<AccountId, String>,
}

#[near_bindgen]
impl Contract {
    pub fn toggle(&mut self) {
        self.status = !self.status;
    }

    pub fn toggle_with_message(&mut self, message: String) {
        let account_id = env::signer_account_id();
        self.status = !self.status;
        self.messages.insert(account_id, message);
    }

    pub fn clear_all_message(&mut self) {
        self.messages.clear();
    }

    pub fn get_all_messages(self) -> HashMap<AccountId, String> {
        self.messages
    }

    pub fn get_status(&self) -> bool {
        self.status
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::serde_json::Map;
    use near_sdk::testing_env;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::json_types::ValidAccountId;
    use near_sdk::serde::export::TryFrom;

    // simple helper function to take a string literal and return a ValidAccountId
    fn to_valid_account(account: &str) -> ValidAccountId {
        ValidAccountId::try_from(account.to_string()).expect("Invalid account")
    }

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: ValidAccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // mark individual unit tests with #[test] for them to be registered and fired
    #[test]
    fn toggle() {
        let context = get_context(to_valid_account("leeaki.near"));
        testing_env!(context.build());
        let mut contract = Contract {status: false, messages: HashMap::new()};
        contract.toggle();
        let status: bool = contract.get_status();
        println!("Status after toggle: {}", status);
        assert_eq!(true, status);
    }
}
