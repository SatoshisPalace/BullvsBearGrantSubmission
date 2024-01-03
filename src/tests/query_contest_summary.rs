#[cfg(test)]
pub mod tests {
    use cosmwasm_std::{
        from_binary,
        testing::{mock_dependencies, mock_env, MockApi, MockQuerier, MockStorage},
        Empty, OwnedDeps, Uint128,
    };

    use crate::{
        contest::{response::ContestQueryResponse, error::ContestError, constants::{PERCENTAGE_BASE, FEE_PERCENTAGE}},
        contract::query,
        msg::{ExecuteMsg, QueryMsg},
        tests::{
            bet_contest_test::tests::{_bet_contest_test, _get_valid_bet_contest_msg},
            contract_init_test::tests::_initialize_test,
            create_contest_test::tests::{_create_contest_test, _get_valid_create_contest_msg},
        },
    };

    ////////TESTS////////
    #[test]
    fn query_contest_after_creation() {
        let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> =
            mock_dependencies();

        _initialize_test(&mut deps);

        let msg: ExecuteMsg = _get_valid_create_contest_msg();

        _create_contest_test(&mut deps, msg);

        _query_contest_with_initial_bet(&mut deps);
    }

    #[test]
    fn query_contest_after_creation_and_bet() {
        let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> =
            mock_dependencies();

        _initialize_test(&mut deps);

        let msg: ExecuteMsg = _get_valid_create_contest_msg();

        _create_contest_test(&mut deps, msg);

        _query_contest_with_initial_bet(&mut deps);

        _bet_contest_test(&mut deps);

        _query_contest_with_additional_bet(&mut deps)
    }
    ////////INNER TESTS////////

    pub fn _query_contest_with_initial_bet(
        deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
    ) {
        let execute_msg = _get_valid_create_contest_msg();
        let (contest_info, amount) = match execute_msg {
            ExecuteMsg::CreateContest {
                contest_info,
                amount,
                ..
            } => (contest_info, amount),
            _ => panic!("Expected CreateContest variant"),
        };

        let msg = QueryMsg::GetContest {
            contest_id: contest_info.id,
        };

        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let contest_query_response: ContestQueryResponse = from_binary(&res).unwrap();

        assert_eq!(
            contest_info.options.len(),
            contest_query_response.contest_bet_summary.options.len()
        );
        assert_eq!(
            amount.unwrap(),
            contest_query_response.contest_bet_summary.calc_total_pool()
        );
    }
    // Function to query the total pool for a given contest.
    pub fn query_total_pool(
        deps: &OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
        contest_id: u32,
    ) -> Uint128 {
        let msg = QueryMsg::GetContest { contest_id };
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let contest_query_response: ContestQueryResponse = from_binary(&res).unwrap();
        contest_query_response.contest_bet_summary.calc_total_pool() // This should return Uint128
    }

    // Function to calculate the user's share of the winnings.
    // Function to calculate the user's share of the winnings, replicating contract logic.
    pub fn calculate_user_share(
        deps: &OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
        contest_id: u32,
        user_bet_amount: u128,
        outcome_id: u8,
    ) -> Result<u128, ContestError> {
        // Query contest details to get total pool and allocation for the specific outcome
        let msg = QueryMsg::GetContest { contest_id };
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let contest_query_response: ContestQueryResponse = from_binary(&res).unwrap();
    
        // Extract total pool
        let total_pool = contest_query_response.contest_bet_summary.calc_total_pool().u128();
    
        // Apply the fee
        let total_pool_after_fee = total_pool * (PERCENTAGE_BASE - FEE_PERCENTAGE) / PERCENTAGE_BASE;
    
        // Get the total allocation for the user's chosen outcome
        let total_allocation_for_outcome = contest_query_response.contest_bet_summary.get_allocation(outcome_id)?.u128();
    
        // Calculate the user's share proportionally to their bet and the total allocation
        let user_share = if total_allocation_for_outcome > 0 {
            user_bet_amount * total_pool_after_fee / total_allocation_for_outcome
        } else {
            0 // Or handle error as you see fit
        };
    
        Ok(user_share)
    }
    

    pub fn _query_contest_with_additional_bet(
        deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
    ) {
        let execute_msg = _get_valid_create_contest_msg();
        let (contest_info, initial_amount) = match execute_msg {
            ExecuteMsg::CreateContest {
                contest_info,
                amount,
                ..
            } => (contest_info, amount),
            _ => panic!("Expected CreateContest variant"),
        };

        let msg = QueryMsg::GetContest {
            contest_id: contest_info.id,
        };

        let execute_msg2 = _get_valid_bet_contest_msg();
        let added_amount = match execute_msg2 {
            ExecuteMsg::BetContest { amount, .. } => amount,
            _ => panic!("Expected CreateContest variant"),
        };

        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let contest_query_response: ContestQueryResponse = from_binary(&res).unwrap();

        assert_eq!(
            contest_info.options.len(),
            contest_query_response.contest_bet_summary.options.len()
        );
        assert_eq!(
            initial_amount.unwrap() + added_amount.unwrap(),
            contest_query_response.contest_bet_summary.calc_total_pool()
        );
        assert_eq!(
            initial_amount.unwrap() + added_amount.unwrap(),
            contest_query_response.contest_bet_summary.options[0].bet_allocation
        );
        assert_eq!(
            Uint128::from(0u128),
            contest_query_response.contest_bet_summary.options[1].bet_allocation
        );
    }
}
