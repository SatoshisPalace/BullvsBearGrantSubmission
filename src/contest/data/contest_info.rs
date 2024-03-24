use cosmwasm_std::Storage;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sp_secret_toolkit::macros::{identifiable::Identifiable, keymap::KeymapStorage};

use crate::{
    contest::error::{
        contest_info_error::ContestInfoError, real_contest_info_error::RealContestInfoError,
    },
    integrations::oracle::oracle::NULL_AND_VOID_CONTEST_RESULT,
};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema, KeymapStorage)]
pub struct ContestInfo {
    pub id: u32,
    pub options: Vec<ContestOutcome>,
    pub time_of_close: u64,
    pub time_of_resolve: u64,
    pub event_details: String,
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
        id: u32,
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
    pub fn id(&self) -> u32 {
        return self.id;
    }
    pub fn time_of_close(&self) -> u64 {
        return self.time_of_close;
    }
    pub fn time_of_resolve(&self) -> u64 {
        return self.time_of_resolve;
    }
    pub fn options(&self) -> &Vec<ContestOutcome> {
        return &self.options;
    }
    pub fn to_json(&self) -> String {
        let raw_json = serde_json::to_string(&self).expect("Failed to serialize struct to JSON");
        return raw_json.replace("\\", "");
    }
    pub fn assert_time_of_close_not_passed(
        &self,
        current_time: u64,
    ) -> Result<(), RealContestInfoError> {
        if current_time >= self.time_of_close {
            Err(RealContestInfoError::TimeOfClosePassed(self.id))
        } else {
            Ok(())
        }
    }

    pub fn assert_time_of_resolve_is_passed(
        &self,
        current_time: u64,
    ) -> Result<(), RealContestInfoError> {
        if current_time < self.time_of_resolve {
            return Err(RealContestInfoError::TimeOfResolveHasYetToPassed {
                contest_id: self.id,
                time_of_resolve: self.time_of_resolve,
                current_time,
            });
        }
        Ok(())
    }

    pub fn find_outcome(&self, id: u8) -> Result<ContestOutcome, RealContestInfoError> {
        let option: Option<ContestOutcome> = self
            .options
            .iter()
            .find(|&outcome| outcome.id == id)
            .cloned();
        option.ok_or(RealContestInfoError::OutcomeNotFound {
            contest_id: self.id(),
            outcome_id: id,
        })
    }

    pub fn validate_contest(&self) -> Result<(), RealContestInfoError> {
        if self.options.iter().any(|outcome| outcome.id == 0) {
            return Err(RealContestInfoError::InvalidOutcomeId {
                contest_id: self.id,
            });
        }
        Ok(())
    }
}

impl Identifiable for ContestInfo {
    type ID = u32;

    fn id(&self) -> Self::ID {
        self.id
    } // Or another type that implements Serialize + DeserializeOwned
}

pub fn verify_contest(
    storage: &dyn Storage,
    contest_id: &u32,
    outcome_id: u8,
) -> Result<ContestInfo, ContestInfoError> {
    let contest = ContestInfo::keymap_get_by_id(storage, contest_id);

    // Check if the contest exists
    if let Some(contest) = contest {
        // Check if the option_id exists within the contest's options
        if contest
            .options
            .iter()
            .any(|outcome| outcome.id == outcome_id)
        {
            Ok(contest)
        } else {
            Err(ContestInfoError::OutcomeDNE)
        }
    } else {
        Err(ContestInfoError::ContestDNE)
    }
}
