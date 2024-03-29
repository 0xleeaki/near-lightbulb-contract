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
