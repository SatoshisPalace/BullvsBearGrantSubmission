use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::commands::{
    get_active_contests::GetActiveContests, get_contest::GetContest, get_contests::GetContests,
    get_min_bet::GetMinBet, get_snip20::GetSnip20, get_user_bet::GetUserBet,
    get_users_bets::GetUsersBets,
};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetContest(GetContest),
    GetContests(GetContests),
    GetActiveContests(GetActiveContests),
    GetUserBet(GetUserBet),
    GetUsersBets(GetUsersBets),
    GetMinBet(GetMinBet),
    GetSnip20(GetSnip20),
}
