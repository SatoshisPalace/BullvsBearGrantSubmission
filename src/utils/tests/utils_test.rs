#[cfg(test)]

pub mod tests {
    use cosmwasm_std::{
        coins,
        testing::{mock_env, mock_info},
    };

    use crate::utils::{error::UtilError, utils::contract_only_call};

    ////////TESTS////////

    #[test]
    fn contract_only_call_valid_test() {
        let env = mock_env();
        let info = mock_info(env.contract.address.as_str(), &coins(1000, "earth"));

        let result = contract_only_call(env.clone(), info);
        assert!(result.is_ok(), "Expected Ok, but got an error");
    }

    #[test]
    fn contract_only_call_error_test() {
        let env = mock_env();
        let info = mock_info("some_other_address", &coins(1000, "earth"));

        let result = contract_only_call(env.clone(), info);
        match result {
            Ok(_) => panic!("Expected error, but got Ok"),
            Err(e) => assert_eq!(e, UtilError::ContractOnlyCall),
        }
    }
}
