#[cfg(test)]
pub mod tests{

use cosmwasm_std::{testing::{mock_env, MockStorage, MockApi, MockQuerier, mock_dependencies, mock_info}, OwnedDeps, Empty, coins};

	use crate::{msg::ExecuteMsg, contract::execute, tests::contract_init_test::tests::_initialize_test, viewingkeys::response::{ViewingKeyResponse, ResponseStatus}};
	
	////////TESTS////////
	#[test]
	fn create_viewing_key(){
		let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> = mock_dependencies();

		_initialize_test(&mut deps);

		_create_viewing_key_test(&mut deps, "ENTROPY", "api_key_/WMIRnqFsFmb6KuvRSX8LQGSz3umCjcXcptco4gl3Lg=" );
	}

	#[test]
	fn create_viewing_key_twice(){
		let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> = mock_dependencies();

		_initialize_test(&mut deps);

		_create_viewing_key_test(&mut deps, "ENTROPY", "api_key_/WMIRnqFsFmb6KuvRSX8LQGSz3umCjcXcptco4gl3Lg=" );
		_create_viewing_key_test(&mut deps, "ENTROPY", "api_key_lsZ01+yImFptNK114U3QgMvGkQg3Zc8fCy42s2VCcs4=");
	}

	#[test]
	fn set_viewing_key(){
		let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> = mock_dependencies();

		_initialize_test(&mut deps);

		_create_viewing_key_test(&mut deps, "ENTROPY", "api_key_/WMIRnqFsFmb6KuvRSX8LQGSz3umCjcXcptco4gl3Lg=" );

		_set_viewing_key_test(&mut deps, "api_key_lsZ01+yImFptNK114U3QgMvGkQg3Zc8fCy42s2VCcs4=");
	}


	////////INNER TESTS////////
	pub fn _set_viewing_key_test(
		deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
		key: &str,
	) {
		let env = mock_env();
		let msg = _get_set_viewing_key_message(key);  // Assuming this function returns the correct message
		let info = mock_info(env.contract.address.as_str(), &coins(1000, "earth"));
	
		// Execute the contract and get the Response
		let res = execute(deps.as_mut(), env, info, msg).unwrap();
	
		// Extract and deserialize the data from the Response
		let binary_data = res.data.unwrap();
		let response_data: ViewingKeyResponse = ViewingKeyResponse::try_from(binary_data).unwrap();
	
		// Compare the key or status
		match response_data {
			ViewingKeyResponse::SetViewingKey { status } => {
				// Compare the status to some expected value
				assert_eq!(status, ResponseStatus::Success);  // Replace with the actual expected value
			},
			_ => panic!("Unexpected response data"),
		}
	
		// Optionally, you can add more assertions here to check the state of the contract storage
		// For example, you might want to query the contract storage to check if the viewing key has been set correctly
	}

	pub fn _create_viewing_key_test(
		deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
		entropy: &str,
		expected_key : &str
	){
		let env = mock_env();
		let msg = _get_create_viewing_key_message(entropy);
		let info = mock_info(env.contract.address.as_str(), &coins(1000, "earth"));


		let res = execute(deps.as_mut(), env, info, msg).unwrap();
		let binary_data = res.data.unwrap();  // Assuming data is Some(Binary)
		let response_data: ViewingKeyResponse = ViewingKeyResponse::try_from(binary_data).unwrap();  // Using TryFrom

		match response_data {
			ViewingKeyResponse::CreateViewingKey { key } => {
				// Now compare the key
				assert_eq!(key, expected_key);
			},
			_ => panic!("Unexpected response data"),
		}
	}

	/////////Helpers/////////
	pub fn _get_create_viewing_key_message(entropy : &str) -> ExecuteMsg {
		let msg = ExecuteMsg::CreateViewingKey { 
			entropy: entropy.to_owned(),
    		padding: Some("Padding?".to_owned()),
		};
		return msg
	}

	pub fn _get_set_viewing_key_message(key: &str) -> ExecuteMsg {
		let msg = ExecuteMsg::SetViewingKey { 
			key: key.to_owned(),
			padding: Some("Padding?".to_owned()),
		};
		return msg
	}
}