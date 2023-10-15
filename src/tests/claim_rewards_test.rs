#[cfg(test)]

pub mod tests {
    use cosmwasm_std::{
        coins,
        testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage}, Empty, OwnedDeps, Timestamp,
    };

    use crate::{
        contract::execute,
        msg::ExecuteMsg,
        tests::{
            contract_init_test::tests::_initialize_test,
            create_contest_test::tests::{_create_contest_test, _get_valid_create_contest_msg}, constants::FAR_IN_THE_FUTURE, bet_contest_test::tests::_bet_contest_test,
        },
    };

    ////////TESTS////////
	#[test]
	fn claim_rewards_before_time_of_resolve() {
		let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> = mock_dependencies();
	
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
	fn claim_rewards_after_time_of_resolve() {
		let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> = mock_dependencies();
	
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
	
		// Try to claim rewards after time_of_resolve
		claim_rewards_after_time_of_resolve_test(&mut deps, contest_id);
	}

	#[test]
	fn claim_rewards_after_time_of_resolve_twice() {
		let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> = mock_dependencies();
	
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
	
		// Try to claim rewards after time_of_resolve
		claim_rewards_after_time_of_resolve_twice_test(&mut deps, contest_id);
	}

	////////INNER TESTS////////
	
	fn claim_rewards_before_time_of_resolve_test(deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>, contest_id: u32) {
		let env = mock_env();
		let claim_msg = ExecuteMsg::Claim { contest_id };
		let info = mock_info(env.contract.address.as_str(), &coins(1000, "earth"));
	
		// Expect an error when claiming before time_of_resolve
		let res = execute(deps.as_mut(), env.clone(), info.clone(), claim_msg);
		assert!(res.is_err(), "Expected an error but got {:?}", res);
	}
	fn claim_rewards_after_time_of_resolve_test(deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>, contest_id: u32) {
		let mut env = mock_env();
		let claim_msg = ExecuteMsg::Claim { contest_id };
		let info = mock_info(env.contract.address.as_str(), &coins(1000, "earth"));
	
		env.block.time = Timestamp::from_seconds(FAR_IN_THE_FUTURE + 1u64);
	
		// Expect a successful claim after time_of_resolve
		let res = execute(deps.as_mut(), env.clone(), info.clone(), claim_msg);
		//If worked should make it all the way to the payout, but since this is a unit test there is no snip-20 to integrate with this will giv error
		let err_msg = res.unwrap_err().to_string();
		assert!(err_msg.contains("Snip-20"), "Expected error containing 'Snip-20', got: {}", err_msg);
	}

	fn claim_rewards_after_time_of_resolve_twice_test(deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>, contest_id: u32) {
		let mut env = mock_env();
		let claim_msg = ExecuteMsg::Claim { contest_id };
		let info = mock_info(env.contract.address.as_str(), &coins(1000, "earth"));
	
		env.block.time = Timestamp::from_seconds(FAR_IN_THE_FUTURE + 1u64);
	
		// Expect a successful claim after time_of_resolve
		let res: Result<cosmwasm_std::Response, cosmwasm_std::StdError> = execute(deps.as_mut(), env.clone(), info.clone(), claim_msg.clone());
		//If worked should make it all the way to the payout, but since this is a unit test there is no snip-20 to integrate with this will giv error
		let err_msg = res.unwrap_err().to_string();
		assert!(err_msg.contains("Snip-20"), "Expected error containing 'Snip-20', got: {}", err_msg);


		// Expect a successful claim after time_of_resolve
		let res: Result<cosmwasm_std::Response, cosmwasm_std::StdError> = execute(deps.as_mut(), env.clone(), info.clone(), claim_msg.clone());
		
		let err_msg = res.unwrap_err().to_string();
		assert!(err_msg.contains("paid"), "Expected error containing 'paid', got: {}", err_msg);
	}
	

}
