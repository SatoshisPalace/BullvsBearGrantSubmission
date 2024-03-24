#[cfg(test)]
pub mod tests {
    use cosmwasm_std::{
        coins, from_binary,
        testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
        to_binary, Addr, ContractInfo, Empty, OwnedDeps, StdResult, Uint128,
    };

    use crate::{
        contest::{
            execute_handlers::{handle_bet_on_contest, handle_create_contest},
            query_handlers::handle_users_bets_query,
            responses::query::users_bets_response::UserBetsResponse,
        },
        contract::instantiate,
        msg::InstantiateMsg,
        tests::{
            constants::TESTING_SP_SIGNING_KEY, contest_infos::get_contest_info_and_signature_by_id,
        },
    };

    // Test environment struct
    pub struct TestEnv {
        deps: OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
        info: cosmwasm_std::MessageInfo,
        env: cosmwasm_std::Env,
    }

    impl TestEnv {
        // Constructor for TestEnv
        pub fn new() -> Self {
            let deps = mock_dependencies();
            let info = mock_info("creator", &coins(1000, "coin"));
            let env = mock_env();
            TestEnv { deps, info, env }
        }

        pub fn set_sender(&mut self, sender: String) {
            self.info = mock_info(&sender.to_owned(), &coins(1000, "coin"));
        }

        pub fn initialize(&mut self) {
            let msg = InstantiateMsg {
                satoshis_palace: Addr::unchecked(TESTING_SP_SIGNING_KEY),
                oracle_contract_info: ContractInfo {
                    address: Addr::unchecked("Oracle Address"),
                    code_hash: "Oracle CodeHash".to_owned(),
                },
                snip20: ContractInfo {
                    address: Addr::unchecked("Snip20 Address"),
                    code_hash: "Snip20 Codehash".to_owned(),
                },
                entropy: to_binary("Entropy").unwrap(),
                master_viewing_key_contract: ContractInfo {
                    address: Addr::unchecked("Master Viewing Key Address"),
                    code_hash: "Master Viewing Key CodeHash".to_owned(),
                },
            };
            let _res = instantiate(self.deps.as_mut(), self.env.clone(), self.info.clone(), msg)
                .expect("contract initialization failed");
        }

        pub fn create_valid_contest_success(
            &mut self,
            contest_id: &u32,
            outcome_to_bet_on: &u8,
            amount_to_bet: &u128,
        ) {
            if let Some((contest_info, signature_hex)) =
                get_contest_info_and_signature_by_id(*contest_id)
            {
                let response = handle_create_contest(
                    &mut self.deps.as_mut(),
                    self.env.clone(),
                    contest_info,
                    signature_hex.to_string(),
                    outcome_to_bet_on.clone(),
                    self.info.sender.clone(),
                    Some(Uint128::new(amount_to_bet.clone())),
                );
                response.expect("Failed To Create a valid contest");
            }
        }

        fn query_users_bets(&mut self) -> StdResult<UserBetsResponse> {
            let binary_respoonse = handle_users_bets_query(
                self.deps.as_ref(),
                self.info.sender.clone(),
                "Valid viewing Key".to_owned(),
            )?;
            from_binary(&binary_respoonse)
        }

        pub fn users_bets_has_length(&mut self, expected_length: usize) {
            let users_bets = self.query_users_bets().unwrap();
            assert_eq!(users_bets.contests_bets.len(), expected_length);
        }

        pub fn users_bets_includes_id(&mut self, contest_id: &u32) {
            let users_bets: UserBetsResponse = self.query_users_bets().unwrap();

            for user_contest_bet_info in users_bets.contests_bets.iter() {
                if user_contest_bet_info.contest_info.id == *contest_id
                    && user_contest_bet_info.user_bet.get_contest_id() == contest_id
                {
                    return;
                }
            }

            // If we reach this point, no matching entry was found
            assert!(false, "Users bets does not contain the contest id");
        }

        pub fn bet_on_contest_success(
            &mut self,
            contest_id: &u32,
            outcome_to_bet_on: &u8,
            amount_to_bet: &u128,
        ) {
            let response = handle_bet_on_contest(
                &mut self.deps.as_mut(),
                &self.env.clone(),
                contest_id.clone(),
                outcome_to_bet_on.clone(),
                self.info.sender.clone(),
                Some(Uint128::new(amount_to_bet.clone())),
            );
            response.expect("Failed to bet on contest");
        }
    }
}
