use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::response_types::{
    bet::UserBetResponse, contest_data::ContestDataResponse,
    contest_data_list::ContestDataListResponse, get_snip20::GetSnip20Response,
    minimum_bet::MinimumBetResponse, users_bets::UsersBetsResponse,
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
    Snip20(GetSnip20Response),
}
