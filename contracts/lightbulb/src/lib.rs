use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, Promise};

// 5 Ⓝ in yoctoNEAR
const PRIZE_AMOUNT: u128 = 5_000_000_000_000_000_000_000_000;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    status: bool,
}

#[near_bindgen]
impl Contract {
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

    pub fn get_status(&self) -> bool {
        self.status
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::json_types::ValidAccountId;
    use near_sdk::serde::export::TryFrom;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;

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
        let mut contract = Contract { status: false };
        contract.toggle();
        let status: bool = contract.get_status();
        println!("Status after toggle: {}", status);
        assert_eq!(true, status);
    }
}
