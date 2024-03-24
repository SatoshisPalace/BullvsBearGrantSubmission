use std::collections::HashMap;

use crate::contest::data::contest_info::{ContestInfo, ContestOutcome};

use super::constants::FAR_IN_THE_FUTURE;

#[derive(Debug, Clone)]
pub struct ContestInfoWithSignature {
    pub contest_info: ContestInfo,
    pub signature_hex: String,
}

pub fn get_contest_infos_with_signatures() -> HashMap<u32, ContestInfoWithSignature> {
    let mut contest_infos = HashMap::new();

    // Example of inserting a ContestInfoWithSignature
    contest_infos.insert(1, ContestInfoWithSignature {
        contest_info: ContestInfo {
            id: 1,
            options: vec![
                ContestOutcome::new(1, "option1".to_string()),
                ContestOutcome::new(2, "option2".to_string()),
            ],
            event_details: "Example event details".to_string(),
            time_of_close: FAR_IN_THE_FUTURE,
            time_of_resolve: FAR_IN_THE_FUTURE,
        },
        signature_hex: "b5876a3fc9f0ff470fd2d5d446dbdac994486eb2c7db61ebc2bd6e96a5fb05f7773b3eb0e59d1a4dc80e317e4e69bb6bbb7635084c29dd1bedeabcd4544a9d40".to_string(),
    });

    // Add more ContestInfoWithSignature objects to the HashMap as needed

    contest_infos
}

pub fn get_contest_info_and_signature_by_id(id: u32) -> Option<(ContestInfo, String)> {
    let contest_infos = get_contest_infos_with_signatures();
    contest_infos.get(&id).map(|info_with_sig| {
        (
            info_with_sig.contest_info.clone(),
            info_with_sig.signature_hex.clone(),
        )
    })
}
