use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, Promise, Timestamp};

pub type UserId = u32;
pub type CriteriaId = u32;
pub type PollId = u32;
pub type PollOptionId = u32;
pub type ResultId = u32;

pub use crate::criterias::*;
pub use crate::custom_struct::*;
pub use crate::poll_options::*;
pub use crate::polls::*;
pub use crate::results::*;
pub use crate::tests::*;
pub use crate::users::*;
use crate::utils::*;

mod criterias;
mod custom_struct;
mod poll_options;
mod polls;
mod results;
mod tests;
mod users;
mod utils;

const PAGINATION_SIZE: u64 = 10;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct AppVoteContract {
    pub owner_id: AccountId, // Account id of the Smart Contract
    pub users_by_id: UnorderedMap<UserId, User>, // List of Users in this Smart Contract
    pub criterias_by_id: UnorderedMap<CriteriaId, Criteria>, // List of Criterias in this Smart Contract
    pub polls_by_id: UnorderedMap<PollId, Poll>,             // List of Votes in this Smart Contract
    pub poll_options_by_id: UnorderedMap<PollOptionId, PollOption>, // List of Results in this Smart Contract
    pub results_by_id: UnorderedMap<ResultId, Result>, // List of Results in this Smart Contract

    pub users_by_id_counter: u32,     // Counter of the list of User Id
    pub polls_by_id_counter: u32,     // Counter of the list of Poll Id
    pub criterias_by_id_counter: u32, // Counter of the list of Criteria Id
    pub poll_options_by_id_counter: u32, // Counter of the list of PollOption Id
    pub results_by_id_counter: u32,   // Counter of the list of Result Id
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum StorageKey {
    UsersByIdKey,
    CriteriasByIdKey,
    PollsByIdKey,
    PollOptionsByIdKey,
    ResultsByIdKey,
}

#[near_bindgen]
impl AppVoteContract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            users_by_id: UnorderedMap::new(StorageKey::UsersByIdKey.try_to_vec().unwrap()),
            criterias_by_id: UnorderedMap::new(StorageKey::CriteriasByIdKey.try_to_vec().unwrap()),
            polls_by_id: UnorderedMap::new(StorageKey::PollsByIdKey.try_to_vec().unwrap()),
            poll_options_by_id: UnorderedMap::new(
                StorageKey::PollOptionsByIdKey.try_to_vec().unwrap(),
            ),
            results_by_id: UnorderedMap::new(StorageKey::ResultsByIdKey.try_to_vec().unwrap()),
            users_by_id_counter: 1,
            polls_by_id_counter: 1,
            criterias_by_id_counter: 1,
            poll_options_by_id_counter: 1,
            results_by_id_counter: 1,
        }
    }
}
