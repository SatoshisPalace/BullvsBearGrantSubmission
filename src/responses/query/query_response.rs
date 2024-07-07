use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::response_types::{
    bet::UserBetResponse, claimable_fees::ClaimableFeesResponse, contest_data::ContestDataResponse,
    contest_data_list::ContestDataListResponse, fee_percent::FeePercentResponse,
    get_claimable_value::ClaimableValueResponse, get_snip20::GetSnip20Response,
    minimum_bet::MinimumBetResponse, times_to_resolve::TimesToResolveResponse,
    total_number_of_bets::TotalNumberOfBetsResponse,
    total_number_of_contests::TotalNumberOfContestsResponse,
    total_number_of_users::TotalNumberOfUsersResponse,
    total_users_number_of_bets::TotalUsersNumberOfBetsResponse, total_value::TotalValueResponse,
    total_volume::TotalVolumeResponse, users_bets::UsersBetsResponse,
};

// Enum to encapsulate each query response type
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryResponse {
    ContestData(ContestDataResponse),
    ContestDataList(ContestDataListResponse),
    UserBet(UserBetResponse),
    UsersBets(UsersBetsResponse),
    MinimumBet(MinimumBetResponse),
    TotalValue(TotalValueResponse),
    Snip20(GetSnip20Response),
    TimesToResolve(TimesToResolveResponse),
    ClaimableFees(ClaimableFeesResponse),
    FeePercent(FeePercentResponse),
    ClaimableValue(ClaimableValueResponse),
    TotalNumberOfContests(TotalNumberOfContestsResponse),
    TotalNumberOfBets(TotalNumberOfBetsResponse),
    TotalVolume(TotalVolumeResponse),
    TotalUsersNumberOfBets(TotalUsersNumberOfBetsResponse),
    TotalNumberOfUsers(TotalNumberOfUsersResponse),
}
