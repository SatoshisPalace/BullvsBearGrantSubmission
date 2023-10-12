#[cfg(test)]

pub mod tests {
    use cosmwasm_std::{
        coins,
        testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
        Addr, Empty, OwnedDeps, Response, StdResult, Uint128, Timestamp,
    };

    use crate::{
        contract::execute_from_snip_20,
        msg::ExecuteMsg,
        tests::{
            contract_init_test::tests::_initialize_test,
            create_contest_test::tests::{_create_contest_test, _get_valid_create_contest_msg}, constants::FAR_IN_THE_FUTURE,
        },
    };

    ////////TESTS////////
    #[test]
    fn bet_on_contest_creation() {
        let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> =
            mock_dependencies();

        _initialize_test(&mut deps);

        let msg: ExecuteMsg = _get_valid_create_contest_msg();

        _create_contest_test(&mut deps, msg);
    }

    #[test]
    fn bet_on_contest_after_creation() {
        let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> =
            mock_dependencies();

        _initialize_test(&mut deps);

        let msg: ExecuteMsg = _get_valid_create_contest_msg();

        _create_contest_test(&mut deps, msg);

        _bet_contest_test(&mut deps)
    }

    #[test]
    fn bet_on_contest_after_time_of_close() {
        let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> =
            mock_dependencies();

        _initialize_test(&mut deps);

        let msg: ExecuteMsg = _get_valid_create_contest_msg();

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

        let msg: ExecuteMsg = _get_valid_create_contest_msg();

        _create_contest_test(&mut deps, msg);

        _bet_contest_opposite_sides_test(&mut deps)
    }

    ////////INNER TESTS////////

    pub fn _bet_contest_test(deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>) {
        let env = mock_env();
        let msg = _get_valid_bet_contest_msg();
        let info = mock_info(env.contract.address.as_str(), &coins(1000, "earth"));
        let res = execute_from_snip_20(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(Response::default(), res);
    }

    pub fn _bet_contest_opposite_sides_test(
        deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
    ) {
        let contest_id = 1;
        let outcome_id = 1;
        let env = mock_env();
        let sender = Some(env.contract.address.clone());
        let amount = Some(Uint128::from(100u128)); // Replace with the actual amount you want to bet

        // Perform the bet using the helper function
        let res = _bet_contest(deps, contest_id, outcome_id, sender.clone(), amount.clone());

        // Check if the result is an error
        assert!(res.is_err(), "Expected an error but got a result");
    }

    /////////Helpers/////////

    pub fn _bet_contest(
        deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
        contest_id: u32,
        outcome_id: u8,
        sender: Option<Addr>,
        amount: Option<Uint128>,
    ) -> StdResult<Response> {
        // Clone the sender to avoid the "use of moved value" error
        let sender_clone = sender.clone();

        // Create the ExecuteMsg
        let msg = ExecuteMsg::BetContest {
            contest_id,
            outcome_id,
            sender,
            amount,
        };

        // Create the environment and info
        let env = mock_env();
        let info = mock_info(sender_clone.unwrap().as_str(), &coins(1000, "earth"));

        // Execute the function
        execute_from_snip_20(deps.as_mut(), env, info, msg)
    }

    pub fn _get_valid_bet_contest_msg() -> ExecuteMsg {
        let execute_msg = _get_valid_create_contest_msg();
        let (contest_info, outcome_id) = match execute_msg {
            ExecuteMsg::CreateContest {
                contest_info,
                outcome_id,
                ..
            } => (contest_info, outcome_id),
            _ => panic!("Expected CreateContest variant"),
        };
        let env = mock_env();

        let msg = ExecuteMsg::BetContest {
            contest_id: contest_info.id,
            outcome_id: outcome_id,
            sender: Option::Some(env.contract.address),
            amount: Option::Some(Uint128::from(100u128)),
        };
        return msg;
    }
}
