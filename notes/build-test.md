# How to build and test this contract

1. Create nft_contract_id -> deploy out/nft-contract.wasm to nft_contract_id (`nearhub-nft.duongnh.testnet`)

```
- ./build.sh
- near dev-deploy ./out/app-vote-contract.wasm
```

---

### Init contract

```
near call dev-1661499707374-98283221529225 new '{"owner_id": "duongnh.testnet"}' --accountId duongnh.testnet
```

---

### Users

1. Create a User:

```
near call dev-1661499707374-98283221529225 create_user '{"name": "Zuong", "role": "Admin", "email": "muoi07052001@gmail.com", "blockchain_type": "Near", "wallet_address": "duongnh.testnet"}' --deposit 0.1 --accountId duongnh.testnet
```

2. View total number of User in the Smart Contract
```
near view dev-1661499707374-98283221529225 user_total_supply
```

3. View list of Users (with pagination) of the Contract: (`from_index`: integer, `limit`: integer)

```
near view dev-1661499707374-98283221529225 get_all_users '{"from_index": 0, "limit": 10}'
```

4. View 1 User by user_id

```
near view dev-1661499707374-98283221529225 get_user_by_id '{"user_id": 1}'
```

5. View 1 User by wallet_address (Ex: Near Wallet)

```
near view dev-1661499707374-98283221529225 get_user_by_wallet_address '{"wallet_address": "duongnh.testnet"}'
```

6. Update User information

```
near call dev-1661499707374-98283221529225 update_user '{"user_id": 1, "name": "Hai", "role": "Admin", "email": "duong07052001@gmail.com", "blockchain_type": "Near", "wallet_address": "duongnh.testnet"}' --accountId duongnh.testnet
```

7. Delete a User

```
near call dev-1661499707374-98283221529225 delete_user '{"user_id": 1}' --accountId duongnh.testnet
```

---

### Criterias

1. Create a Criteria:

```
near call dev-1661499707374-98283221529225 create_criteria '{"created_by": 1, "descriptions": ["The most handsome employee", "The most creative employee"]}' --deposit 0.1 --accountId duongnh.testnet
```

2. View total number of Criteria in the Smart Contract
```
near view dev-1661499707374-98283221529225 criteria_total_supply
```

3. View list of Criterias (with pagination) of the Contract: (`from_index`: integer, `limit`: integer)

```
near view dev-1661499707374-98283221529225 get_all_criterias '{"from_index": 0, "limit": 10}'
```
4. View 1 Criteria by criteria_id

```
near view dev-1661499707374-98283221529225 get_criteria_by_id '{"criteria_id": 1}'
```

5. Update Criteria information

```
near call dev-1661499707374-98283221529225 update_criteria '{"criteria_id": 1, "description": "Updated description"}' --accountId duongnh.testnet
```

6. Delete a Criteria

```
near call dev-1661499707374-98283221529225 delete_criteria '{"criteria_id": 1}' --accountId duongnh.testnet
```

---

### Poll Options

1. Create a Poll Option:

```
near call dev-1661499707374-98283221529225 create_poll_option '{"created_by": 1, "title": "Test Option", "description": "Test Option description", "options": ["Zuong", "Manh"]}' --deposit 0.1 --accountId duongnh.testnet
```

2. View total number of Poll Option in the Smart Contract
```
near view dev-1661499707374-98283221529225 poll_option_total_supply
```

3. View list of Poll Option (with pagination) of the Contract: (`from_index`: integer, `limit`: integer)

```
near view dev-1661499707374-98283221529225 get_all_poll_options '{"from_index": 0, "limit": 10}'
```

4. View 1 Poll Option by poll_option_id

```
near view dev-1661499707374-98283221529225 get_poll_option_by_id '{"poll_option_id": 1}'
```

5. Update a Poll Option

```
near call dev-1661499707374-98283221529225 update_poll_option '{"poll_option_id": 1, "title": "Updated title", "description": "Updated description", options: ["Zuong", "Hieu"]}' --accountId duongnh.testnet
```

6. Delete a Poll Option

```
near call dev-1661499707374-98283221529225 delete_poll_option '{"poll_option_id": 1}' --accountId duongnh.testnet
```

---

### Polls

1. Create a Poll:

```
near call dev-1661499707374-98283221529225 create_poll '{"criteria_option_id_array": [{"criteria_id": 1, "poll_option_id": 1}, {"criteria_id": 2, "poll_option_id": 2}], "created_by": 1, "img_url": "", "title": "Test poll", "description": "Test poll description", "start_at": 0, "end_at": 0}' --deposit 0.1 --accountId duongnh.testnet
```

2. View total number of Poll in the Smart Contract
```
near view dev-1661499707374-98283221529225 poll_total_supply
```

3. View list of Poll (with pagination) of the Contract: (`from_index`: integer, `limit`: integer)

```
near view dev-1661499707374-98283221529225 get_all_polls '{"from_index": 0, "limit": 10}'
```

4. View 1 Poll by poll_id

```
near view dev-1661499707374-98283221529225 get_poll_by_id '{"poll_id": 1}'
```

5. Update Poll information

```
near call dev-1661499707374-98283221529225 update_poll '{"poll_id": 1, "img_url": "", "title": "Updated title", "description": "Updated description", "start_at": 0, "end_at": 0}' --accountId duongnh.testnet
```

6. Delete a Poll

```
near call dev-1661499707374-98283221529225 delete_poll '{"poll_id": 1}' --accountId duongnh.testnet
```

---
### Results
1. View list of Results (with pagination) of the Contract: (`from_index`: integer, `limit`: integer)

```
near view dev-1661499707374-98283221529225 get_all_results '{"from_index": 0, "limit": 10}'
```

2. View total number of Result in the Smart Contract
```
near view dev-1661499707374-98283221529225 result_total_supply
```

3. View list of Results of 1 Poll of the Contract

```
near view dev-1661499707374-98283221529225 get_all_results_by_poll_criteria_id '{"poll_id": 1, "criteria_id": 1}'

```

4. Update Result information

```
near call dev-1661499707374-98283221529225 vote '{"voted_user_id": 1, "poll_id": 1, "criteria_option_array": [{"criteria_id": 1,"option": "Hamburger"}, {"criteria_id": 2,"option": "Pepsi"}]}' --accountId duongnh.testnet --gas 300000000000000
```

5. Get the number of Users who voted for a poll
```
near view dev-1661499707374-98283221529225 num_users_vote_for_a_poll '{"poll_id": 1}'
```

5. Delete a Result

```
near call dev-1661499707374-98283221529225 delete_result '{"result_id": 1}' --accountId duongnh.testnet
```

---
## Is Voted
1. Check if a User has voted for a Poll or not
```
near view dev-1661499707374-98283221529225 is_voted '{"user_id": 1, "poll_id": 1}'
```

2. View list of IsUserVote (with pagination) of the Contract: (`from_index`: integer, `limit`: integer)
```
near view dev-1661499707374-98283221529225 get_all_is_user_votes '{"from_index": 0, "limit": 10}'
```

## Thứ tự delete
- Delete Poll
- Delete Criteria
- Delete Poll Option
- Delele User