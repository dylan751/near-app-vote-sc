use crate::*;

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub enum Role {
    Admin,
    Employee,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub enum BlockchainType {
    Near,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct UserWallet {
    pub blockchain_type: BlockchainType, // Blockchain's type of this User Wallet
    pub wallet_address: String,          // Wallet Account
}

// ----------------------------------- User Struct -----------------------------------
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct User {
    pub id: UserId,              // Id of the User
    pub name: AccountId,         // Name of the User
    pub role: Role,              // Role of the User
    pub email: String,           // Email of the User
    pub user_wallet: UserWallet, // Blockchain Wallet of the User
    pub created_at: Option<Timestamp>,
    pub updated_at: Option<Timestamp>,
}

// ----------------------------------- Criteria Struct -----------------------------------
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Criteria {
    pub id: CriteriaId,      // Id of the Criteria
    pub created_by: UserId,  // Id of the User who created this Criteria
    pub description: String, // Description of the Criteria
    pub created_at: Option<Timestamp>,
    pub updated_at: Option<Timestamp>,
}

// ----------------------------------- Poll Struct -----------------------------------
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Poll {
    pub id: PollId,                                      // Id of the Poll
    pub criteria_option_id_array: Vec<CriteriaOptionId>, // Array of relative Criterias and Answer Options for this Poll
    pub created_by: UserId,                              // Id of the User who vote
    pub img_url: Option<String>,                         // URL of the Poll's image
    pub title: String,                                   // Title of the Poll
    pub description: String,                             // Description of the Poll
    pub start_at: Option<Timestamp>, // Start time of the Vote (In epoch -> nanoseconds)
    pub end_at: Option<Timestamp>,   // End time of the Vote (In epoch -> nanoseconds)
    pub created_at: Option<Timestamp>,
    pub updated_at: Option<Timestamp>,
}

// ----------------------------------- Option Struct -----------------------------------
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct PollOption {
    pub id: PollOptionId,
    pub created_by: UserId,   // Id of the User who vote
    pub title: String,        // Title of the PollOption
    pub description: String,  // Description of the PollOption
    pub options: Vec<String>, // List of options of this poll option
    pub created_at: Option<Timestamp>,
    pub updated_at: Option<Timestamp>,
}

// ----------------------------------- Result Struct -----------------------------------
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Result {
    pub id: ResultId,            // Id of the Result
    pub criteria_id: CriteriaId, // Id of the Criteria this Vote belongs to
    pub poll_id: PollId,         // Id of the Poll this Vote belongs to
    pub option: String,          // The Option that being Voted
    pub total_vote: u32,         // Total Vote for this User
    pub created_at: Option<Timestamp>,
    pub updated_at: Option<Timestamp>,
}
// ----------------------------------- Result by a Poll's Criteria Struct -----------------------------------
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ResultByPollCriteria {
    pub poll_id: PollId,         // Id of the Poll this Vote belongs to
    pub criteria_id: CriteriaId, // Id of the Criteria this Vote belongs to
    pub option: String,          // Id of the User of the Vote
    pub total_vote: u32,         // Total Vote for this User
}

// ------------------------ Check if user_id has voted for poll_id or not ------------------------
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct IsUserVote {
    pub user_id: UserId, // If of the User
    pub poll_id: PollId, // If of the Poll
    pub is_voted: bool,  // Check if this User has voted for this Poll or not
}

// An Option id due to a Crite
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct CriteriaOption {
    pub criteria_id: CriteriaId,
    pub option: String,
}

// An Option id due to a Criteria
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct CriteriaOptionId {
    pub criteria_id: CriteriaId,
    pub poll_option_id: PollOptionId,
}
