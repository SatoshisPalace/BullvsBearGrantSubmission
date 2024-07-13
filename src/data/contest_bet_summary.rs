use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sp_secret_toolkit::macros::{identifiable::Identifiable, keymap::KeymapStorage};

use crate::error::contest_bet_summary_error::ContestBetSummaryError;

use super::{
    contest_info::{ContestId, ContestInfo, ContestOutcome},
    state::FeePercent,
};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema, KeymapStorage)]
pub struct ContestBetSummary {
    contest_id: <ContestInfo as Identifiable>::ID,
    options: Vec<OptionBetSummary>,
    outcome: Option<ContestOutcome>,
    fee: FeePercent,
}

impl ContestBetSummary {
    pub fn new(contest_info: &ContestInfo, fee: &FeePercent) -> Self {
        let options = contest_info
            .get_options()
            .iter()
            .map(|option| OptionBetSummary::new(option.clone()))
            .collect();
        let contest_id = contest_info.get_id();
        ContestBetSummary {
            contest_id,
            options,
            outcome: None,
            fee: fee.clone(),
        }
    }

    pub fn get_outcome(&self) -> &Option<ContestOutcome> {
        &self.outcome
    }

    pub fn set_outcome(&mut self, outcome: &ContestOutcome) -> Result<(), ContestBetSummaryError> {
        if self.outcome.is_none() {
            self.outcome = Some(outcome.clone());
            Ok(())
        } else {
            Err(ContestBetSummaryError::CannotResetOutcome)
        }
    }

    pub fn get_fee(&self) -> &FeePercent {
        &self.fee
    }

    pub fn calc_total_pool(&self) -> Uint128 {
        let mut total: Uint128 = Uint128::from(0u128);
        for option in &self.options {
            total += option.bet_allocation;
        }
        total
    }

    pub fn get_allocation(&self, outcome_id: u8) -> Result<Uint128, ContestBetSummaryError> {
        for option in &self.options {
            if option.option.get_id() == &outcome_id {
                return Ok(option.bet_allocation);
            }
        }
        Err(ContestBetSummaryError::OutcomeDNE)
    }

    pub fn add_bet_to_option(
        &mut self,
        outcome_id: &u8,
        amount: &Uint128,
    ) -> Result<(), ContestBetSummaryError> {
        for option_summary in &mut self.options {
            if option_summary.option.get_id() == outcome_id {
                option_summary.add_bet(amount);
                return Ok(());
            }
        }
        Err(ContestBetSummaryError::OutcomeDNE)
    }
}

impl Identifiable for ContestBetSummary {
    type ID = ContestId; // Or another type that implements Serialize + DeserializeOwned

    fn id(&self) -> Self::ID {
        return self.contest_id.clone();
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct OptionBetSummary {
    option: ContestOutcome,
    num_bets: u32,
    bet_allocation: Uint128,
}
impl OptionBetSummary {
    // Constructor
    pub fn new(option: ContestOutcome) -> Self {
        OptionBetSummary {
            option,
            num_bets: 0,
            bet_allocation: Uint128::zero(),
        }
    }

    // Getters
    pub fn add_bet(&mut self, amount: &Uint128) {
        self.num_bets += 1;
        self.bet_allocation += amount
    }
}
