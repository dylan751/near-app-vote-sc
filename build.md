# How to build and test this contract

1. Create nft_contract_id -> deploy out/nft-contract.wasm to nft_contract_id (`nearhub-nft.duongnh.testnet`)

```
- ./build.sh
- near create-account btc-app-vote.duongnh.testnet --masterAccount duongnh.testnet --initialBalance 20
- near deploy --wasmFile out/app-vote-contract.wasm --accountId btc-app-vote.duongnh.testnet --initFunction new --initArgs '{"owner_id": "duongnh.testnet"}'
```

---

### Init contract

```
near call btc-app-vote.duongnh.testnet new '{"owner_id": "duongnh.testnet"}' --accountId duongnh.testnet
```

---
