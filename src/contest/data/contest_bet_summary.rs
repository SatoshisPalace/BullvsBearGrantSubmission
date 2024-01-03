use cosmwasm_std::{Storage, Uint128, QuerierWrapper};
use schemars::JsonSchema;
use secret_toolkit::storage::Keymap;
use serde::{Deserialize, Serialize};

use crate::{contest::{
    constants::{CONTEST_BET_SUMMARY_CONFIG_KEY, FEE_PERCENTAGE, PERCENTAGE_BASE},
    error::ContestError,
}, integrations::oracle::{oracle::query_contest_result, response::GetContestResultResponse, constants::NULL_AND_VOID_CONTEST_RESULT}};

use super::contest_info::{ContestInfo, ContestOutcome};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct OptionBetSummary {
    pub option: ContestOutcome,
    pub bet_allocation: Uint128,
}
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]

pub struct ContestBetSummary {
    pub options: Vec<OptionBetSummary>,
    outcome: Option<ContestOutcome>,
}

impl ContestBetSummary {
    pub fn initialize(contest_info: &ContestInfo) -> Self {
        let options = contest_info
            .options
            .iter()
            .map(|option| OptionBetSummary {
                option: option.clone(),
                bet_allocation: Uint128::zero(),
            })
            .collect();

        ContestBetSummary { 
            options,
            outcome: None 
        }
    }

    pub fn set_outcome(&mut self, outcome: &ContestOutcome)->Result<(), ContestError>{
        if self.outcome.is_none(){
            self.outcome = Some(outcome.clone());
            Ok(())
        }else{
            Err(ContestError::CannotResetOutcome)
        }
    }

    pub fn get_outcome(
        &mut self, 
        querier: &QuerierWrapper, 
        storage: &dyn Storage, 
        contest_info: &ContestInfo,
        contest_id: u32
    ) -> Result<ContestOutcome, ContestError> {
        match self.outcome {
            Some(ref outcome) => Ok(outcome.clone()),
            None => {
                // Query oracle if the outcome is not resolved
                let oracle_result: GetContestResultResponse = query_contest_result(querier, storage, contest_id as u64)?;
                if oracle_result.result == NULL_AND_VOID_CONTEST_RESULT {
                    return Ok(ContestOutcome::nullified_result())
                }                
                let outcome: ContestOutcome = contest_info.find_outcome(oracle_result.result)?;
                self.set_outcome(&outcome)?;
                Ok(outcome)
            }
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
    pub fn add_bet_to_option(
        &mut self,
        outcome_id: u8,
        amount: Uint128,
    ) -> Result<(), ContestError> {
        for option_summary in &mut self.options {
            if option_summary.option.id == outcome_id {
                option_summary.bet_allocation += amount;
                return Ok(());
            }
        }
        Err(ContestError::OutcomeDNE)
    }
    
    pub fn calculate_user_share(
        &self,
        user_bet_amount: Uint128,
        outcome_id: u8,
    ) -> Result<Uint128, ContestError> {
        // Calculate the total pool
        let total_pool = self.calc_total_pool();

        // Apply the fee
        let total_pool_after_fee = total_pool.u128() * (PERCENTAGE_BASE - FEE_PERCENTAGE) / PERCENTAGE_BASE;

        // Get the total allocation for the user's chosen outcome
        let total_allocation_for_outcome = self.get_allocation(outcome_id)?;

        // Calculate the user's share
        // To avoid floating-point arithmetic, we multiply before dividing
        let user_share = user_bet_amount.u128() * total_pool_after_fee / total_allocation_for_outcome.u128();

        Ok(Uint128::from(user_share))
    }
}

static CONTEST_BET_SUMMARIES: Keymap<u32, ContestBetSummary> =
    Keymap::new(CONTEST_BET_SUMMARY_CONFIG_KEY);

////////
pub fn update_contest_bet_summary(
    storage: &mut dyn Storage,
    contest_id: u32,
    amount: Uint128,
    outcome_id: u8,
) -> Result<(), ContestError> {
    let mut current_contest_bet_summary = match get_contest_bet_summary(storage, contest_id) {
        Some(summary) => summary,
        None => return Err(ContestError::ContestDNE),
    };

    current_contest_bet_summary.add_bet_to_option(outcome_id, amount)?;
    save_contest_bet_summary(storage, contest_id, &current_contest_bet_summary)
}

pub fn get_contest_bet_summary(
    storage: &dyn Storage,
    contest_id: u32,
) -> Option<ContestBetSummary> {
    return CONTEST_BET_SUMMARIES.get(storage, &contest_id);
}

pub fn save_contest_bet_summary(
    storage: &mut dyn Storage,
    contest_id: u32,
    contest_bet_summary: &ContestBetSummary,
) -> Result<(), ContestError> {
    CONTEST_BET_SUMMARIES.insert(storage, &contest_id, contest_bet_summary)?;
    Ok(())
}
