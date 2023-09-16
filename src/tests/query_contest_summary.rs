#[cfg(test)]
pub mod tests{
	use cosmwasm_std::{testing::{mock_env, MockStorage, MockApi, MockQuerier, mock_dependencies}, OwnedDeps, Empty, from_binary, Uint128};

	use crate::{msg::{ExecuteMsg, QueryMsg}, contract::query, contest::queries::ContestQueryResponse, tests::{contract_init_test::tests::_initialize_test, create_contest_test::tests::{_get_valid_create_contest_msg, _create_contest_test}, bet_contest_test::tests::{_bet_contest_test, _get_valid_bet_contest_msg}}};
	
	////////TESTS////////
	#[test]
	fn query_contest_after_creation(){
		let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> = mock_dependencies();

		_initialize_test(&mut deps);

		let msg: ExecuteMsg = _get_valid_create_contest_msg();

		_create_contest_test(&mut deps, msg);

		_query_contest_with_initial_bet(&mut deps);
	}

	#[test]
	fn query_contest_after_creation_and_bet(){
		let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> = mock_dependencies();

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
	){
		let execute_msg = _get_valid_create_contest_msg();
		let (contest_info, amount) = match execute_msg {
			ExecuteMsg::CreateContest { contest_info, amount, .. } => (contest_info, amount),
			_ => panic!("Expected CreateContest variant"),
		};
		
		let msg = QueryMsg::GetContest { 
			contest_id: contest_info.id
		};

		let res = query(deps.as_ref(), mock_env(), msg).unwrap();
		let contest_query_response: ContestQueryResponse = from_binary(&res).unwrap();

		assert_eq!(contest_info.options.len(), contest_query_response.contest_bet_summary.options.len());
		assert_eq!(amount.unwrap(), contest_query_response.contest_bet_summary.calc_total_pool());
	}

	pub fn _query_contest_with_additional_bet(
		deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
	){
		let execute_msg = _get_valid_create_contest_msg();
		let (contest_info, initial_amount) = match execute_msg {
			ExecuteMsg::CreateContest { contest_info, amount, .. } => (contest_info, amount),
			_ => panic!("Expected CreateContest variant"),
		};

		
		let msg = QueryMsg::GetContest { 
			contest_id: contest_info.id
		};

		let execute_msg2 = _get_valid_bet_contest_msg();
		let added_amount = match execute_msg2 {
			ExecuteMsg::BetContest { amount, .. } => amount,
			_ => panic!("Expected CreateContest variant"),
		};


		let res = query(deps.as_ref(), mock_env(), msg).unwrap();
		let contest_query_response: ContestQueryResponse = from_binary(&res).unwrap();

		assert_eq!(contest_info.options.len(), contest_query_response.contest_bet_summary.options.len());
		assert_eq!(initial_amount.unwrap() + added_amount.unwrap(), contest_query_response.contest_bet_summary.calc_total_pool());
		assert_eq!(initial_amount.unwrap() + added_amount.unwrap(), contest_query_response.contest_bet_summary.options[0].bet_allocation);
		assert_eq!(Uint128::from(0u128), contest_query_response.contest_bet_summary.options[1].bet_allocation);
	}


}