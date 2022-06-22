use crate::*;

// ----------------------------------- User Struct -----------------------------------
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct User {
    
}

// ----------------------------------- Vote Struct -----------------------------------
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Vote {
    
}

// ----------------------------------- Criteria Struct -----------------------------------
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Criteria {
    
}

// ----------------------------------- Result Struct -----------------------------------
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Result {
    
}