#[cfg(test)]


pub mod tests{
	use cosmwasm_std::{testing::{mock_dependencies, mock_env, mock_info, MockStorage, MockApi, MockQuerier}, Addr, coins, OwnedDeps, Empty, Response, from_binary};

	use crate::{msg::{InstantiateMsg, ExecuteMsg, QueryMsg}, contract::{instantiate, query, execute}, contest::contest_info::{ContestInfo, ContestOutcome}, bet::bet::Bet};
	
	////////TESTS////////
	#[test]
	fn initialize(){
		let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> = mock_dependencies();
		_initialize_test(&mut deps);
	}
	
	#[test]
	fn create_contest(){
		let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> = mock_dependencies();

		_initialize_test(&mut deps);

		let msg = _get_valid_create_contest_msg();

		_create_contest_test(&mut deps, msg);
	}

	#[test]
	fn create_invalid_contest(){
		let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> = mock_dependencies();

		_initialize_test(&mut deps);

		let msg = _get_invalid_create_contest_msg();

		_create_invalid_contest_test(&mut deps, msg)
	}

	#[test]
	fn query_contest(){
		let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> = mock_dependencies();

		_initialize_test(&mut deps);

		let msg: ExecuteMsg = _get_valid_create_contest_msg();

		_create_contest_test(&mut deps, msg);

		_query_contest_test(&mut deps);
	}

	#[test]
	fn query_invalid_contest(){
		let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> = mock_dependencies();

		_initialize_test(&mut deps);

		let msg = _get_invalid_create_contest_msg();

		_create_invalid_contest_test(&mut deps, msg);

		_query_invalid_contest_test(&mut deps);
	}

	////////INNER TESTS////////
	fn _query_contest_test(
		deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
	){
		let msg = QueryMsg::GetContest { 
			contest_id: _get_valid_contest_info().id()
		};

		let res = query(deps.as_ref(), mock_env(), msg).unwrap();
		let contest_info: ContestInfo = from_binary(&res).unwrap();
		assert_eq!(0, contest_info.id());
		assert_eq!(0, contest_info.time_of_close());
		assert_eq!(0, contest_info.time_of_resolve());
		assert_eq!(2, contest_info.options().len())
	}

	fn _query_invalid_contest_test(deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>){
		let msg = QueryMsg::GetContest { 
			contest_id: 0 
		};

		let res = query(deps.as_ref(), mock_env(), msg);
		assert!(res.is_err());
	}

	fn _create_contest_test(
		deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
		msg: ExecuteMsg
	){
		let info = mock_info("creator", &coins(1000, "earth"));
		let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
		assert_eq!(Response::default(), res);
	}	

	fn _create_invalid_contest_test(
		deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
		msg: ExecuteMsg
	){
		let info = mock_info("creator", &coins(1000, "earth"));
		let res = execute(deps.as_mut(), mock_env(), info, msg);
		assert!(res.is_err());
	}	

	fn _initialize_test(deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>){
        let msg = InstantiateMsg { 
			satoshis_palace: Addr::unchecked("04eec6a876668ffb7031f9b9ade7c0c4bc47681ac27fec532bfd5c94fb3dd71d675a363d7036ba8d831a499b12e4f04c8741b90e3c4f3c6b64dd1104132d49498c"),
			oracle_contract: Addr::unchecked("TODO FIXME WHEN INTEGRATING WITH ORACLE")
		};

        let info = mock_info("creator", &coins(1000, "earth"));

        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
	}

	////////HELPERS////////
	fn _get_valid_contest_info() -> ContestInfo{
		return ContestInfo{
			id: 0,
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
		}
	}
	fn _get_valid_create_contest_msg() -> ExecuteMsg {
		let msg = ExecuteMsg::CreateContest {
			contest_info: _get_valid_contest_info(),
			contest_info_signature_hex: "ccf5c5b987455453eaddc62ce5b8e64877ea4f14500a7bdcce594e4b79303ceb29c5c9038e70177005b61cb6fbb486e7b22b76831da46c34e42f77909f0310f5".to_string(),
			users_bet: Bet{
				value: 0,
				option: 0,
			},
		};
		return msg
	}

	fn _get_invalid_create_contest_msg() -> ExecuteMsg {
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
			users_bet: Bet{
				value: 0,
				option: 0,
			},
		};
		return msg
	}


}