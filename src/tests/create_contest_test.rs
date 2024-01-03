#[cfg(test)]
pub mod tests {
    use cosmwasm_std::{
        coins,
        testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
        Empty, OwnedDeps, Response, StdResult, Uint128, Addr,
    };

    use crate::{
        answer::{ExecuteAnswer, ResponseStatus::Success},
        contest::data::contest_info::{ContestInfo, ContestOutcome},
        contract::execute_from_snip_20,
        msg::ExecuteMsg,
        tests::{
            constants::{FAR_IN_THE_FUTURE, FAR_IN_THE_PAST},
            contract_init_test::tests::_initialize_test,
        },
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

    #[test]
    fn create_contest_after_time_of_close() {
        let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> =
            mock_dependencies();

        _initialize_test(&mut deps);

        let msg = _get_past_create_contest_msg();

        let env = mock_env();
        let info = mock_info(env.contract.address.as_str(), &coins(1000, "earth"));
        let res = execute_from_snip_20(deps.as_mut(), env, info, msg);

        assert!(res.is_err(), "Expected an error but got {:?}", res);
    }
    #[test]
    fn create_contest_with_invalid_outcome_id() {
        let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> =
            mock_dependencies();

        _initialize_test(&mut deps);

        let invalid_contest_info = ContestInfo {
            id: 1,
            options: vec![
                ContestOutcome {
                    id: 0, // Invalid outcome ID
                    name: "option1".to_string(),
                },
                ContestOutcome {
                    id: 1,
                    name: "option2".to_string(),
                },
            ],
            event_details: "Example event details".to_string(),
            time_of_close: FAR_IN_THE_FUTURE,
            time_of_resolve: FAR_IN_THE_FUTURE,
        };

        let res = create_contest_with_params(&mut deps, invalid_contest_info, "cbda1e8b4c5f0d326989c850a56aad7d0712b54b0b9acf2d9df3bde22de0c6627db8380ff2281e8881886c00b1d41336ca3a82426dcc5d929bcefec85ae604df");

        match res {
            Ok(_) => panic!("Expected an error but got success"),
            Err(e) => assert!(
                e.to_string().contains("412"),
                "Error message does not contain '412': {:?}",
                e
            ),
        }
    }

    ////////INNER TESTS////////
    pub fn create_contest_with_params(
        deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
        contest_info: ContestInfo,
        contest_info_signature_hex: &str,
    ) -> StdResult<Response> {
        let msg = ExecuteMsg::CreateContest {
            contest_info,
            contest_info_signature_hex: contest_info_signature_hex.to_string(),
            outcome_id: 1,
            sender: Some(mock_env().contract.address),
            amount: Some(Uint128::from(100u128)),
        };
        let env = mock_env();
        let info = mock_info(env.contract.address.as_str(), &coins(1000, "earth"));

        execute_from_snip_20(deps.as_mut(), env, info, msg)
    }

    pub fn _create_contest_with_sender_test(
        deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
        msg: ExecuteMsg,
        sender: &str,
    ) {
        let env = mock_env();
        let info = mock_info(sender, &coins(1000, "earth"));
        let res = execute_from_snip_20(deps.as_mut(), env, info, msg).unwrap();
        let expected =
            Response::default().set_data(ExecuteAnswer::CreateContestAnswer { status: Success });
        assert_eq!(expected, res);
    }


    pub fn _create_contest_test(
        deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
        msg: ExecuteMsg,
    ) {
        let env = mock_env();
        let info = mock_info(env.contract.address.as_str(), &coins(1000, "earth"));
        let res = execute_from_snip_20(deps.as_mut(), env, info, msg).unwrap();
        let expected =
            Response::default().set_data(ExecuteAnswer::CreateContestAnswer { status: Success });
        assert_eq!(expected, res);
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
						id: 1,
						name: "option1".to_string(),
					},
					ContestOutcome {
						id: 2,
						name: "option2".to_string(),
					}
				],
                event_details: "Example event details".to_string(),
				time_of_close: FAR_IN_THE_FUTURE,
				time_of_resolve: FAR_IN_THE_FUTURE,
			},
			contest_info_signature_hex: "c59576d467bc77be37b5b1d74e4a3fc056f7642746964a1bd8fb897955458d2c6c8801b4017b9d09c8ceb77356002c0f2a0ce425cb830cb7305bd4ab1ae4c261".to_string(),
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
			contest_info_signature_hex: "b5876a3fc9f0ff470fd2d5d446dbdac994486eb2c7db61ebc2bd6e96a5fb05f7773b3eb0e59d1a4dc80e317e4e69bb6bbb7635084c29dd1bedeabcd4544a9d40".to_string(),
			outcome_id: 1,
			sender: Option::Some(env.contract.address), //TODO fix
			amount: Option::Some(Uint128::from(100u128)), //TODO fix
		};
        return msg;
    }

    pub fn _get_valid_create_contest_msg_with_params(sender: &str, amount: Uint128) -> ExecuteMsg {
        let msg = ExecuteMsg::CreateContest {
			contest_info: _get_valid_contest_info(),
			contest_info_signature_hex: "b5876a3fc9f0ff470fd2d5d446dbdac994486eb2c7db61ebc2bd6e96a5fb05f7773b3eb0e59d1a4dc80e317e4e69bb6bbb7635084c29dd1bedeabcd4544a9d40".to_string(),
			outcome_id: 1,
			sender: Option::Some(Addr::unchecked(sender)), //TODO fix
			amount: Option::Some(amount), //TODO fix
		};
        return msg;
    }

    pub fn _get_past_create_contest_msg() -> ExecuteMsg {
        let env = mock_env();

        let msg = ExecuteMsg::CreateContest {
			contest_info: _get_past_contest_info(),
			contest_info_signature_hex: "b92ebd05884ed1871e03a720b843cca606c041dd47d48334968c83ed307f5ad14c60e7420ef1d27a35702ae5d31eda329f0a978829b88513589e32c1fa28eee6".to_string(),
			outcome_id: 0,
			sender: Option::Some(env.contract.address), //TODO fix
			amount: Option::Some(Uint128::from(100u128)), //TODO fix
		};
        return msg;
    }

    pub fn _get_past_contest_info() -> ContestInfo {
        return ContestInfo {
            id: 2,
            options: vec![
                ContestOutcome {
                    id: 1,
                    name: "option1".to_string(),
                },
                ContestOutcome {
                    id: 2,
                    name: "option2".to_string(),
                },
            ],
            event_details: "Example event details".to_string(),
            time_of_close: FAR_IN_THE_PAST,
            time_of_resolve: FAR_IN_THE_PAST,
        };
    }

    pub fn _get_valid_contest_info() -> ContestInfo {
        return ContestInfo {
            id: 1,
            options: vec![
                ContestOutcome {
                    id: 1,
                    name: "option1".to_string(),
                },
                ContestOutcome {
                    id: 2,
                    name: "option2".to_string(),
                },
            ],
            event_details: "Example event details".to_string(),
            time_of_close: FAR_IN_THE_FUTURE,
            time_of_resolve: FAR_IN_THE_FUTURE,
        };
    }
}
