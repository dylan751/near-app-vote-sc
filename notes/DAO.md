## Proposals

- Add new proposal (Add Member to DAO)

```
near call btc-app-vote.sputnikv2.testnet add_proposal '{"proposal": {"description": "Add New Council", "kind": {"AddMemberToRole": {"member_id": "btcs-duongnh.testnet", "role": "council"}}}}' --accountId duongnh.testnet --amount 0.1
```

- Add new proposal (Add Poll to DAO)

```
near call btc-app-vote.sputnikv2.testnet add_proposal '{"proposal": {"description": "Should i learn a new technology", "kind": "Vote"}}' --accountId duongnh.testnet --amount 0.1
```

- View all proposals

```
near view btc-app-vote.sputnikv2.testnet get_proposals '{"from_index": 0, "limit": 10}'
```

- View 1 proposal

```
near view btc-app-vote.sputnikv2.testnet get_proposal '{"id": 0}'
```

## Vote

- Approve:

```
near call btc-app-vote.sputnikv2.testnet act_proposal '{"id": 1, "action": "VoteApprove"}' --accountId duongnh.testnet
```

- Reject:

```
near call btc-app-vote.sputnikv2.testnet act_proposal '{"id": 1, "action": "VoteReject"}' --accountId duongnh.testnet
```
