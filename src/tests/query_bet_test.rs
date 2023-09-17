#[cfg(test)]
pub mod tests {
    use cosmwasm_std::{
        from_binary,
        testing::{mock_dependencies, mock_env, MockApi, MockQuerier, MockStorage},
        Addr, Binary, Empty, OwnedDeps, StdResult, Uint128,
    };

    use crate::{
        contest::{data::bets::UserContest, response::UserBetQueryResponse},
        contract::query,
        msg::{ExecuteMsg, QueryMsg},
        tests::{
            bet_contest_test::tests::{_bet_contest_test, _get_valid_bet_contest_msg},
            contract_init_test::tests::_initialize_test,
            create_contest_test::tests::{_create_contest_test, _get_valid_create_contest_msg},
            viewing_key_test::tests::_set_viewing_key_test,
        },
    };

    ////////TESTS////////
    #[test]
    fn query_bet_after_contest_creation() {
        let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> =
            mock_dependencies();

        _initialize_test(&mut deps);

        let mut msg = _get_valid_create_contest_msg();
        _create_contest_test(&mut deps, msg);

        msg = _get_valid_create_contest_msg();

        let viewing_key = "api_key_/WMIRnqFsFmb6KuvRSX8LQGSz3umCjcXcptco4gl3Lg=";
        _set_viewing_key_test(&mut deps, viewing_key);

        if let ExecuteMsg::CreateContest {
            contest_info,
            contest_info_signature_hex: _,
            outcome_id: _,
            sender: _,
            amount,
        } = msg
        {
            _query_bet_test(&mut deps, contest_info.id, viewing_key, amount.unwrap())
        } else {
            panic!("This isnt supposed to happen")
        }
    }

    #[test]
    fn query_bet_twice() {
        // Initialize Contract
        let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> =
            mock_dependencies();
        _initialize_test(&mut deps);

        // Create Contest
        let msg = _get_valid_create_contest_msg();
        _create_contest_test(&mut deps, msg);

        // Place Additional bet on contest
        _bet_contest_test(&mut deps);

        // Set Viewing Key
        let viewing_key = "api_key_/WMIRnqFsFmb6KuvRSX8LQGSz3umCjcXcptco4gl3Lg=";
        _set_viewing_key_test(&mut deps, viewing_key);

        _query_bet_after_betting_twice_test(&mut deps, viewing_key)
    }

    #[test]
    fn query_bet_bad_viewing_key() {
        let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> =
            mock_dependencies();

        _initialize_test(&mut deps);

        let mut msg = _get_valid_create_contest_msg();
        _create_contest_test(&mut deps, msg);

        msg = _get_valid_create_contest_msg();

        let viewing_key = "api_key_/WMIRnqFsFmb6KuvRSX8LQGSz3umCjcXcptco4gl3Lg=";
        _set_viewing_key_test(&mut deps, viewing_key);

        if let ExecuteMsg::CreateContest {
            contest_info,
            contest_info_signature_hex: _,
            outcome_id: _,
            sender: _,
            amount: _,
        } = msg
        {
            _query_bet_test_invalid_viewing_key(&mut deps, contest_info.id)
        } else {
            panic!("This isnt supposed to happen")
        }
    }

    ////////INNER TESTS////////
    fn _query_bet_after_betting_twice_test(
        deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
        viewing_key: &str,
    ) {
        let create_contest_msg = _get_valid_create_contest_msg();
        let bet_contest_msg = _get_valid_bet_contest_msg();
        // Extract the bet amounts from the messages
        let (first_bet, contest_id) = if let ExecuteMsg::CreateContest {
            contest_info,
            amount,
            ..
        } = create_contest_msg
        {
            (amount.unwrap_or(Uint128::from(0u128)), contest_info.id)
        } else {
            panic!("Expected CreateContest message");
        };

        let second_bet = if let ExecuteMsg::BetContest { amount, .. } = bet_contest_msg {
            amount.unwrap()
        } else {
            panic!("Expected BetContest message");
        };

        let expected_amount = first_bet + second_bet;
        _query_bet_test(deps, contest_id, viewing_key, expected_amount)
    }

    fn _query_bet_test_invalid_viewing_key(
        deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
        contest_id: u32,
    ) {
        let viewing_key = "api_key_literally_not_a_viewing_key";

        let env = mock_env();
        let msg = _get_query_bet_msg(env.contract.address.as_str(), contest_id, viewing_key);

        // Execute the query and get the Binary result
        let res: StdResult<Binary> = query(deps.as_ref(), env, msg);
        // let binary_data = res.unwrap();

        // Check if the result is an error
        assert!(res.is_err(), "Expected an error but got a result");
    }

    fn _query_bet_test(
        deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
        contest_id: u32,
        viewing_key: &str,
        expected_bet: Uint128,
    ) {
        let env = mock_env();
        let msg = _get_query_bet_msg(env.contract.address.as_str(), contest_id, viewing_key);

        // Execute the query and get the Binary result
        let res: StdResult<Binary> = query(deps.as_ref(), env, msg);
        let binary_data = res.unwrap();

        // Deserialize the Binary data into your specific type
        let response_data: UserBetQueryResponse = from_binary(&binary_data).unwrap();

        // Compare the bet
        assert_eq!(response_data.bet.amount, expected_bet); // Assuming `bet` has a field `amount` of type Uint128
    }

    /////////Helpers/////////

    fn _get_query_bet_msg(address: &str, contest_id: u32, viewing_key: &str) -> QueryMsg {
        // Create a UserContest object
        let user_contest = UserContest {
            address: Addr::unchecked(address), // Create an Addr from the provided string
            contest_id,
        };

        // Create a GetUserBet message
        QueryMsg::GetUserBet {
            user_contest,
            key: viewing_key.to_string(),
        }
    }
}
