use cosmwasm_std::Storage;
use schemars::JsonSchema;
use secret_toolkit::storage::Keymap;
use serde::{Deserialize, Serialize};

use crate::contest::{constants::{CONNTEST_SAVE_ERROR_MESSAGE, CONTEST_CONFIG_KEY}, error::ContestError};


#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct ContestInfo {
    pub id: u32,
    pub options: Vec<ContestOutcome>,
    pub time_of_close: i32,
    pub time_of_resolve: i32,
}
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct ContestOutcome {
    pub id: u8,
    pub name: String,
}

impl<'a> ContestInfo {
    pub fn new(
        id: u32,
        time_of_close: i32,
        time_of_resolve: i32,
        options: Vec<ContestOutcome>,
    ) -> ContestInfo {
        Self {
            id,
            time_of_close,
            time_of_resolve,
            options,
        }
    }
    pub fn id(&self) -> u32 {
        return self.id;
    }
    pub fn time_of_close(&self) -> i32 {
        return self.time_of_close;
    }
    pub fn time_of_resolve(&self) -> i32 {
        return self.time_of_resolve;
    }
    pub fn options(&self) -> &Vec<ContestOutcome> {
        return &self.options;
    }
    pub fn to_json(&self) -> String {
        let raw_json = serde_json::to_string(&self).expect("Failed to serialize struct to JSON");
        return raw_json.replace("\\", "");
    }
}

static CONTESTS: Keymap<u32, ContestInfo> = Keymap::new(CONTEST_CONFIG_KEY);

pub fn save_contest(storage: &mut dyn Storage, contest_info: &ContestInfo) {
    let key  = contest_info.id;
    CONTESTS
        .insert(storage, &key, &contest_info)
        .expect(CONNTEST_SAVE_ERROR_MESSAGE);
}

pub fn get_contest(storage: &dyn Storage, contest_id: u32) -> Option<ContestInfo> {
    return CONTESTS.get(storage, &contest_id);
}

pub fn verify_contest(storage: &dyn Storage, contest_id: u32, outcome_id: u8) -> Result<(), ContestError> {
    let contest = get_contest(storage, contest_id);
    
    // Check if the contest exists
    if let Some(contest) = contest {
        // Check if the option_id exists within the contest's options
        if contest.options.iter().any(|outcome| outcome.id == outcome_id) {
            Ok(())
        } else {
            Err(ContestError::OutcomeDNE)
        }
    } else {
        Err(ContestError::ContestDNE)
    }
}