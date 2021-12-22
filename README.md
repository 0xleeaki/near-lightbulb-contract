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

7. Build the contract

   `RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release`

8. Create sub subaccount

   `NEAR_ENV=testnet near create-account YOUR_CONTRACT.YOUR_ACCOUNT.testnet --masterAccount YOUR_ACCOUNT.testnet`

9. Deploy the contract

   `NEAR_ENV=testnet near deploy --wasmFile target/wasm32-unknown-unknown/release/lightbulb-contract.wasm --accountId YOUR_CONTRACT.YOUR_ACCOUNT.testnet`

10. Read contract

    `NEAR_ENV=testnet near view YOUR_CONTRACT.YOUR_ACCOUNT.testnet get_status`

11. Write contract

    `NEAR_ENV=testnet near call YOUR_CONTRACT.YOUR_ACCOUNT.testnet toggle --accountId YOUR_ACCOUNT.testnet`

**Get more info at:**

- [Rust Smart Contract Quick Start](https://docs.near.org/docs/develop/contracts/rust/intro)
- [Rust SDK Book](https://www.near-sdk.io/)


**DEPLOY DOLA**

- NEAR_ENV=testnet near create-account dola.leeaki.testnet --masterAccount leeaki.testnet

- NEAR_ENV=testnet near deploy --wasmFile target/wasm32-unknown-unknown/release/dola_token.wasm --accountId dola.leeaki.testnet

- NEAR_ENV=testnet near call dola.leeaki.testnet new '{"owner_id": "dola.leeaki.testnet", "total_supply": "1000000000000000", "metadata": { "spec": "ft-1.0.0", "name": "DOLA Token", "symbol": "DOLA", "decimals": 8 }}' --accountId dola.leeaki.testnet

- NEAR_ENV=testnet near view dola.leeaki.testnet ft_metadata

**CHECK BALANCE**

- NEAR_ENV=testnet near view dola.leeaki.testnet ft_balance_of '{"account_id": "dola.leeaki.testnet"}'

- NEAR_ENV=testnet near view dola.leeaki.testnet ft_balance_of '{"account_id": "leeaki.testnet"}'

- NEAR_ENV=testnet near view dola.leeaki.testnet ft_balance_of '{"account_id": "lightbulb.leeaki.testnet"}'

**CREATE STORAGE**

- NEAR_ENV=testnet near call dola.leeaki.testnet storage_deposit '' --accountId lightbulb.leeaki.testnet --amount 0.00125

**TRANSFER**

- NEAR_ENV=testnet near call dola.leeaki.testnet ft_transfer '{"receiver_id": "leeaki.testnet", "amount": "200000000"}' --accountId lightbulb.leeaki.testnet --amount 0.000000000000000000000001


**DEPLOY CONTRACT**

- NEAR_ENV=testnet near delete lightbulb.leeaki.testnet leeaki.testnet

- NEAR_ENV=testnet near create-account lightbulb.leeaki.testnet --masterAccount leeaki.testnet

- NEAR_ENV=testnet near deploy --wasmFile target/wasm32-unknown-unknown/release/lightbulb_contract.wasm --initFunction new --initArgs '{"accepted_token": "dola.leeaki.testnet", "status": false}' --accountId lightbulb.leeaki.testnet

- NEAR_ENV=testnet near call dola.leeaki.testnet ft_transfer_call '{"receiver_id": "lightbulb.leeaki.testnet", "amount": "100000000", "msg": ""}' --accountId leeaki.testnet --amount 0.000000000000000000000001 --gas 300000000000000

- NEAR_ENV=testnet near call lightbulb.leeaki.testnet withdraw '{"amount": "100000000"}' --accountId leeaki.testnet

- NEAR_ENV=testnet near view lightbulb.leeaki.testnet get_account '{"account_id": "leeaki.testnet"}'