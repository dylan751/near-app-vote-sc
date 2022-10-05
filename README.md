# App Vote Project

## Useful Links

1. [App Vote Domain](https://app-vote-git-developphase2-app-vote-front-end.vercel.app/)
2. [App Vote Repository](https://github.com/btc-studio/app-vote-smart-contract)

## What is App Vote?

A NEAR smart contract for the Near Protocol based voting app.

- Allow users/organizations to create voting Poll for anything
- Allow users/organizations to create criterias for each Poll
- Allow users/organizations to create answer options
- Allow users/organizations to vote for their Polls

## Built with

- [NEAR](https://near.org/)
- [Rust Programming Language](https://www.rust-lang.org/)

## Prerequisites

### Rust toolchain

- Install Rust toolchain:
  - Install rustup: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
  - Configure the current shell: `source $HOME/.cargo/env`
  - Add the wasm target to the blockchain network: `rustup target add wasm32-unknown-unknown`

### A NEAR account

- Create a NEAR account: https://wallet.testnet.near.org/create

### NEAR command-line interface (near-cli)

- Install near-cli: `npm install -g near-cli`

## App Vote Repository

### Overview

app-vote-smart-contract </br>
├── Cargo.toml </br>
├── README.md </br>
├── build.sh </br>
├── neardev </br>
├ ├── dev-account </br>
├   └── dev-account.env </br>
├── notes </br>
├   ├── DAO.md </br>
├   ├── build-test-stable.md </br>
├   ├── build-test.md </br>
├   └── build.md </br>
├── out </br>
├   └── app-vote-contract.wasm </br>
├── scripts </br>
├   ├── create-data.sh </br>
├   ├── log-deposit.sh </br>
├   └── log-deposit.txt </br>
├── src </br>
├ ├── criterias.rs </br>
├ ├── custom_struct.rs </br>
├ ├── event.rs </br>
├ ├── is_user_votes.rs </br>
├ ├── lib.rs </br>
├ ├── poll_options.rs </br>
├ ├── polls.rs </br>
├ ├── results.rs </br>
├ ├── tests.rs </br>
├ ├── users.rs </br>
└ └── utils.rs </br>

### Directory Details

- **build.sh** - Contains the build script
- **neardev** - The folder contains dev account which the Smart Contract being deployed to
- **out** - Contains .wasm file to push to blockchain
- **scripts** - Contains scripts
- **src** - Contains all the Smart Contract logic for App Vote

## Interacting with the App Vote contracts locally

### Deploy to testnet for usable API

1. Install dependencies

```shell
cargo install
```

2. Compile code to .wasm file

```shell
./build
```

3. Deploy to NEAR testnet

```shell
near dev-deploy ./out/app-vote-contract.wasm
```

4. Use smart contract id in **neardev/dev-account** to call in near-cli

```
near call dev-1660616028365-78983722768651 <command> <arguments>
```

> For more information, see: `/notes/build-test.md`

## Network

development

## Contracts

|   CONTRACT   |        ADDRESS        |
| :----------: | :-------------------: |
| **App Vote** | app-vote.btcs.testnet |
