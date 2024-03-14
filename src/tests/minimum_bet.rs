#[cfg(test)]
pub mod tests {
    use std::borrow::BorrowMut;

    use cosmwasm_std::{
        coins, from_binary,
        testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
        Addr, Empty, OwnedDeps, Uint128,
    };

    use crate::{
        contest::response::MinimumBetResponse,
        contract::{execute, query},
        msg::{ExecuteMsg, InvokeMsg, QueryMsg},
        tests::{
            bet_contest_test::tests::{_bet_contest, _bet_contest_test_with_sender_outcome},
            contract_init_test::tests::_initialize_test,
            create_contest_test::tests::{
                _create_contest_with_sender_test, _get_valid_create_contest_msg_with_params,
            },
        },
    };

    ////////TESTS////////
    #[test]
    fn min_bet_on_init() {
        let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> =
            mock_dependencies();
        _initialize_test(&mut deps);
        _check_min_bet_after_instantiation_test(&mut deps);
    }

    #[test]
    fn min_bet_on_set() {
        let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> =
            mock_dependencies();
        _initialize_test(&mut deps);
        _set_min_bet_test(&mut deps, Uint128::from(123u128));
    }

    #[test]
    fn bet_more_than_min() {
        let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> =
            mock_dependencies();
        _initialize_test(&mut deps);
        _set_min_bet_test(&mut deps, Uint128::from(123u128));

        let address = &"better";

        let create_contest_msg =
            _get_valid_create_contest_msg_with_params(address, Uint128::from(1000u128));
        let binding = create_contest_msg.clone();
        let (contest_info, _, _) = match &binding {
            InvokeMsg::CreateContest {
                contest_info,
                outcome_id,
                amount,
                ..
            } => (contest_info, outcome_id, amount.unwrap()),
            _ => panic!("Expected CreateContest message"),
        };

        _create_contest_with_sender_test(&mut deps, create_contest_msg, address);

        _bet_contest_test_with_sender_outcome(
            &mut deps,
            address,
            contest_info.id,
            1,
            Uint128::from(1000u128),
        );
    }

    #[test]
    fn bet_less_than_min() {
        let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> =
            mock_dependencies();
        _initialize_test(&mut deps);
        _set_min_bet_test(&mut deps, Uint128::from(123u128));

        let address = "better1";

        let create_contest_msg =
            _get_valid_create_contest_msg_with_params(address, Uint128::from(1000u128));
        let binding = create_contest_msg.clone();
        let (contest_info, _, _) = match &binding {
            InvokeMsg::CreateContest {
                contest_info,
                outcome_id,
                amount,
                ..
            } => (contest_info, outcome_id, amount.unwrap()),
            _ => panic!("Expected CreateContest message"),
        };

        _create_contest_with_sender_test(&mut deps, create_contest_msg, address);
        let env = mock_env();

        let res = _bet_contest(
            deps.borrow_mut(),
            env,
            contest_info.id(),
            2,
            Some(Addr::unchecked("better")),
            Some(Uint128::from(1u128)),
        );
        assert!(res.is_err())
    }

    pub fn _check_min_bet_after_instantiation_test(
        deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
    ) {
        let query_msg = QueryMsg::GetMinBet {};
        let response_binary = query(deps.as_ref(), mock_env(), query_msg).unwrap();
        let response: MinimumBetResponse = from_binary(&response_binary).unwrap();
        assert_eq!(response.minimum_bet, Uint128::from(1u128))
    }

    pub fn _set_min_bet_test(
        deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
        actual: Uint128,
    ) {
        let execute_msg = ExecuteMsg::SetMinBet { amount: actual };
        let info = mock_info("creator", &coins(1000, "earth"));
        let execute_response = execute(deps.as_mut(), mock_env(), info, execute_msg);
        // print!("{}", execute_response.);
        assert!(execute_response.is_ok());

        let query_msg = QueryMsg::GetMinBet {};
        let response_binary = query(deps.as_ref(), mock_env(), query_msg).unwrap();
        let response: MinimumBetResponse = from_binary(&response_binary).unwrap();
        assert_eq!(response.minimum_bet, actual)
    }
}
