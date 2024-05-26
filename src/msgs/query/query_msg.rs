use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::commands::{
    get_claimable_fees::GetClaimableFees, get_contest_by_id::GetContestById,
    get_contests::GetContests, get_contests_by_ids::GetContestsByIds,
    get_fee_percent::GetFeePercent, get_min_bet::GetMinBet, get_snip20::GetSnip20,
    get_times_to_resolve::GetTimesToResolve, get_total_value::GetTotalValue,
    get_user_bet::GetUserBet, get_users_bets::GetUsersBets,
};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetContestById(GetContestById),
    GetContestsByIds(GetContestsByIds),
    GetContests(GetContests),
    GetUserBet(GetUserBet),
    GetUsersBets(GetUsersBets),
    GetMinBet(GetMinBet),
    GetTotalValue(GetTotalValue),
    GetSnip20(GetSnip20),
    GetTimesToResolve(GetTimesToResolve),
    GetClaimableFees(GetClaimableFees),
    GetFeePercent(GetFeePercent),
}
