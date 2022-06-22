use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, serde_json, AccountId, PanicOnDefault};

pub type UserId = u32;
pub type VoteId = u32;
pub type CriteriaId = u32;
pub type ResultId = u32;

pub use crate::custom_struct::*;

mod custom_struct;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct AppVoteContract {
    pub owner_id: AccountId, // Account id of the Smart Contract
    pub users_by_id: UnorderedMap<UserId, User>, // List of Users in this Smart Contract
    pub votes_by_id: UnorderedMap<VoteId, Vote>, // List of Votes in this Smart Contract
    pub criterias_by_id: UnorderedMap<CriteriaId, Criteria>, // List of Criterias in this Smart Contract
    pub results_by_id: UnorderedMap<ResultId, Result>, // List of Results in this Smart Contract
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum StorageKey {
    UsersByIdKey,
    VotesByIdKey,
    CriteriasByIdKey,
    ResultsByIdKey,
}

#[near_bindgen]
impl AppVoteContract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            users_by_id: UnorderedMap::new(StorageKey::UsersByIdKey.try_to_vec().unwrap()),
            votes_by_id: UnorderedMap::new(StorageKey::VotesByIdKey.try_to_vec().unwrap()),
            criterias_by_id: UnorderedMap::new(StorageKey::CriteriasByIdKey.try_to_vec().unwrap()),
            results_by_id: UnorderedMap::new(StorageKey::ResultsByIdKey.try_to_vec().unwrap()),
        }
    }
}
