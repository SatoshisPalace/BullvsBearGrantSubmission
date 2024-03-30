use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sp_secret_toolkit::macros::{identifiable::Identifiable, keymap::KeymapStorage};

use crate::{
    error::contest_info_error::ContestInfoError,
    services::integrations::oracle_service::oracle::NULL_AND_VOID_CONTEST_RESULT,
};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema, KeymapStorage)]
pub struct ContestInfo {
    id: String,
    options: Vec<ContestOutcome>,
    time_of_close: u64,
    time_of_resolve: u64,
    event_details: String,
}
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct ContestOutcome {
    id: u8,
    name: String,
}
impl ContestOutcome {
    pub fn new(id: u8, name: String) -> Self {
        ContestOutcome { id, name }
    }
    pub fn nullified_result() -> Self {
        ContestOutcome {
            id: NULL_AND_VOID_CONTEST_RESULT,
            name: "Nullified Result".to_string(),
        }
    }
    pub fn get_id(&self) -> &u8 {
        &self.id
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
}

impl ContestInfo {
    pub fn new(
        id: String,
        time_of_close: u64,
        time_of_resolve: u64,
        options: Vec<ContestOutcome>,
        event_details: String,
    ) -> ContestInfo {
        Self {
            id,
            time_of_close,
            time_of_resolve,
            options,
            event_details,
        }
    }
    pub fn get_id(&self) -> String {
        return self.id.clone();
    }
    pub fn get_time_of_close(&self) -> u64 {
        return self.time_of_close;
    }
    pub fn get_time_of_resolve(&self) -> u64 {
        return self.time_of_resolve;
    }
    pub fn get_options(&self) -> &Vec<ContestOutcome> {
        return &self.options;
    }
    pub fn to_json(&self) -> String {
        let raw_json = serde_json::to_string(&self).expect("Failed to serialize struct to JSON");
        return raw_json.replace("\\", "");
    }

    pub fn find_outcome(&self, id: &u8) -> Result<ContestOutcome, ContestInfoError> {
        let option: Option<ContestOutcome> = self
            .options
            .iter()
            .find(|&outcome| outcome.id == *id)
            .cloned();
        option.ok_or(ContestInfoError::OutcomeNotFound {
            contest_id: self.get_id(),
            outcome_id: *id,
        })
    }
}

impl Identifiable for ContestInfo {
    type ID = String;

    fn id(&self) -> Self::ID {
        self.id.clone()
    } // Or another type that implements Serialize + DeserializeOwned
}
