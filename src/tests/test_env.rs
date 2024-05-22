#[cfg(test)]
pub mod tests {
    use cosmwasm_std::{
        coins, from_binary,
        testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
        to_binary, Addr, ContractInfo, Empty, OwnedDeps, StdResult, Timestamp, Uint128,
    };
    use sp_secret_toolkit::macros::identifiable::Identifiable;

    use crate::{
        command_handlers::{
            admin_execute_handlers::handle_set_minimum_bet,
            execute_handlers::{handle_claim, handle_claim_multiple},
            invoke_handlers::handle_bet_on_contest,
            query_handlers::{
                handle_get_contest_by_id, handle_get_contests, handle_get_contests_by_ids,
                handle_get_minimum_bet, handle_get_snip20, handle_user_bet,
                handle_users_bets_query,
            },
        },
        contract::instantiate,
        data::contest_info::ContestInfo,
        msgs::{
            execute::commands::{
                claim::Claim, claim_multiple::ClaimMultiple, set_minimum_bet::SetMinimumBet,
            },
            instantiate::InstantiateMsg,
            invoke::commands::bet_contest::BetContest,
            query::commands::{
                get_contest_by_id::GetContestById,
                get_contests::{ContestQueryFilter, ContestQuerySortOrder, GetContests},
                get_contests_by_ids::GetContestsByIds,
                get_user_bet::GetUserBet,
                get_users_bets::{GetUsersBets, UsersBetsQueryFilters},
            },
        },
        responses::{
            execute::execute_response::ExecuteResponse,
            query::{query_response::QueryResponse, response_types::users_bets::UsersBetsResponse},
        },
        services::integrations::price_feed_service::pricefeed::{configure_mock, MockConfig},
        tests::{constants::{INTERVAL, TESTING_SP_SIGNING_KEY}, contest_infos::get_contest_open},
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

        pub fn set_time(&mut self, seconds: u64) {
            self.env.block.time = Timestamp::from_seconds(seconds)
        }

        pub fn initialize(&mut self) {
            configure_mock(MockConfig::ReturnError(false));
            let msg = InstantiateMsg {
                satoshis_palace: Addr::unchecked(TESTING_SP_SIGNING_KEY),
                price_feed_info: ContractInfo {
                    address: Addr::unchecked("Price Feed Address"),
                    code_hash: "Price Feed CodeHash".to_owned(),
                },
                snip20: ContractInfo {
                    address: Addr::unchecked("Snip20 Address"),
                    code_hash: "Snip20 Codehash".to_owned(),
                },
                interval: INTERVAL,
                entropy: to_binary("Entropy").unwrap(),
                master_viewing_key_contract: ContractInfo {
                    address: Addr::unchecked("Master Viewing Key Address"),
                    code_hash: "Master Viewing Key CodeHash".to_owned(),
                },
            };
            let _res = instantiate(self.deps.as_mut(), self.env.clone(), self.info.clone(), msg)
                .expect("contract initialization failed");
        }

        fn query_users_bets(
            &mut self,
            filters: Option<Vec<UsersBetsQueryFilters>>,
        ) -> StdResult<UsersBetsResponse> {
            let command = GetUsersBets {
                user: self.info.sender.clone(),
                viewing_key: "valid viewing key".to_owned(),
                filters,
            };
            let binary_respoonse =
                handle_users_bets_query(self.deps.as_ref(), self.env.clone(), command)?;
            let query_response: QueryResponse = from_binary(&binary_respoonse)?;
            match query_response {
                QueryResponse::UsersBets(response) => Ok(response),
                _ => panic!("Expected Users Bets response but received something else"),
            }
        }

        pub fn users_bets_has_length(
            &mut self,
            filters: Option<Vec<UsersBetsQueryFilters>>,
            expected_length: usize,
        ) {
            let users_bets = self.query_users_bets(filters).unwrap();
            assert_eq!(users_bets.contests_bets.len(), expected_length);
        }

        pub fn users_bets_includes_contest(
            &mut self,
            file_number: &u8,
            filters: Option<Vec<UsersBetsQueryFilters>>,
        ) {
            if let Ok(contest_info) = get_contest_open(*file_number) {
                let users_bets: UsersBetsResponse = self.query_users_bets(filters).unwrap();

                for user_contest_bet_info in users_bets.contests_bets.iter() {
                    if user_contest_bet_info.contest_info.id() == contest_info.id()
                        && user_contest_bet_info.user_bet.get_contest_id() == &contest_info.id()
                    {
                        return;
                    }
                }

                // If we reach this point, no matching entry was found
                assert!(false, "Users bets does not contain the contest id");
            } else {
                assert!(false, "Contest Info not found")
            }
        }

        pub fn first_bet_on_contest_success(
            &mut self,
            file_number: &u8,
            outcome_to_bet_on: &u8,
            amount_to_bet: &u128,
        ) {
            if let Ok(contest_info) = get_contest_open(*file_number) {
                let command = BetContest {
                    ticker: contest_info.get_ticker(),
                    outcome_id: *outcome_to_bet_on,
                    user: self.info.sender.clone(),
                };
                let response = handle_bet_on_contest(
                    self.deps.as_mut(),
                    self.env.clone(),
                    command,
                    Uint128::new(amount_to_bet.clone()),
                );
                response.expect("Failed to bet on contest");
            } else {
                assert!(false, "Contest Info not found")
            }
        }

        pub fn first_bet_on_contest_fail(
            &mut self,
            file_number: &u8,
            outcome_to_bet_on: &u8,
            amount_to_bet: &u128,
        ) {
            if let Ok(contest_info) = get_contest_open(*file_number) {
                let command = BetContest {
                    ticker: contest_info.get_ticker(),
                    outcome_id: *outcome_to_bet_on,
                    user: self.info.sender.clone(),
                };

                let response = handle_bet_on_contest(
                    self.deps.as_mut(),
                    self.env.clone(),
                    command,
                    Uint128::new(*amount_to_bet),
                );
                assert!(
                    response.is_err(),
                    "Expected bet on contest to fail, but it succeeded"
                );
            } else {
                assert!(
                    false,
                    "Contest Info not found or already closed for betting"
                )
            }
        }

        pub fn bet_on_contest_success(
            &mut self,
            file_number: &u8,
            outcome_to_bet_on: &u8,
            amount_to_bet: &u128,
        ) {
            if let Ok(contest_info) = get_contest_open(*file_number) {
                let command = BetContest {
                    ticker: contest_info.get_ticker(),
                    outcome_id: *outcome_to_bet_on,
                    user: self.info.sender.clone(),
                };
                let response = handle_bet_on_contest(
                    self.deps.as_mut(),
                    self.env.clone(),
                    command,
                    Uint128::new(amount_to_bet.clone()),
                );
                response.expect("Failed to bet on contest");
            } else {
                assert!(false, "Contest Info not found")
            }
        }

        pub fn bet_on_contest_fail(
            &mut self,
            file_number: &u8,
            outcome_to_bet_on: &u8,
            amount_to_bet: &u128,
        ) {
            if let Ok(contest_info) = get_contest_open(*file_number) {
                let command = BetContest {
                    ticker: contest_info.get_ticker(),
                    outcome_id: *outcome_to_bet_on,
                    user: self.info.sender.clone(),
                };

                let response = handle_bet_on_contest(
                    self.deps.as_mut(),
                    self.env.clone(),
                    command,
                    Uint128::new(*amount_to_bet),
                );
                assert!(
                    response.is_err(),
                    "Expected bet on contest to fail, but it succeeded"
                );
            } else {
                assert!(
                    false,
                    "Contest Info not found or already closed for betting"
                )
            }
        }

        pub fn set_minimum_bet_success(&mut self, minimum_bet: &u128) {
            let command = SetMinimumBet {
                amount: Uint128::from(*minimum_bet),
            };
            let response = handle_set_minimum_bet(self.deps.as_mut(), self.info.clone(), command);
            assert!(response.is_ok(), "Expected Set minimum bet to succeed")
        }

        pub fn set_minimum_bet_fail(&mut self, minimum_bet: &u128) {
            let command = SetMinimumBet {
                amount: Uint128::from(*minimum_bet),
            };
            let response = handle_set_minimum_bet(self.deps.as_mut(), self.info.clone(), command);
            assert!(
                response.is_err(),
                "Expected set minimum bet to fail but succeded"
            )
        }

        pub fn claim_success(&mut self, file_number: &u8, expected_amount: Option<&u128>) {
            if let Ok(contest_info) = get_contest_open(*file_number) {
                let command = Claim {
                    contest_id: contest_info.get_id(),
                };
                let response_result = handle_claim(
                    self.deps.as_mut(),
                    self.env.clone(),
                    self.info.clone(),
                    command,
                );
                assert!(
                    response_result.is_ok(),
                    "Expected Claim to succeed but failed"
                );
                let response = response_result.unwrap();
                assert_eq!(
                    response.messages.len(),
                    1,
                    "Expected claim response to have snip20 msg on it"
                );
                if let Some(binary_data) = response.data {
                    match from_binary::<ExecuteResponse>(&binary_data) {
                        Ok(claim_response) => match claim_response {
                            ExecuteResponse::Claim(claim_response) => {
                                // Successfully deserialized and matched the Claim variant.
                                // You can now use `claim_data` here.
                                if let Some(expected) = expected_amount {
                                    assert_eq!(
                                        claim_response.amount,
                                        Uint128::from(*expected),
                                        "Claim Amount does not match expected"
                                    )
                                }
                            }
                            _ => assert!(false, "Could not deserialize claim response"),
                        },
                        Err(_e) => assert!(false, "Could not deserialize claim response"),
                    }
                } else {
                    assert!(false, "Could not deserialize claim response")
                }
            } else {
                assert!(
                    false,
                    "Contest Info not found or already closed for betting"
                )
            }
        }

        pub fn claim_failure(&mut self, file_number: &u8) {
            if let Ok(contest_info) = get_contest_open(*file_number) {
                let command = Claim {
                    contest_id: contest_info.get_id(),
                };
                let response_result = handle_claim(
                    self.deps.as_mut(),
                    self.env.clone(),
                    self.info.clone(),
                    command,
                );
                assert!(
                    response_result.is_err(),
                    "Expected Claim to Fail but Succeeded"
                );
            } else {
                assert!(
                    false,
                    "Contest Info not found or already closed for betting"
                )
            }
        }

        pub fn claim_multiple_failure(&mut self, file_numbers: Vec<&u8>) {
            let mut requested_ids = Vec::new();

            // Loop through each file number to get contest info and signature.
            for file_number in file_numbers {
                let contest_info = Self::get_open_contest_from_file(file_number);

                // Collect the id from contest_info.
                requested_ids.push(contest_info.get_id());
            }
            let command = ClaimMultiple {
                contest_ids: requested_ids,
            };
            let response_result = handle_claim_multiple(
                self.deps.as_mut(),
                self.env.clone(),
                self.info.clone(),
                command,
            );
            assert!(
                response_result.is_err(),
                "Expected Claim to Fail but Succeeded"
            );
        }

        pub fn claim_multiple_success(
            &mut self,
            file_numbers: Vec<&u8>,
            expected_amount: Option<&u128>,
        ) {
            let mut requested_ids = Vec::new();

            // Loop through each file number to get contest info and signature.
            for file_number in file_numbers {
                let contest_info = Self::get_open_contest_from_file(file_number);

                // Collect the id from contest_info.
                requested_ids.push(contest_info.get_id());
            }
            let command = ClaimMultiple {
                contest_ids: requested_ids,
            };
            let response_result = handle_claim_multiple(
                self.deps.as_mut(),
                self.env.clone(),
                self.info.clone(),
                command,
            );
            assert!(
                response_result.is_ok(),
                "Expected Claim to succeed but failed"
            );
            let response = response_result.unwrap();

            if let Some(binary_data) = response.data {
                match from_binary::<ExecuteResponse>(&binary_data) {
                    Ok(claim_response) => match claim_response {
                        ExecuteResponse::Claim(claim_response) => {
                            // Successfully deserialized and matched the Claim variant.
                            // You can now use `claim_data` here.
                            if let Some(expected) = expected_amount {
                                assert_eq!(
                                    claim_response.amount,
                                    Uint128::from(*expected),
                                    "Claim Amount does not match expected"
                                )
                            }
                        }
                        _ => assert!(false, "Could not deserialize claim response"),
                    },
                    Err(_e) => assert!(false, "Could not deserialize claim response"),
                }
            } else {
                assert!(false, "Could not deserialize claim response")
            }
        }

        pub fn get_contest_success(&mut self, file_number: &u8) {
            if let Ok(contest_info) = get_contest_open(*file_number) {
                let command = GetContestById {
                    contest_id: contest_info.get_id(),
                };
                let response_result = handle_get_contest_by_id(self.deps.as_ref(), command);
                assert!(
                    response_result.is_ok(),
                    "Expected Get Contest to receive but failed"
                );

                match from_binary::<QueryResponse>(&response_result.unwrap()) {
                    Ok(claim_response) => match claim_response {
                        QueryResponse::ContestData(contest_data) => {
                            // Successfully deserialized and matched the Claim variant.
                            // You can now use `claim_data` here.
                            assert_eq!(
                                contest_data.contest_info, contest_info,
                                "Expected Different Contest Info"
                            )
                        }
                        _ => assert!(false, "Could not deserialize contest data response"),
                    },
                    Err(_e) => assert!(false, "Could not deserialize contest data response"),
                }
            } else {
                assert!(
                    false,
                    "Contest Info not found or already closed for betting"
                )
            }
        }

        pub fn get_contests_by_ids_success(
            &mut self,
            file_numbers: &Vec<u8>,
            expected_num_contests: Option<&u128>,
        ) {
            let mut requested_ids = Vec::new();

            // Loop through each file number to get contest info and signature.
            for file_number in file_numbers {
                let contest_info = Self::get_open_contest_from_file(file_number);

                // Collect the id from contest_info.
                requested_ids.push(contest_info.get_id());
            }

            // Populate the GetContests command with the collected ids.
            let command = GetContestsByIds {
                contest_ids: requested_ids.clone(),
            };

            // Handle the get contests command and unwrap the successful result.
            let binary_response = handle_get_contests_by_ids(self.deps.as_ref(), command)
                .expect("Expected GetContests to succeed but it failed");

            // Deserialize the binary response into QueryResponse.
            let response: QueryResponse =
                from_binary(&binary_response).expect("Failed to deserialize QueryResponse");

            if let QueryResponse::ContestDataList(contest_data_list_response) = response {
                // Check if the number of contests matches the expected number, if provided.
                if let Some(expected_count) = expected_num_contests {
                    assert_eq!(
                        *expected_count as usize,
                        contest_data_list_response.contests.len(),
                        "Number of contests does not match the expected number"
                    );
                }
            } else {
                panic!("Expected ContestDataList response but received something else");
            }
        }

        fn get_open_contest_from_file(file_number: &u8) -> ContestInfo {
            if let Ok(contest_info) = get_contest_open(*file_number) {
                return contest_info;
            } else {
                assert!(false, "Contest File not found");
                return ContestInfo::new("BTC".to_owned(), 1, 1, vec![]);
            }
        }

        pub fn get_snip20_success(&mut self) {
            let binary_response = handle_get_snip20(self.deps.as_ref())
                .expect("Expected Get snip20 to succeed but failed");

            let response: QueryResponse =
                from_binary(&binary_response).expect("Failed to deserialize QueryResponse");
            if let QueryResponse::Snip20(snip20_response) = response {
                assert!(
                    !snip20_response.snip20.address.as_str().is_empty(),
                    "Address should not be empty"
                );
                assert!(
                    !snip20_response.snip20.code_hash.is_empty(),
                    "Code hash should not be empty"
                );
            } else {
                panic!("Expected ContestDataList response but received something else");
            }
        }
        pub fn get_minimum_bet_success(&mut self, expected_minimum_bet_option: Option<&u128>) {
            let binary_response = handle_get_minimum_bet(self.deps.as_ref())
                .expect("Expected GetMinimumBet to succeed but failed");

            let response: QueryResponse =
                from_binary(&binary_response).expect("Failed to deserialize QueryResponse");

            match response {
                QueryResponse::MinimumBet(minimum_bet_response) => {
                    if let Some(expected_minimum_bet) = expected_minimum_bet_option {
                        // If an expected minimum bet is specified, assert it matches the retrieved value.
                        assert_eq!(
                            minimum_bet_response.minimum_bet,
                            Uint128::from(*expected_minimum_bet),
                            "Minimum Bet is not what was expected"
                        );
                    } else {
                        // If no expected minimum bet is provided, just check that a minimum bet exists and is non-zero.
                        assert!(
                            minimum_bet_response.minimum_bet > Uint128::zero(),
                            "Minimum Bet should exist and be greater than zero"
                        );
                    }
                }
                _ => panic!("Expected MinimumBet response but received something else"),
            }
        }

        pub fn get_user_bet_success(
            &mut self,
            file_number: &u8,
            expected_bet_option: Option<&u128>,
            expected_side_option: Option<&u8>,
            expected_has_been_paid_option: Option<&bool>,
        ) {
            let contest_info = Self::get_open_contest_from_file(file_number);

            let command = GetUserBet {
                user: self.info.sender.clone(),
                contest_id: contest_info.get_id(),
                viewing_key: "Valid Viewing Key".to_owned(),
            };

            let binary_response = handle_user_bet(self.deps.as_ref(), command)
                .expect("Expected GetUserBet to succeed but failed");

            let response: QueryResponse =
                from_binary(&binary_response).expect("Failed to deserialize QueryResponse");
            if let QueryResponse::UserBet(user_bet_response) = response {
                let bet = user_bet_response.bet;

                // Mandatory Checks
                assert_eq!(bet.get_user(), &self.info.sender, "User does not match");
                assert_eq!(
                    bet.get_contest_id(),
                    &contest_info.get_id(),
                    "Contest ID does not match"
                );

                // Optional Checks
                if let Some(expected_bet) = expected_bet_option {
                    assert_eq!(
                        bet.get_amount(),
                        Uint128::from(*expected_bet),
                        "Bet amount does not match"
                    );
                }
                if let Some(expected_side) = expected_side_option {
                    assert_eq!(
                        bet.get_outcome_id(),
                        expected_side,
                        "Outcome ID does not match"
                    );
                }
                if let Some(expected_has_been_paid) = expected_has_been_paid_option {
                    assert_eq!(
                        bet.has_been_paid(),
                        *expected_has_been_paid,
                        "has_been_paid status does not match"
                    );
                }
            } else {
                panic!("Expected UserBet response but received something else");
            }
        }

        pub fn get_contests_success(
            &self,
            page_num: Option<u32>,
            page_size: Option<u32>,
            sort_order: Option<ContestQuerySortOrder>,
            filter: Option<ContestQueryFilter>,
            expected_length: usize,
        ) {
            let command = GetContests {
                page_num,
                page_size,
                sort_order: sort_order.clone(),
                filter: filter.clone(),
            };

            let binary_response =
                handle_get_contests(self.deps.as_ref(), self.env.clone(), command)
                    .expect("Expected Get Active Contests to succeed but failed");
            let response: QueryResponse =
                from_binary(&binary_response).expect("Failed to deserialize QueryResponse");

            if let QueryResponse::ContestDataList(contest_data_list_response) = response {
                let contest_data_list = contest_data_list_response.contests;
                assert_eq!(contest_data_list.len(), expected_length);
                if let Some(ContestQuerySortOrder::Volume) = sort_order {
                    // Extract the total pool volumes for each contest
                    let volumes: Vec<Uint128> = contest_data_list
                        .iter()
                        .map(|contest_data_response| {
                            contest_data_response.contest_bet_summary.calc_total_pool()
                        })
                        .collect();

                    // Check if the volumes list is sorted in descending order
                    // Adjust this logic if you need ascending order or have other sort orders
                    let is_sorted_descending = volumes.windows(2).all(|w| w[0] >= w[1]);

                    assert!(
                        is_sorted_descending,
                        "Contests are not sorted in descending order of volume as expected."
                    );
                } else if let Some(ContestQuerySortOrder::Descending) = sort_order {
                    // Extract the time of close for each contest
                    let times_of_close: Vec<u64> = contest_data_list
                        .iter()
                        .map(|contest_data_response| {
                            contest_data_response.contest_info.get_time_of_close()
                        })
                        .collect();
                    // Check if the volumes list is sorted in descending order
                    // Adjust this logic if you need ascending order or have other sort orders
                    let is_sorted_descending = times_of_close.windows(2).all(|w| w[0] >= w[1]);

                    assert!(
                        is_sorted_descending,
                        "Contests are not sorted in descending order of time of close as expected."
                    );
                }
            } else {
                panic!("Expected ContestDataList response but received something else");
            }
        }
    }
}
