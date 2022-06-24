use crate::*;

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub enum Role {
    Admin,
    Employee,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub enum Status {
    Done,
    Pending,
    Failed,
}

// ----------------------------------- User Struct -----------------------------------
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct User {
    pub name: AccountId,            // Name of the User
    pub role: Role,                 // Role of the User
    pub email: String,              // Email of the User
    pub near_account_id: AccountId, // Near Account of the User
    pub created_at: Option<Timestamp>,
    pub updated_at: Option<Timestamp>,
}

// ----------------------------------- Vote Struct -----------------------------------
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Vote {
    pub criteria_id: CriteriaId, // Id of the Criteria to vote for
    pub user_id: UserId,         // Id of the User who vote
    pub status: Status,          // Status of the Vote
    pub month: u32,              // Voted month
    pub start_at: Option<Timestamp>,     // Start time of the Vote (In epoch -> nanoseconds)
    pub end_at: Option<Timestamp>,       // End time of the Vote (In epoch -> nanoseconds)
    pub created_at: Option<Timestamp>,
    pub updated_at: Option<Timestamp>,
}

// ----------------------------------- Criteria Struct -----------------------------------
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Criteria {
    pub user_id: UserId,     // Id of the User who created this Criteria
    pub description: String, // Description of the Criteria
    pub created_at: Option<Timestamp>,
    pub updated_at: Option<Timestamp>,
}

// ----------------------------------- Result Struct -----------------------------------
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Result {
    pub month: u32,      // Voted month
    pub user_id: UserId, // Id of the User of this Result
    pub total_vote: u32, // Total Vote for this User
    pub created_at: Option<Timestamp>,
    pub updated_at: Option<Timestamp>,
}
