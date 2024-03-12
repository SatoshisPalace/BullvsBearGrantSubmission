#[cfg(test)]
pub mod tests {

    use cosmwasm_std::{
        coins,
        testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
        Empty, OwnedDeps,
    };

    use crate::{
        contract::execute,
        msg::ExecuteMsg,
        tests::contract_init_test::tests::_initialize_test,
        viewingkeys::response::{ResponseStatus, ViewingKeyResponse},
    };

    ////////TESTS////////
    #[test]
    fn set_viewing_key() {
        let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> =
            mock_dependencies();

        _initialize_test(&mut deps);

        _set_viewing_key_test(
            &mut deps,
            "api_key_lsZ01+yImFptNK114U3QgMvGkQg3Zc8fCy42s2VCcs4=",
        );
    }

    ////////INNER TESTS////////
    pub fn _set_viewing_key_test(
        deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
        key: &str,
    ) {
        let env = mock_env();
        let msg = _get_set_viewing_key_message(key); // Assuming this function returns the correct message
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
                assert_eq!(status, ResponseStatus::Success); // Replace with the actual expected value
            }
            _ => panic!("Unexpected response data"),
        }

        // Optionally, you can add more assertions here to check the state of the contract storage
        // For example, you might want to query the contract storage to check if the viewing key has been set correctly
    }

    /////////Helpers/////////

    pub fn _get_set_viewing_key_message(key: &str) -> ExecuteMsg {
        let msg = ExecuteMsg::SetViewingKey {
            key: key.to_owned(),
            padding: Some("Padding?".to_owned()),
        };
        return msg;
    }
}
