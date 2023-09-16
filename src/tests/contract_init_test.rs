
#[cfg(test)]

pub mod tests{
	use cosmwasm_std::{testing::{mock_dependencies, mock_env, mock_info, MockStorage, MockApi, MockQuerier}, Addr, coins, OwnedDeps, Empty};

	use crate::{msg::InstantiateMsg, contract::instantiate};
	
	////////TESTS////////
	#[test]
	fn initialize(){
		let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> = mock_dependencies();
		_initialize_test(&mut deps);
	}
	////////INNER TESTS////////
	pub fn _initialize_test(deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>){
	let msg = InstantiateMsg { 
		satoshis_palace: Addr::unchecked("04eec6a876668ffb7031f9b9ade7c0c4bc47681ac27fec532bfd5c94fb3dd71d675a363d7036ba8d831a499b12e4f04c8741b90e3c4f3c6b64dd1104132d49498c"),
		oracle_contract: Addr::unchecked("TODO FIXME WHEN INTEGRATING WITH ORACLE")
	};

	let info = mock_info("creator", &coins(1000, "earth"));

	let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
	assert_eq!(0, res.messages.len());
}
}

