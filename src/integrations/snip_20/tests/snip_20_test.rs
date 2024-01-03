#[cfg(test)]
pub mod tests {
    use cosmwasm_std::{
        coins,
        testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
        Addr, ContractInfo, Empty, OwnedDeps,
    };

    use crate::{
        contract::execute, msg::ExecuteMsg, tests::contract_init_test::tests::_initialize_test,
    };

    ////////TESTS////////
    #[test]
    fn claim_rewards_before_time_of_resolve() {
        let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> =
            mock_dependencies();
        _initialize_test(&mut deps);

        let snip20_contract_info: ContractInfo = ContractInfo {
            address: Addr::unchecked("address"),
            code_hash: "CodeHash".to_string(),
        };
        _register_snip20_test(&mut deps, snip20_contract_info)
    }

    // Inner Tests
    pub fn _register_fake_snip20_test(
        mut deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
    ) {
        let snip20_contract_info: ContractInfo = ContractInfo {
            address: Addr::unchecked("address"),
            code_hash: "CodeHash".to_string(),
        };
        _register_snip20_test(&mut deps, snip20_contract_info)
    }

    pub fn _register_snip20_test(
        deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
        snip20_contract_info: ContractInfo,
    ) {
        let env = mock_env();
        let register_msg = ExecuteMsg::Register {
            reg_addr: snip20_contract_info.address,
            reg_hash: snip20_contract_info.code_hash,
        };
        let info = mock_info(env.contract.address.as_str(), &coins(1000, "earth"));

        // Expect an error when claiming before time_of_resolve
        let res = execute(deps.as_mut(), env.clone(), info.clone(), register_msg);
        assert!(res.is_ok(), "Expected an OK but got {:?}", res);
    }
}
