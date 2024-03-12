// #[cfg(test)]
// pub mod tests {
//     use cosmwasm_std::{
//         coins,
//         testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
//         Addr, ContractInfo, Empty, OwnedDeps,
//     };

//     use crate::{
//         contract::execute, msg::ExecuteMsg, tests::contract_init_test::tests::_initialize_test,
//     };

//     ////////TESTS////////
//     #[test]
//     fn claim_rewards_before_time_of_resolve() {
//         let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> =
//             mock_dependencies();
//         _initialize_test(&mut deps);

//         let snip20_contract_info: ContractInfo = ContractInfo {
//             address: Addr::unchecked("address"),
//             code_hash: "CodeHash".to_string(),
//         };
//         _register_snip20_test(&mut deps, snip20_contract_info)
//     }

//     // Inner Tests
//     pub fn _register_fake_snip20_test(
//         mut deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
//     ) {
//         let snip20_contract_info: ContractInfo = ContractInfo {
//             address: Addr::unchecked("address"),
//             code_hash: "CodeHash".to_string(),
//         };
//         _register_snip20_test(&mut deps, snip20_contract_info)
//     }
// }
