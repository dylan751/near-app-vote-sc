/**
 * Để các ứng dụng (bên thứ 3) theo dõi được các
 * hành động (events) cho tất cả các Near-driven apps.
 */
use std::fmt;

use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json;

/// Enum that represents the data type of the EventLog.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[serde(crate = "near_sdk::serde")]
#[non_exhaustive]
pub enum EventLogVariant {
    CreateUser,
    UpdateUser,
    CreateCriteria,
    UpdateCriteria,
    CreatePollOption,
    UpdatePollOption,
    CreatePoll,
    UpdatePoll,
}

/// Interface to capture data about an event
///
/// Arguments:
/// * `standard`: name of standard e.g. nep171
/// * `version`: e.g. 1.0.0
/// * `event`: associate event data
/// * `data`?: Stringified JSON 
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct EventLog {
    pub standard: String,
    pub version: String,

    // `flatten` to not have "event": {<EventLogVariant>} in the JSON, just have the contents of {<EventLogVariant>}.
    #[serde(flatten)]
    pub event: EventLogVariant,
    pub data: String,
}

impl fmt::Display for EventLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "EVENT_JSON:{}",
            &serde_json::to_string(self).map_err(|_| fmt::Error)?
        ))
    }
}

// --- EXAMPLE INTERFACE ---
// /// An event log to capture token minting
// ///
// /// Arguments
// /// * `owner_id`: "account.near"
// /// * `token_ids`: ["1", "abc"]
// /// * `memo`: optional message
// #[derive(Serialize, Deserialize, Debug)]
// #[serde(crate = "near_sdk::serde")]
// pub struct NftMintLog {
//     pub owner_id: String,
//     pub token_ids: Vec<String>,

//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub memo: Option<String>,
// }

