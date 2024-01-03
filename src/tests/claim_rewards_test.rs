#[cfg(test)]
pub mod tests {
    use cosmwasm_std::{
        coins, from_binary,
        testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
        Binary, CosmosMsg, Empty, OwnedDeps, Response, Timestamp, WasmMsg,
    };

    use crate::{
        contract::execute,
        integrations::{
            oracle::oracle::{
                assert_query_contest_result_call_count, reset_query_contest_result_call_count,
                set_mock_result,
            },
            snip_20::{
                snip_20_msg::Snip20Msg, tests::snip_20_test::tests::_register_fake_snip20_test,
            },
        },
        msg::ExecuteMsg,
        tests::{
            bet_contest_test::tests::{_bet_contest_test, _bet_contest_test_with_sender},
            constants::FAR_IN_THE_FUTURE,
            contract_init_test::tests::_initialize_test,
            create_contest_test::tests::{_create_contest_test, _get_valid_create_contest_msg},
            query_contest_summary::tests::calculate_user_share,
        },
    };

    ////////TESTS////////
    #[test]
    fn claim_rewards_before_time_of_resolve() {
        let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> =
            mock_dependencies();

        // Initialize and create a contest
        _initialize_test(&mut deps);
        let create_contest_msg: ExecuteMsg = _get_valid_create_contest_msg();
        let contest_info = match &create_contest_msg {
            ExecuteMsg::CreateContest { contest_info, .. } => contest_info,
            _ => panic!("Expected CreateContest message"),
        };
        let contest_id = contest_info.id; // Extract the contest_id		_create_contest_test(&mut deps, create_contest_msg);
        _create_contest_test(&mut deps, create_contest_msg);

        // Bet on the contest
        _bet_contest_test(&mut deps);

        // Try to claim rewards before time_of_resolve
        claim_rewards_before_time_of_resolve_test(&mut deps, contest_id);
    }

    #[test]
    fn test_query_contest_result_called_once() {
        let mut deps = mock_dependencies();

        // Initialize and create a contest
        _initialize_test(&mut deps);
        let create_contest_msg: ExecuteMsg = _get_valid_create_contest_msg();

        // Extract contest_id from the create_contest_msg
        let contest_id = if let ExecuteMsg::CreateContest { contest_info, .. } = &create_contest_msg
        {
            contest_info.id
        } else {
            panic!("Expected CreateContest message")
        };
        _create_contest_test(&mut deps, create_contest_msg);
        // Two different users place bets
        _bet_contest_test_with_sender(&mut deps, "user1");
        _bet_contest_test_with_sender(&mut deps, "user2");

        // User1 tries to claim rewards - query_contest_result should be called
        first_time_claim_test(&mut deps, contest_id, "user1");

        // User2 tries to claim rewards - query_contest_result should NOT be called again
        subsequent_claim_test(&mut deps, contest_id, "user2");

        // Assertions to confirm query_contest_result was called only once
    }

    #[test]
    fn claim_contest_after_nullification() {
        let mut deps = mock_dependencies();
        _initialize_test(&mut deps);
        // Initialize and create a contest
        let create_contest_msg: ExecuteMsg = _get_valid_create_contest_msg();
        // Extract contest_id from the create_contest_msg
        let (contest_id, amount_bet, sender) = if let ExecuteMsg::CreateContest {
            contest_info,
            amount,
            sender,
            ..
        } = &create_contest_msg
        {
            (contest_info.id, amount.unwrap_or_default(), sender.clone())
        } else {
            panic!("Expected CreateContest message with contest_info, amount, and sender")
        };
        _register_fake_snip20_test(&mut deps);
        // Assume _create_contest_test creates the contest and returns the contest_id
        _create_contest_test(&mut deps, create_contest_msg.clone());

        // Set oracle result to 0 (nullified)
        set_mock_result(0);

        // Define a function for the user to claim after nullification
        let sender_address = sender.map_or_else(
            || panic!("Expected sender address"),
            |addr| addr.to_string(),
        );
        claim_after_nullification_test(&mut deps, contest_id, &sender_address, amount_bet.u128());

        // Additional assertions and cleanup can be performed here
    }

    ////////INNER TESTS////////
    pub fn claim_after_nullification_test(
        deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
        contest_id: u32,
        sender: &str,
        amount_bet: u128, // Added parameter for the bet amount
    ) {
        reset_query_contest_result_call_count(); // Reset call count before the test

        let mut env = mock_env();
        env.block.time = Timestamp::from_seconds(FAR_IN_THE_FUTURE);
        let claim_msg = ExecuteMsg::Claim { contest_id };
        let info = mock_info(sender, &coins(amount_bet, "earth")); // Using amount_bet here

        // Attempt to execute claim action
        let response = execute(deps.as_mut(), env, info, claim_msg).unwrap();

        // Assert that query_contest_result was called once
        assert_query_contest_result_call_count(1);

        // Obtain the actual result from contains_snip20_send_msg
        let result = contains_snip20_send_msg(&response, sender, amount_bet);

        // Capture details for the error message before the assertion
        let (actual_recipient, actual_amount) = result.clone().unwrap_or_default();

        // Assert that there is a SNIP-20 send message in the response
        assert!(
            result.is_none(),
            "Expected SNIP-20 send message to '{}' with amount {}, but found message to '{}' with amount {}.",
            sender, amount_bet, actual_recipient, actual_amount
        );
    }

    pub fn winning_claim_test(
        deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
        contest_id: u32,
        sender: &str,
        amount_bet: u128,
        winning_result: u8, // The result indicating a win in the mock environment
    ) {
        reset_query_contest_result_call_count(); // Reset call count before the test

        let mut env = mock_env();
        env.block.time = Timestamp::from_seconds(FAR_IN_THE_FUTURE);
        let claim_msg = ExecuteMsg::Claim { contest_id };
        let info = mock_info(sender, &coins(amount_bet, "earth"));

        // Set up contest state to reflect a winning position for the user
        set_mock_result(winning_result); // Set the contest to a winning outcome

        // Execute the claim action
        let response = execute(deps.as_mut(), env, info, claim_msg).unwrap();

        // Query the total pool and calculate the winnings
        let user_share = calculate_user_share(
            deps,
            contest_id,
            amount_bet,
            winning_result, // Ensure you have this information
        ).unwrap();        // Adjusted winning claim test example:
        print!("USER SHARE{}", user_share);
        // Adjusted winning claim test example:
        let result = contains_snip20_send_msg(&response, sender, user_share);
        let (actual_recipient, actual_amount) = result.clone().unwrap_or_default();
        assert!(
            result.is_none(),
            "Expected SNIP-20 send message to '{}' with amount {}, but found message to '{}' with amount {}.",
            sender, user_share, actual_recipient, actual_amount
        );
    }

    pub fn losing_claim_test(
        deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
        contest_id: u32,
        sender: &str,
        amount_bet: u128,
        losing_result: u8,
    ) {
        reset_query_contest_result_call_count(); // Reset call count before the test
    
        let mut env = mock_env();
        env.block.time = Timestamp::from_seconds(FAR_IN_THE_FUTURE);
        let claim_msg = ExecuteMsg::Claim { contest_id };
        let info = mock_info(sender, &coins(amount_bet, "earth"));
    
        // Set up contest state to reflect a losing position for the user
        set_mock_result(losing_result); // Set the contest to a losing outcome
    
        // Execute the claim action and expect it to fail
        let response = execute(deps.as_mut(), env, info, claim_msg);
    
        // Check the response for an error containing the specific error text
        match response {
            Err(err) => {
                // Check that the error message contains the expected error code or text
                let error_msg = err.to_string();
                assert!(
                    error_msg.contains("403"),
                    "Expected a '403 Forbidden' error for claiming on a lost contest, but got: {}",
                    error_msg
                );
            }
            _ => panic!("Expected an error for claiming on a lost contest, but claim was successful."),
        }
    }
    
    

    fn contains_snip20_send_msg(
        response: &Response,
        recipient: &str,
        expected_amount: u128,
    ) -> Option<(String, u128)> {
        for submsg in response.messages.iter() {
            if let CosmosMsg::Wasm(WasmMsg::Execute { msg, .. }) = &submsg.msg {
                if let Some((actual_recipient, actual_amount)) = is_send_msg(msg) {
                    // Check if recipient and amount match the expected values
                    if recipient == actual_recipient && expected_amount == actual_amount {
                        return None; // Correct message found; return None indicating no error
                    } else {
                        return Some((actual_recipient, actual_amount)); // Incorrect message; return details
                    }
                }
            }
        }
        Some(("No recipient found".to_string(), 0)) // No message found; return default error details
    }

    fn is_send_msg(msg: &Binary) -> Option<(String, u128)> {
        // Attempt to deserialize the Binary message into Snip20Msg
        if let Ok(Snip20Msg::Send {
            recipient, amount, ..
        }) = from_binary(msg)
        {
            // Return recipient and amount
            return Some((recipient, amount.u128()));
        }
        None // Return None if message is not of type Send or deserialization fails
    }

    fn first_time_claim_test(
        deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
        contest_id: u32,
        sender: &str,
    ) {
        reset_query_contest_result_call_count(); // Reset the call count before the test

        let mut env = mock_env();
        env.block.time = Timestamp::from_seconds(FAR_IN_THE_FUTURE);
        let claim_msg = ExecuteMsg::Claim { contest_id };
        let info = mock_info(sender, &coins(1000, "earth"));
        let _ = execute(deps.as_mut(), env, info, claim_msg);

        // Assert that query_contest_result was called once
        assert_query_contest_result_call_count(1);
    }

    fn subsequent_claim_test(
        deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
        contest_id: u32,
        sender: &str,
    ) {
        let env = mock_env();
        let claim_msg = ExecuteMsg::Claim { contest_id };
        let info = mock_info(sender, &coins(1000, "earth"));

        let _ = execute(deps.as_mut(), env, info, claim_msg);

        // Assert that query_contest_result was NOT called again (still only once)
        assert_query_contest_result_call_count(1);
    }

    fn claim_rewards_before_time_of_resolve_test(
        deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
        contest_id: u32,
    ) {
        let env = mock_env();
        let claim_msg = ExecuteMsg::Claim { contest_id };
        let info = mock_info(env.contract.address.as_str(), &coins(1000, "earth"));

        // Expect an error when claiming before time_of_resolve
        let res = execute(deps.as_mut(), env.clone(), info.clone(), claim_msg);
        assert!(res.is_err(), "Expected an error but got {:?}", res);
    }
}
