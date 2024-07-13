use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::commands::{
    get_claimable_contests::GetClaimableContests, get_claimable_fees::GetClaimableFees,
    get_contest_by_id::GetContestById, get_contests_by_ids::GetContestsByIds,
    get_fee_percent::GetFeePercent, get_last_ten_contests::GetLastTenContests,
    get_min_bet::GetMinBet, get_snip20::GetSnip20, get_stats::GetStats,
    get_times_to_resolve::GetTimesToResolve, get_total_number_of_bets::GetTotalNumberOfBets,
    get_total_number_of_contests::GetTotalNumberOfContests, get_total_users::GetTotalUsers,
    get_total_value::GetTotalValue, get_total_volume::GetTotalVolume, get_user_bet::GetUserBet,
    get_users_last_ten_bets::GetUsersLastTenBets, get_users_list_of_bets::GetUsersListOfBets,
    get_users_number_of_bets::GetUsersNumberOfBets,
};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetContestById(GetContestById),
    GetContestsByIds(GetContestsByIds),
    GetUserBet(GetUserBet),
    GetUsersLastTenBets(GetUsersLastTenBets),
    GetMinBet(GetMinBet),
    GetTotalValue(GetTotalValue),
    GetSnip20(GetSnip20),
    GetTimesToResolve(GetTimesToResolve),
    GetClaimableFees(GetClaimableFees),
    GetClaimableContests(GetClaimableContests),
    GetFeePercent(GetFeePercent),
    GetTotalNumberOfContests(GetTotalNumberOfContests),
    GetTotalNumberOfBets(GetTotalNumberOfBets),
    GetTotalVolume(GetTotalVolume),
    GetUsersNumberOfBets(GetUsersNumberOfBets),
    GetUsersListOfBets(GetUsersListOfBets),
    GetLastTenContests(GetLastTenContests),
    GetTotalUsers(GetTotalUsers),
    GetStats(GetStats),
}
