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
near call btc-app-vote.duongnh.testnet create_user '{"name": "Zuong", "role": "Admin", "email": "muoi07052001@gmail.com", "blockchain_type": "Near", "wallet_address": "duongnh.testnet"}' --deposit 0.1 --accountId duongnh.testnet
```

2. View list of Users (with pagination) of the Contract: (`from_index`: integer, `limit`: integer)

```
near view btc-app-vote.duongnh.testnet get_all_users '{"from_index": 0, "limit": 10}'
```

3. View 1 User by user_id

```
near view btc-app-vote.duongnh.testnet get_user_by_id '{"user_id": 0}'
```

4. Update User information

```
near call btc-app-vote.duongnh.testnet update_user '{"user_id": 0, "name": "Hai", "role": "Admin", "email": "duong07052001@gmail.com", "blockchain_type": "Near", "wallet_address": "duongnh.testnet"}' --accountId duongnh.testnet
```

5. Delete a User

```
near call btc-app-vote.duongnh.testnet delete_user '{"user_id": 0}' --accountId duongnh.testnet
```

---

### Criterias

1. Create a Criteria:

```
near call btc-app-vote.duongnh.testnet create_criteria '{"created_by": 0, "description": "The most handsome employee"}' --deposit 0.1 --accountId duongnh.testnet
```

2. View list of Criterias (with pagination) of the Contract: (`from_index`: integer, `limit`: integer)

```
near view btc-app-vote.duongnh.testnet get_all_criterias '{"from_index": 0, "limit": 10}'
```

3. View 1 Criteria by criteria_id

```
near view btc-app-vote.duongnh.testnet get_criteria_by_id '{"criteria_id": 0}'
```

4. Update Criteria information

```
near call btc-app-vote.duongnh.testnet update_criteria '{"criteria_id": 0, "description": "Updated description"}' --accountId duongnh.testnet
```

5. Delete a Criteria

```
near call btc-app-vote.duongnh.testnet delete_criteria '{"criteria_id": 0}' --accountId duongnh.testnet
```

---

### Polls

1. Create a Poll:

```
near call btc-app-vote.duongnh.testnet create_poll '{"criteria_ids": [0, 1], "created_by": 0, "title": "Test poll", "description": "Test poll description",  "start_at": 0, "end_at": 0}' --deposit 0.1 --accountId duongnh.testnet
```

2. View list of Poll (with pagination) of the Contract: (`from_index`: integer, `limit`: integer)

```
near view btc-app-vote.duongnh.testnet get_all_polls '{"from_index": 0, "limit": 10}'
```

3. View 1 Poll by poll_id

```
near view btc-app-vote.duongnh.testnet get_poll_by_id '{"poll_id": 0}'
```

4. Update Poll information

```
near call btc-app-vote.duongnh.testnet update_poll '{"poll_id": 0, "title": "Updated title", "description": "Updated description", "start_at": 0, "end_at": 0}' --accountId duongnh.testnet
```

5. Delete a Poll

```
near call btc-app-vote.duongnh.testnet delete_poll '{"poll_id": 0}' --accountId duongnh.testnet
```

---

### Poll Options

1. Create a Poll Option:

```
near call btc-app-vote.duongnh.testnet create_poll_option '{"poll_id": 0, "created_by": 0, "title": "Test Option", "description": "Test Option description", "user_ids": [0, 1]}' --deposit 0.1 --accountId duongnh.testnet
```

2. View list of Poll Option (with pagination) of the Contract: (`from_index`: integer, `limit`: integer)

```
near view btc-app-vote.duongnh.testnet get_all_poll_options '{"from_index": 0, "limit": 10}'
```

3. View 1 Poll Option by poll_option_id

```
near view btc-app-vote.duongnh.testnet get_poll_option_by_id '{"poll_option_id": 0}'
```

5. Delete a Poll Option

```
near call btc-app-vote.duongnh.testnet delete_poll_option '{"poll_option_id": 0}' --accountId duongnh.testnet
```

---

### Results

1. Create a Result:

```
near call btc-app-vote.duongnh.testnet create_result '{"criteria_id": 0, "poll_id": 0, "user_id": 0}' --deposit 0.1 --accountId duongnh.testnet
```

2. View list of Results (with pagination) of the Contract: (`from_index`: integer, `limit`: integer)

```
near view btc-app-vote.duongnh.testnet get_all_results '{"from_index": 0, "limit": 10}'
```

3. View 1 Result by result_id

```
near view btc-app-vote.duongnh.testnet get_result_by_id '{"result_id": 0}'
```

4. Update Result information

```
near call btc-app-vote.duongnh.testnet update_result '{"result_id": 0}' --accountId duongnh.testnet
```

5. Delete a Result

```
near call btc-app-vote.duongnh.testnet delete_result '{"result_id": 0}' --accountId duongnh.testnet
```
