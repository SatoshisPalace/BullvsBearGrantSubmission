use cosmwasm_std::{QuerierWrapper, Storage, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sp_secret_toolkit::{
    macros::{identifiable::Identifiable, keymap::KeymapStorage},
    oracle::response::GetContestResultResponse,
};

use crate::{
    contest::{
        constants::{FEE_PERCENTAGE, PERCENTAGE_BASE},
        error::ContestError,
    },
    integrations::oracle::oracle::{query_contest_result, NULL_AND_VOID_CONTEST_RESULT},
};

use super::contest_info::{ContestInfo, ContestOutcome};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema, KeymapStorage)]
pub struct ContestBetSummary {
    contest_id: u32,
    options: Vec<OptionBetSummary>,
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
        let contest_id = contest_info.id();
        ContestBetSummary {
            contest_id,
            options,
            outcome: None,
        }
    }

    pub fn get_options(&self) -> &Vec<OptionBetSummary> {
        &self.options
    }

    pub fn get_outcome(&self) -> &Option<ContestOutcome> {
        &self.outcome
    }

    fn set_outcome(&mut self, outcome: &ContestOutcome) -> Result<(), ContestError> {
        if self.outcome.is_none() {
            self.outcome = Some(outcome.clone());
            Ok(())
        } else {
            Err(ContestError::CannotResetOutcome)
        }
    }

    pub fn query_set_outcome(
        &mut self,
        querier: &QuerierWrapper,
        storage: &dyn Storage,
        contest_info: &ContestInfo,
        contest_id: u32,
    ) -> Result<ContestOutcome, ContestError> {
        match self.outcome {
            Some(ref outcome) => Ok(outcome.clone()),
            None => {
                // Query oracle if the outcome is not resolved
                let oracle_result: GetContestResultResponse =
                    query_contest_result(querier, storage, &(contest_id as u64))?;
                if oracle_result.result == NULL_AND_VOID_CONTEST_RESULT {
                    return Ok(ContestOutcome::nullified_result());
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
        let total_pool_after_fee =
            total_pool.u128() * (PERCENTAGE_BASE - FEE_PERCENTAGE) / PERCENTAGE_BASE;

        // Get the total allocation for the user's chosen outcome
        let total_allocation_for_outcome = self.get_allocation(outcome_id)?;

        // Calculate the user's share
        // To avoid floating-point arithmetic, we multiply before dividing
        let user_share =
            user_bet_amount.u128() * total_pool_after_fee / total_allocation_for_outcome.u128();

        Ok(Uint128::from(user_share))
    }
}

impl Identifiable for ContestBetSummary {
    type ID = u32; // Or another type that implements Serialize + DeserializeOwned

    fn id(&self) -> Self::ID {
        return self.contest_id;
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct OptionBetSummary {
    option: ContestOutcome,
    bet_allocation: Uint128,
}
impl OptionBetSummary {
    // Constructor
    pub fn new(option: ContestOutcome, bet_allocation: Uint128) -> Self {
        OptionBetSummary {
            option,
            bet_allocation,
        }
    }

    // Getters
    pub fn option(&self) -> &ContestOutcome {
        &self.option
    }

    pub fn bet_allocation(&self) -> Uint128 {
        self.bet_allocation
    }
}
