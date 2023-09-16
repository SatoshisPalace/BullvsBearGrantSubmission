#[cfg(test)]


pub mod tests{
	use cosmwasm_std::{testing::{mock_dependencies, mock_env, MockStorage, MockApi, MockQuerier, mock_info}, OwnedDeps, Empty, Uint128, Addr, coins, Response};

	use crate::{msg::ExecuteMsg, contract::execute_from_snip_20, tests::{contract_init_test::tests::_initialize_test, create_contest_test::tests::{_get_valid_create_contest_msg, _create_contest_test}}};
	
	////////TESTS////////
	#[test]
	fn bet_on_contest_creation(){
		let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> = mock_dependencies();

		_initialize_test(&mut deps);

		let msg: ExecuteMsg = _get_valid_create_contest_msg();

		_create_contest_test(&mut deps, msg);
	}

	#[test]
	fn bet_on_contest_after_creation(){
		let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> = mock_dependencies();

		_initialize_test(&mut deps);

		let msg: ExecuteMsg = _get_valid_create_contest_msg();

		_create_contest_test(&mut deps, msg);

		_bet_contest_test(&mut deps)
	}
	////////INNER TESTS////////

	pub fn _bet_contest_test(
		deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
	){
		let env = mock_env();
		let msg = _get_valid_bet_contest_msg();
		let info = mock_info(env.contract.address.as_str(), &coins(1000, "earth"));
		let res = execute_from_snip_20(deps.as_mut(), env, info, msg).unwrap();
		assert_eq!(Response::default(), res);
	}

	/////////Helpers/////////
	pub fn _get_valid_bet_contest_msg() -> ExecuteMsg {
		let execute_msg = _get_valid_create_contest_msg();
		let (contest_info, outcome_id) = match execute_msg {
			ExecuteMsg::CreateContest { contest_info, outcome_id, .. } => (contest_info, outcome_id ),
			_ => panic!("Expected CreateContest variant"),
		};

		let msg = ExecuteMsg::BetContest { 
			contest_id: contest_info.id,
			outcome_id: outcome_id,
			sender: Option::Some(Addr::unchecked("secret101010101010101010101010101010101010101".to_owned())),
			amount: Option::Some(Uint128::from(100u128))
		};
		return msg
	}

}