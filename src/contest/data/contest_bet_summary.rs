use cosmwasm_std::{Uint128, Storage};
use schemars::JsonSchema;
use secret_toolkit::storage::Keymap;
use serde::{Deserialize, Serialize};

use crate::contest::{error::ContestError, constants::CONTEST_BET_SUMMARY_CONFIG_KEY};

use super::contest_info::{ContestInfo, ContestOutcome};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct OptionBetSummary{
    pub option: ContestOutcome,
    pub bet_allocation: Uint128,
}
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]

pub struct ContestBetSummary {
    pub options: Vec<OptionBetSummary>,
}

impl ContestBetSummary{
    pub fn initialize(contest_info: &ContestInfo) -> Self {
        let options = contest_info.options.iter().map(|option| {
            OptionBetSummary {
                option: option.clone(),
                bet_allocation: Uint128::zero(),
            }
        }).collect();

        ContestBetSummary {
            options
        }
    }

	pub fn calc_total_pool(&self) -> Uint128 {
        let mut total: Uint128 = Uint128::from(0u128);
        for option in &self.options {
            total += option.bet_allocation;
        }
        total
    }
	pub fn get_allocation(&self, outcome_id: u8) -> Result<Uint128, ContestError> {
        for option in &self.options {
            if option.option.id == outcome_id {
                return Ok(option.bet_allocation);
            }
        }
        Err(ContestError::OutcomeDNE)
    }
	pub fn add_bet_to_option(&mut self, outcome_id: u8, amount: Uint128) -> Result<(), ContestError> {
        for option_summary in &mut self.options {
            if option_summary.option.id == outcome_id {
                option_summary.bet_allocation += amount;
                return Ok(());
            }
        }
        Err(ContestError::OutcomeDNE)
    }
}


static CONTEST_BET_SUMMARIES: Keymap<u32, ContestBetSummary> = Keymap::new(CONTEST_BET_SUMMARY_CONFIG_KEY);

////////
pub fn update_contest_bet_summary(storage: &mut dyn Storage, contest_id: u32, amount: Uint128, outcome_id: u8) -> Result<(), ContestError> {
    let mut current_contest_bet_summary = match get_contest_bet_summary(storage, contest_id) {
        Some(summary) => summary,
        None => return Err(ContestError::ContestDNE),
    };

    current_contest_bet_summary.add_bet_to_option(outcome_id, amount)?;
    save_contest_bet_summary(storage, contest_id, &current_contest_bet_summary)
}

pub fn get_contest_bet_summary(storage: &dyn Storage, contest_id: u32) -> Option<ContestBetSummary> {
    return CONTEST_BET_SUMMARIES.get(storage, &contest_id);
}

pub fn save_contest_bet_summary(storage: &mut dyn Storage, contest_id: u32, contest_bet_summary: &ContestBetSummary) -> Result<(), ContestError> {
    CONTEST_BET_SUMMARIES.insert(storage, &contest_id, contest_bet_summary)?;
    Ok(())
}