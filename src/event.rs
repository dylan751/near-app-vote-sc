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
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct NftMintLog {
    pub owner_id: String,
    pub token_ids: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}

/// An event log to capture token transfer
///
/// Arguments
/// * `authorized_id`: approved account to transfer
/// * `old_owner_id`: "owner.near"
/// * `new_owner_id`: "receiver.near"
/// * `token_ids`: ["1", "12345abc"]
/// * `memo`: optional message
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct NftTransferLog {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_id: Option<String>,

    pub old_owner_id: String,
    pub new_owner_id: String,
    pub token_ids: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn nep_format_vector() {
//         let expected = r#"EVENT_JSON:{"standard":"nep171","version":"1.0.0","event":"nft_mint","data":[{"owner_id":"foundation.near","token_ids":["aurora","proximitylabs"]},{"owner_id":"user1.near","token_ids":["meme"]}]}"#;
//         let log = EventLog {
//             standard: "nep171".to_string(),
//             version: "1.0.0".to_string(),
//             event: EventLogVariant::NftMint(vec![
//                 NftMintLog {
//                     owner_id: "foundation.near".to_owned(),
//                     token_ids: vec!["aurora".to_string(), "proximitylabs".to_string()],
//                     memo: None,
//                 },
//                 NftMintLog {
//                     owner_id: "user1.near".to_owned(),
//                     token_ids: vec!["meme".to_string()],
//                     memo: None,
//                 },
//             ]),
//         };
//         assert_eq!(expected, log.to_string());
//     }

//     #[test]
//     fn nep_format_mint() {
//         let expected = r#"EVENT_JSON:{"standard":"nep171","version":"1.0.0","event":"nft_mint","data":[{"owner_id":"foundation.near","token_ids":["aurora","proximitylabs"]}]}"#;
//         let log = EventLog {
//             standard: "nep171".to_string(),
//             version: "1.0.0".to_string(),
//             event: EventLogVariant::NftMint(vec![NftMintLog {
//                 owner_id: "foundation.near".to_owned(),
//                 token_ids: vec!["aurora".to_string(), "proximitylabs".to_string()],
//                 memo: None,
//             }]),
//         };
//         assert_eq!(expected, log.to_string());
//     }

//     #[test]
//     fn nep_format_transfer_all_fields() {
//         let expected = r#"EVENT_JSON:{"standard":"nep171","version":"1.0.0","event":"nft_transfer","data":[{"authorized_id":"market.near","old_owner_id":"user1.near","new_owner_id":"user2.near","token_ids":["token"],"memo":"Go Team!"}]}"#;
//         let log = EventLog {
//             standard: "nep171".to_string(),
//             version: "1.0.0".to_string(),
//             event: EventLogVariant::NftTransfer(vec![NftTransferLog {
//                 authorized_id: Some("market.near".to_string()),
//                 old_owner_id: "user1.near".to_string(),
//                 new_owner_id: "user2.near".to_string(),
//                 token_ids: vec!["token".to_string()],
//                 memo: Some("Go Team!".to_owned()),
//             }]),
//         };
//         assert_eq!(expected, log.to_string());
//     }
// }