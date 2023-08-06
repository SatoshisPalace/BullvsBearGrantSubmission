use cosmwasm_std::Storage;
use schemars::JsonSchema;
use secret_toolkit::storage::Keymap;
use serde::{Deserialize, Serialize};

use super::constants::{CONNTEST_SAVE_ERROR_MESSAGE, CONTEST_CONFIG_KEY};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct ContestInfo {
    pub id: u8,
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
        id: u8,
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
    pub fn id(&self) -> u8 {
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

static CONTESTS: Keymap<u8, ContestInfo> = Keymap::new(CONTEST_CONFIG_KEY);

pub fn save_contest(storage: &mut dyn Storage, contest_info: &ContestInfo) {
    let key: u8 = contest_info.id;
    CONTESTS
        .insert(storage, &key, &contest_info)
        .expect(CONNTEST_SAVE_ERROR_MESSAGE);
}

pub fn get_contest(storage: &dyn Storage, contest_id: u8) -> Option<ContestInfo> {
    return CONTESTS.get(storage, &contest_id);
}
