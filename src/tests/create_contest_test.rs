#[cfg(test)]
pub mod tests {
    use cosmwasm_std::{
        coins,
        testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
        Empty, OwnedDeps, Response, Uint128,
    };

    use crate::{
        contest::data::contest_info::{ContestInfo, ContestOutcome},
        contract::execute_from_snip_20,
        msg::ExecuteMsg,
        tests::contract_init_test::tests::_initialize_test,
    };

    ////////TESTS////////
    #[test]
    fn create_contest() {
        let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> =
            mock_dependencies();

        _initialize_test(&mut deps);

        let msg = _get_valid_create_contest_msg();

        _create_contest_test(&mut deps, msg);
    }

    #[test]
    fn create_invalid_contest() {
        let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> =
            mock_dependencies();

        _initialize_test(&mut deps);

        let msg = _get_invalid_create_contest_msg();

        _create_invalid_contest_test(&mut deps, msg)
    }

    #[test]
    fn create_contest_twice() {
        let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> =
            mock_dependencies();

        _initialize_test(&mut deps);

        let msg = _get_valid_create_contest_msg();

        _create_contest_test(&mut deps, msg);

        let same_msg_but_again: ExecuteMsg = _get_valid_create_contest_msg();

        _create_contest_for_the_second_time_test(&mut deps, same_msg_but_again);
    }

    ////////INNER TESTS////////

    pub fn _create_contest_test(
        deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
        msg: ExecuteMsg,
    ) {
        let env = mock_env();
        let info = mock_info(env.contract.address.as_str(), &coins(1000, "earth"));
        let res = execute_from_snip_20(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(Response::default(), res);
    }

    pub fn _create_contest_for_the_second_time_test(
        deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
        msg: ExecuteMsg,
    ) {
        let env = mock_env();
        let info = mock_info(env.contract.address.as_str(), &coins(1000, "earth"));
        let res = execute_from_snip_20(deps.as_mut(), env, info, msg);
        assert!(res.is_err());
    }

    pub fn _create_invalid_contest_test(
        deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
        msg: ExecuteMsg,
    ) {
        let info = mock_info("creator", &coins(1000, "earth"));
        let res = execute_from_snip_20(deps.as_mut(), mock_env(), info, msg);
        assert!(res.is_err());
    }

    ///////Helpers/////////
    pub fn _get_invalid_create_contest_msg() -> ExecuteMsg {
        let msg = ExecuteMsg::CreateContest {
			contest_info: ContestInfo{
				id: 1,
				options: vec![
					ContestOutcome {
						id: 0,
						name: "option1".to_string(),
					},
					ContestOutcome {
						id: 1,
						name: "option2".to_string(),
					}
				],
				time_of_close: 0,
				time_of_resolve: 0,
			},
			contest_info_signature_hex: "ccf5c5b987455453eaddc62ce5b8e64877ea4f14500a7bdcce594e4b79303ceb29c5c9038e70177005b61cb6fbb486e7b22b76831da46c34e42f77909f0310f5".to_string(),
			outcome_id: 0,
			sender: Option::None,//TODO fix
			amount: Option::None,//TODO fix
		};
        return msg;
    }

    pub fn _get_valid_create_contest_msg() -> ExecuteMsg {
        let env = mock_env();

        let msg = ExecuteMsg::CreateContest {
			contest_info: _get_valid_contest_info(),
			contest_info_signature_hex: "ccf5c5b987455453eaddc62ce5b8e64877ea4f14500a7bdcce594e4b79303ceb29c5c9038e70177005b61cb6fbb486e7b22b76831da46c34e42f77909f0310f5".to_string(),
			outcome_id: 0,
			sender: Option::Some(env.contract.address), //TODO fix
			amount: Option::Some(Uint128::from(100u128)), //TODO fix
		};
        return msg;
    }

    pub fn _get_valid_contest_info() -> ContestInfo {
        return ContestInfo {
            id: 0,
            options: vec![
                ContestOutcome {
                    id: 0,
                    name: "option1".to_string(),
                },
                ContestOutcome {
                    id: 1,
                    name: "option2".to_string(),
                },
            ],
            time_of_close: 0,
            time_of_resolve: 0,
        };
    }
}
