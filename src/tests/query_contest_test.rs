#[cfg(test)]


pub mod tests{
	use cosmwasm_std::{testing::{mock_dependencies, mock_env, MockStorage, MockApi, MockQuerier}, OwnedDeps, Empty, from_binary};

	use crate::{msg::{ExecuteMsg, QueryMsg}, contract::query, contest::queries::ContestQueryResponse, tests::{create_contest_test::tests::{_get_invalid_create_contest_msg, _get_valid_create_contest_msg, _create_contest_test, _create_invalid_contest_test, _get_valid_contest_info}, contract_init_test::tests::_initialize_test}};
	
	////////TESTS////////
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
	pub fn _query_contest_test(
		deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
	){
		let msg = QueryMsg::GetContest { 
			contest_id: _get_valid_contest_info().id
		};

		let res = query(deps.as_ref(), mock_env(), msg).unwrap();
		let contest_query_response: ContestQueryResponse = from_binary(&res).unwrap();
		assert_eq!(0, contest_query_response.contest_info.id());
		assert_eq!(0, contest_query_response.contest_info.time_of_close());
		assert_eq!(0, contest_query_response.contest_info.time_of_resolve());
		assert_eq!(2, contest_query_response.contest_info.options().len());
	}

	pub fn _query_invalid_contest_test(deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>){
		let msg = QueryMsg::GetContest { 
			contest_id: 0 
		};

		let res = query(deps.as_ref(), mock_env(), msg);
		assert!(res.is_err());
	}
}