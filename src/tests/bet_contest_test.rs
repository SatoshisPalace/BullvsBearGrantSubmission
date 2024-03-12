#[cfg(test)]

pub mod tests {
    use cosmwasm_std::{
        coins,
        testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
        Addr, Empty, Env, OwnedDeps, Response, StdResult, Timestamp, Uint128,
    };

    use crate::{
        answer::{ExecuteAnswer, ResponseStatus::Success},
        contract::execute_from_snip_20,
        msg::InvokeMsg,
        tests::{
            constants::FAR_IN_THE_FUTURE,
            contract_init_test::tests::_initialize_test,
            create_contest_test::tests::{_create_contest_test, _get_valid_create_contest_msg},
        },
    };

    ////////TESTS////////
    #[test]
    fn bet_on_contest_creation() {
        let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> =
            mock_dependencies();

        _initialize_test(&mut deps);

        let msg = _get_valid_create_contest_msg();

        _create_contest_test(&mut deps, msg);
    }

    #[test]
    fn bet_on_contest_after_creation() {
        let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> =
            mock_dependencies();

        _initialize_test(&mut deps);

        let msg = _get_valid_create_contest_msg();

        _create_contest_test(&mut deps, msg);

        _bet_contest_test(&mut deps)
    }

    #[test]
    fn bet_on_contest_after_time_of_close() {
        let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> =
            mock_dependencies();

        _initialize_test(&mut deps);

        let msg = _get_valid_create_contest_msg();

        _create_contest_test(&mut deps, msg);

        let mut env = mock_env();
        env.block.time = Timestamp::from_seconds(FAR_IN_THE_FUTURE);
        let msg = _get_valid_bet_contest_msg();
        let info = mock_info(env.contract.address.as_str(), &coins(1000, "earth"));
        let res = execute_from_snip_20(deps.as_mut(), env, info, msg);
        assert!(res.is_err(), "Expected an error but got {:?}", res);
    }

    #[test]
    fn bet_on_opposite_sides() {
        let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> =
            mock_dependencies();
        _initialize_test(&mut deps);

        let msg = _get_valid_create_contest_msg();

        _create_contest_test(&mut deps, msg);

        _bet_contest_opposite_sides_test(&mut deps)
    }

    ////////INNER TESTS////////

    pub fn _bet_contest_test(deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>) {
        let env = mock_env();
        let msg = _get_valid_bet_contest_msg();
        let info = mock_info(env.contract.address.as_str(), &coins(1000, "earth"));
        let res = execute_from_snip_20(deps.as_mut(), env, info, msg).unwrap();
        let expected =
            Response::default().set_data(ExecuteAnswer::BetContestAnswer { status: Success });
        assert_eq!(expected, res);
    }

    pub fn _bet_contest_test_with_sender(
        deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
        sender: &str,
    ) {
        let env = mock_env();
        let msg = _get_valid_bet_contest_msg(); // Ensure this msg is appropriate for the test
        let info = mock_info(sender, &coins(1000, "earth")); // Use the sender parameter
        let res = execute_from_snip_20(deps.as_mut(), env, info, msg).unwrap();
        let expected =
            Response::default().set_data(ExecuteAnswer::BetContestAnswer { status: Success });

        assert_eq!(expected, res);
    }

    pub fn _bet_contest_test_with_sender_outcome(
        deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
        sender: &str,
        contest_id: u32,
        outcome_id: u8,
        amount: Uint128,
    ) {
        let env = mock_env();
        let msg = InvokeMsg::BetContest {
            contest_id,
            outcome_id,
            user: Addr::unchecked(sender),
            amount: Some(amount),
        };
        let info = mock_info(sender, &coins(1000, "earth")); // Use the sender parameter
        let res = execute_from_snip_20(deps.as_mut(), env, info, msg).unwrap();
        let expected =
            Response::default().set_data(ExecuteAnswer::BetContestAnswer { status: Success });

        assert_eq!(expected, res);
    }

    pub fn _bet_contest_opposite_sides_test(
        deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
    ) {
        let contest_id = 1;
        let outcome_id = 2;
        let env = mock_env();
        let sender = Some(env.contract.address.clone());
        let amount = Some(Uint128::from(100u128)); // Replace with the actual amount you want to bet

        // Perform the bet using the helper function
        let res = _bet_contest(deps, env, contest_id, outcome_id, sender, amount.clone());

        // Check if the result is an error
        match res {
            Ok(_) => panic!("Expected an error but got success"),
            Err(e) => {
                assert!(
                    e.to_string().contains("409"),
                    "{}, does not contain 409",
                    e.to_string()
                )
            }
        }
    }

    /////////Helpers/////////

    pub fn _bet_contest(
        deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
        env: Env,
        contest_id: u32,
        outcome_id: u8,
        sender: Option<Addr>,
        amount: Option<Uint128>,
    ) -> StdResult<Response> {
        // Clone the sender to avoid the "use of moved value" error
        let sender_clone = sender.clone().unwrap().clone();

        // Create the ExecuteMsg
        let msg = InvokeMsg::BetContest {
            contest_id,
            outcome_id,
            user: sender.unwrap(),
            amount,
        };

        // Create the environment and info
        let info = mock_info(sender_clone.as_str(), &coins(1000, "earth"));

        // Execute the function
        execute_from_snip_20(deps.as_mut(), env, info, msg)
    }

    pub fn _get_valid_bet_contest_msg() -> InvokeMsg {
        let execute_msg = _get_valid_create_contest_msg();
        let (contest_info, outcome_id) = match execute_msg {
            InvokeMsg::CreateContest {
                contest_info,
                outcome_id,
                ..
            } => (contest_info, outcome_id),
            _ => panic!("Expected CreateContest variant"),
        };
        let env = mock_env();

        let msg = InvokeMsg::BetContest {
            contest_id: contest_info.id,
            outcome_id: outcome_id,
            user: env.contract.address,
            amount: Option::Some(Uint128::from(100u128)),
        };
        return msg;
    }
}
