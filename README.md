# LightBulb Contract

## Getting started

To get started with this template:

1. Click the "Use this template" button to create a new repo based on this template
2. Update line 2 of `Cargo.toml` with your project name
3. Update line 4 of `Cargo.toml` with your project author names
4. Set up the [prerequisites](https://github.com/near/near-sdk-rs#pre-requisites)
5. Begin writing your smart contract in `src/lib.rs`
6. Test the contract 

    `cargo test -- --nocapture`

8. Build the contract

    `RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release`

9. Create sub subaccount

    `NEAR_ENV=testnet near create-account YOUR_CONTRACT.YOUR_ACCOUNT.testnet --masterAccount YOUR_ACCOUNT.testnet`

10. Deploy the contract

    `NEAR_ENV=testnet near deploy --wasmFile target/wasm32-unknown-unknown/release/lightbulb-contract.wasm --accountId YOUR_CONTRACT.YOUR_ACCOUNT.testnet`

11. Read contract

    `NEAR_ENV=testnet near view YOUR_CONTRACT.YOUR_ACCOUNT.testnet get_status`

12. Write contract

    `NEAR_ENV=testnet near call YOUR_CONTRACT.YOUR_ACCOUNT.testnet toggle --accountId YOUR_ACCOUNT.testnet`

**Get more info at:**

* [Rust Smart Contract Quick Start](https://docs.near.org/docs/develop/contracts/rust/intro)
* [Rust SDK Book](https://www.near-sdk.io/)


NEAR_ENV=testnet near create-account dola.leeaki.testnet --masterAccount leeaki.testnet

NEAR_ENV=testnet near deploy --wasmFile target/wasm32-unknown-unknown/release/dola_token.wasm --accountId dola.leeaki.testnet

near call dola.leeaki.testnet new '{"owner_id": "dola.leeaki.testnet", "total_supply": "1000000000000000", "metadata": { "spec": "ft-1.0.0", "name": "DOLA Token", "symbol": "DOLA", "decimals": 8 }}' --accountId dola.leeaki.testnet

near view dola.leeaki.testnet ft_metadata

near view dola.leeaki.testnet ft_balance_of '{"account_id": "leeaki.testnet"}'

near view dola.leeaki.testnet ft_balance_of '{"account_id": "dola.leeaki.testnet"}'

near call dola.leeaki.testnet storage_deposit '' --accountId leeaki.testnet --amount 0.00125

near call dola.leeaki.testnet ft_transfer '{"receiver_id": "leeaki.testnet", "amount": "10"}' --accountId dola.leeaki.testnet --amount 0.000000000000000000000001

near call dola.leeaki.testnet ft_transfer '{"receiver_id": "leeaki.testnet", "amount": "999999990"}' --accountId dola.leeaki.testnet --amount 0.000000000000000000000001

near call dola.leeaki.testnet ft_transfer '{"receiver_id": "lightbulb.leeaki.testnet", "amount": "1000000000"}' --accountId dola.leeaki.testnet --amount 0.000000000000000000000001



NEAR_ENV=testnet near deploy --wasmFile target/wasm32-unknown-unknown/release/lightbulb_contract.wasm --accountId lightbulb.leeaki.testnet

near call lightbulb.leeaki.testnet new '{"accepted_token": "dola.leeaki.testnet", "status": false}' --accountId lightbulb.leeaki.testnet

NEAR_ENV=testnet near view lightbulb.leeaki.testnet get_status

NEAR_ENV=testnet near call lightbulb.leeaki.testnet toggle --accountId leeaki.testnet

NEAR_ENV=testnet near call lightbulb.leeaki.testnet toggle --accountId leeaki.testnet --amount 6

NEAR_ENV=testnet near call dola.leeaki.testnet storage_deposit '' --accountId lightbulb.leeaki.testnet --amount 0.00125

NEAR_ENV=testnet near view dola.leeaki.testnet ft_balance_of '{"account_id": "lightbulb.leeaki.testnet"}'

NEAR_ENV=testnet near view dola.leeaki.testnet ft_balance_of '{"account_id": "leeaki.testnet"}'

NEAR_ENV=testnet near call lightbulb.leeaki.testnet deposit --accountId leeaki.testnet



NEAR_ENV=testnet near delete lightbulb.leeaki.testnet leeaki.testnet

NEAR_ENV=testnet near create-account lightbulb.leeaki.testnet --masterAccount leeaki.testnet

NEAR_ENV=testnet near deploy --wasmFile target/wasm32-unknown-unknown/release/lightbulb_contract.wasm --initFunction new --initArgs '{"accepted_token": "dola.leeaki.testnet", "status": false}' --accountId lightbulb.leeaki.testnet

NEAR_ENV=testnet near call dola.leeaki.testnet ft_transfer '{"receiver_id": "lightbulb.leeaki.testnet", "amount": "100000000"}' --accountId leeaki.testnet --amount 0.000000000000000000000001

NEAR_ENV=testnet near view lightbulb.leeaki.testnet internal_get_account '{"account_id": "leeaki.testnet"}'