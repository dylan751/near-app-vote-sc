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
### Users
1. Create a User:

```
near call btc-app-vote.duongnh.testnet create_user '{"name": "Zuong", "role": "Admin", "email": "muoi07052001@gmail.com", "near_account_id": "duongnh.testnet"}' --deposit 0.1 --accountId duongnh.testnet
```

2. View list of Users (with pagination) of the Contract: (`from_index`: integer, `limit`: integer)

```
near view btc-app-vote.duongnh.testnet get_all_users '{"from_index": 0, "limit": 10}'
```

3. View 1 User by user_id
```
near view btc-app-vote.duongnh.testnet get_user_by_id '{"user_id": 0}'
```

---
### Criterias
1. Create a Criteria:

```
near call btc-app-vote.duongnh.testnet create_criteria '{"user_id": 0, "description": "The most handsome employee"}' --deposit 0.1 --accountId duongnh.testnet
```

2. View list of Criterias (with pagination) of the Contract: (`from_index`: integer, `limit`: integer)

```
near view btc-app-vote.duongnh.testnet get_all_criterias '{"from_index": 0, "limit": 10}'
```

3. View 1 Criteria by criteria_id
```
near view btc-app-vote.duongnh.testnet get_criteria_by_id '{"criteria_id": 0}'
```

---
### Votes
1. Create a Vote:

```
near call btc-app-vote.duongnh.testnet create_vote '{"criteria_id": 0, "user_id": 0, "month": 6, "start_at": 0, "end_at": 0}' --deposit 0.1 --accountId duongnh.testnet
```

2. View list of Vote (with pagination) of the Contract: (`from_index`: integer, `limit`: integer)

```
near view btc-app-vote.duongnh.testnet get_all_votes '{"from_index": 0, "limit": 10}'
```

3. View 1 Vote by vote_id
```
near view btc-app-vote.duongnh.testnet get_vote_by_id '{"vote_id": 0}'
```


---
### Results
1. Create a Result:

```
near call btc-app-vote.duongnh.testnet create_result '{"month": 6, "user_id": 0}' --deposit 0.1 --accountId duongnh.testnet
```

2. View list of Results (with pagination) of the Contract: (`from_index`: integer, `limit`: integer)

```
near view btc-app-vote.duongnh.testnet get_all_results '{"from_index": 0, "limit": 10}'
```

3. View 1 Result by result_id
```
near view btc-app-vote.duongnh.testnet get_result_by_id '{"result_id": 0}'
```