use serde::{Deserialize, Serialize};

use crate::data::{bets::Bet, contest_bet_summary::ContestBetSummary, contest_info::ContestInfo};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UserContestBetInfo {
    pub contest_info: ContestInfo,
    pub contest_bet_summary: ContestBetSummary,
    pub user_bet: Bet,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UserBetsResponse {
    pub contests_bets: Vec<UserContestBetInfo>,
}
