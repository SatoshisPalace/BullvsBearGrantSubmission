#[cfg(test)]
pub mod tests {

    use cosmwasm_std::{
        coins, from_binary,
        testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
        to_binary, to_vec, Addr, Binary, ContractInfo, Empty, MessageInfo, OwnedDeps, StdResult,
        Timestamp, Uint128,
    };

    use crate::{
        command_handlers::{
            admin_execute_handlers::{handle_claim_fees, handle_set_fee, handle_set_minimum_bet},
            execute_handlers::{handle_claim, handle_claim_multiple, handle_receive},
            invoke_handlers::handle_bet_on_contest,
            query_handlers::{
                handle_get_claimable_contests, handle_get_claimable_fees, handle_get_contest_by_id,
                handle_get_contests_by_ids, handle_get_fee_percent, handle_get_last_ten_contests,
                handle_get_minimum_bet, handle_get_snip20, handle_get_times_to_resolve_from_ids,
                handle_get_total_number_of_bets, handle_get_total_number_of_contests,
                handle_get_total_users, handle_get_total_volume, handle_get_users_list_of_bets,
                handle_get_users_number_of_bets, handle_user_bet, handle_users_last_ten_bets,
            },
        },
        contract::instantiate,
        data::{
            contest_info::ContestInfo, state::FeePercent, user_info::get_users_last_claimed_index,
        },
        msgs::{
            execute::commands::{
                claim::Claim, claim_multiple::ClaimMultiple, receive::Receive, set_fee::SetFee,
                set_minimum_bet::SetMinimumBet,
            },
            instantiate::InstantiateMsg,
            invoke::{commands::bet_contest::BetContest, invoke_msg::InvokeMsg},
            query::commands::{
                get_claimable_contests::GetClaimableContests, get_contest_by_id::GetContestById,
                get_contests_by_ids::GetContestsByIds, get_times_to_resolve::GetTimesToResolve,
                get_user_bet::GetUserBet, get_users_last_ten_bets::GetUsersLastTenBets,
                get_users_list_of_bets::GetUsersListOfBets,
                get_users_number_of_bets::GetUsersNumberOfBets,
            },
        },
        responses::{
            execute::execute_response::ExecuteResponse,
            query::{
                query_response::QueryResponse,
                response_types::{
                    contest_data_list::ContestDataListResponse,
                    times_to_resolve::TimesToResolveResponse, users_bets::UsersBetsResponse,
                },
            },
        },
        services::integrations::price_feed_service::pricefeed::reset_mock_result,
        tests::{
            constants::{BASE_FEE_PERCENT_DENOMINATOR, BASE_FEE_PERCENT_NUMERATOR, INTERVAL},
            contest_infos::get_contest_open,
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

        pub fn set_time(&mut self, seconds: u64) {
            self.env.block.time = Timestamp::from_seconds(seconds)
        }

        pub fn initialize(&mut self, fee_percent: FeePercent) {
            reset_mock_result();
            let msg = InstantiateMsg {
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
                fee_numerator: fee_percent.numerator().to_owned() as u64,
                fee_denominator: fee_percent.denominator().to_owned() as u64,
            };
            let _res = instantiate(self.deps.as_mut(), self.env.clone(), self.info.clone(), msg)
                .expect("contract initialization failed");
        }

        pub fn ensure_index_incrementing(&mut self, expected: Option<u32>) {
            let index = get_users_last_claimed_index(&self.info.sender);
            assert_eq!(
                index.may_load(&self.deps.storage).unwrap(),
                expected,
                "Expected index to be incremting"
            );
        }

        pub fn query_times_to_resolve(
            &mut self,
            command: GetTimesToResolve,
            expected_response: TimesToResolveResponse,
        ) {
            let binary_response =
                handle_get_times_to_resolve_from_ids(self.deps.as_ref(), command).unwrap();
            let query_response: QueryResponse = from_binary(&binary_response).unwrap();
            match query_response {
                QueryResponse::TimesToResolve(response) => {
                    assert_eq!(response, expected_response)
                }
                _ => panic!("Expected Times to Resolve response but received something else"),
            }
        }

        pub fn handle_receive_success(
            &mut self,
            file_number: &u8,
            outcome_to_bet_on: &u8,
            amount_to_bet: &u128,
            sender: Addr,
        ) {
            if let Ok(contest_info) = get_contest_open(*file_number) {
                let info = MessageInfo {
                    sender,
                    funds: coins(1000, "coin"),
                };
                let message = InvokeMsg::BetContest(BetContest {
                    ticker: contest_info.get_ticker(),
                    outcome_id: *outcome_to_bet_on,
                    user: self.info.sender.clone(),
                });
                // Serialize the struct to a JSON byte vector
                let serialized_msg = to_vec(&message).expect("Failed to serialize message");

                // Convert the byte vector to Binary
                let msg: Binary = Binary::from(serialized_msg);

                let command = Receive {
                    sender: Addr::unchecked("Snip20 Address"),
                    from: Addr::unchecked("Address"),
                    amount: Uint128::new(amount_to_bet.clone()),
                    memo: None,
                    msg,
                };
                let response = handle_receive(self.deps.as_mut(), self.env.clone(), info, command);
                response.expect("Failed to bet on contest");
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

        pub fn set_fee_success(&mut self, new_fee: FeePercent) {
            let command = SetFee {
                numerator: *new_fee.numerator() as u64,
                denominator: *new_fee.denominator() as u64,
            };
            let response = handle_set_fee(self.deps.as_mut(), self.info.clone(), command);
            assert!(response.is_ok(), "Expected Set fee to succeed")
        }

        pub fn set_fee_fail(&mut self, new_fee: FeePercent) {
            let command = SetFee {
                numerator: *new_fee.numerator() as u64,
                denominator: *new_fee.denominator() as u64,
            };
            let response = handle_set_fee(self.deps.as_mut(), self.info.clone(), command);
            assert!(response.is_err(), "Expected set fee to fail but succeded")
        }

        pub fn claim_fees_success(&mut self, expected_amount: Option<&u128>) {
            let response_result = handle_claim_fees(self.deps.as_mut(), self.info.clone());
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
        }

        pub fn claim_fees_failure(&mut self) {
            let response_result = handle_claim_fees(self.deps.as_mut(), self.info.clone());
            assert!(
                response_result.is_err(),
                "Expected Claim Fees to Fail but Succeeded"
            );
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
                    "Expected Claim to succeed but failed: {:?}",
                    response_result
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
            let binary_response =
                handle_get_contests_by_ids(self.deps.as_ref(), self.env.clone(), command)
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

        pub fn get_fee_percent(&mut self, expected_fee_percent_option: Option<&FeePercent>) {
            let binary_response = handle_get_fee_percent(self.deps.as_ref())
                .expect("Expected GetFeePercent to succeed but failed");

            let response: QueryResponse =
                from_binary(&binary_response).expect("Failed to deserialize QueryResponse");

            match response {
                QueryResponse::FeePercent(fee_percent_response) => {
                    if let Some(expected_fee_percent) = expected_fee_percent_option {
                        // If an expected fee percent is specified, assert it matches the retrieved value.
                        assert_eq!(
                            fee_percent_response.fee_percent, *expected_fee_percent,
                            "Fee percent is not what was expected"
                        );
                    } else {
                        // If no expected fee percent is provided, just check that itr matches the base fee percent.
                        assert_eq!(
                            fee_percent_response.fee_percent,
                            FeePercent::new(
                                BASE_FEE_PERCENT_NUMERATOR,
                                BASE_FEE_PERCENT_DENOMINATOR
                            ),
                            "Fee percent should exist and be greater than zero"
                        );
                    }
                }
                _ => panic!("Expected FeePercent response but received something else"),
            }
        }

        pub fn get_number_of_contests(&mut self, expected_number: Option<&u32>) {
            let binary_response = handle_get_total_number_of_contests(self.deps.as_ref())
                .expect("Expected GetTotalNumberOfContests to succeed but failed");

            let response: QueryResponse =
                from_binary(&binary_response).expect("Failed to deserialize QueryResponse");

            match response {
                QueryResponse::TotalNumberOfContests(total_number_response) => {
                    if let Some(total_number) = expected_number {
                        // If an expected number is specified, assert it matches the retrieved value.
                        assert_eq!(
                            total_number_response.total_number_of_contests, *total_number,
                            "Total Number is not what was expected"
                        );
                    }
                }
                _ => {
                    panic!("Expected TotalNumberOfContestsResponse but received something else")
                }
            }
        }

        pub fn get_number_of_bets(&mut self, expected_number: Option<&u64>) {
            let binary_response = handle_get_total_number_of_bets(self.deps.as_ref())
                .expect("Expected GetTotalNumberOfBets to succeed but failed");

            let response: QueryResponse =
                from_binary(&binary_response).expect("Failed to deserialize QueryResponse");

            match response {
                QueryResponse::TotalNumberOfBets(total_number_response) => {
                    if let Some(total_number) = expected_number {
                        // If an expected number is specified, assert it matches the retrieved value.
                        assert_eq!(
                            total_number_response.total_number_of_bets, *total_number,
                            "Total Number is not what was expected"
                        );
                    }
                }
                _ => {
                    panic!("Expected TotalNumberOfBetsResponse but received something else")
                }
            }
        }

        pub fn get_number_of_users_bets(&mut self, expected_number: Option<&u32>) {
            let command = GetUsersNumberOfBets {
                user: self.info.sender.clone(),
                viewing_key: "Valid Viewing Key".to_owned(),
            };

            let binary_response = handle_get_users_number_of_bets(self.deps.as_ref(), command)
                .expect("Expected GetUsersNumberOfBets to succeed but failed");

            let response: QueryResponse =
                from_binary(&binary_response).expect("Failed to deserialize QueryResponse");

            match response {
                QueryResponse::TotalUsersNumberOfBets(total_number_response) => {
                    if let Some(total_number) = expected_number {
                        // If an expected number is specified, assert it matches the retrieved value.
                        assert_eq!(
                            total_number_response.total_users_number_of_bets, *total_number,
                            "Total Number is not what was expected"
                        );
                    }
                }
                _ => {
                    panic!("Expected UsersNumberOfBetsResponse but received something else")
                }
            }
        }

        pub fn get_number_of_users(&mut self, expected_number: Option<&u32>) {
            let binary_response = handle_get_total_users(self.deps.as_ref())
                .expect("Expected GetTotalUsers to succeed but failed");

            let response: QueryResponse =
                from_binary(&binary_response).expect("Failed to deserialize QueryResponse");

            match response {
                QueryResponse::TotalNumberOfUsers(total_number_response) => {
                    if let Some(total_number) = expected_number {
                        // If an expected number is specified, assert it matches the retrieved value.
                        assert_eq!(
                            total_number_response.total_number_of_users, *total_number,
                            "Total Number is not what was expected"
                        );
                    }
                }
                _ => {
                    panic!("Expected TotalNumberOfUsersResponse but received something else")
                }
            }
        }

        pub fn get_volume(&mut self, expected_number: Option<&Uint128>) {
            let binary_response = handle_get_total_volume(self.deps.as_ref())
                .expect("Expected GetTotalVolume to succeed but failed");

            let response: QueryResponse =
                from_binary(&binary_response).expect("Failed to deserialize QueryResponse");

            match response {
                QueryResponse::TotalVolume(total_number_response) => {
                    if let Some(total_number) = expected_number {
                        // If an expected number is specified, assert it matches the retrieved value.
                        assert_eq!(
                            total_number_response.total_volume, *total_number,
                            "Total volume is not what was expected"
                        );
                    }
                }
                _ => {
                    panic!("Expected TotalVolumeResponse but received something else")
                }
            }
        }

        pub fn get_claimable_fees(&mut self, expected_claimable_fees_option: Option<&Uint128>) {
            let binary_response = handle_get_claimable_fees(self.deps.as_ref())
                .expect("Expected GetClaimableFees to succeed but failed");

            let response: QueryResponse =
                from_binary(&binary_response).expect("Failed to deserialize QueryResponse");

            match response {
                QueryResponse::ClaimableFees(claimable_fees_response) => {
                    if let Some(expected_claimable_fees) = expected_claimable_fees_option {
                        // If an expected minimum bet is specified, assert it matches the retrieved value.
                        assert_eq!(
                            claimable_fees_response.claimable_fees, *expected_claimable_fees,
                            "ClaimableFees is not what was expected"
                        );
                    } else {
                        // If no expected claimable fees is provided, just check that it exists and is non-zero.
                        assert!(
                            claimable_fees_response.claimable_fees == Uint128::zero(),
                            "ClaimableFees should exist and be 0 if none provided"
                        );
                    }
                }
                _ => panic!("Expected ClaimableFees response but received something else"),
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

        pub fn get_user_bet_failure(&mut self, file_number: &u8) {
            let contest_info = Self::get_open_contest_from_file(file_number);

            let command = GetUserBet {
                user: self.info.sender.clone(),
                contest_id: contest_info.get_id(),
                viewing_key: "Valid Viewing Key".to_owned(),
            };

            let _binary_response = handle_user_bet(self.deps.as_ref(), command)
                .expect_err("Expected GetUserBet to fail but succeeded");
        }

        fn query_users_claimable_contests(&mut self) -> StdResult<UsersBetsResponse> {
            let command = GetClaimableContests {
                user: self.info.sender.clone(),
                viewing_key: "valid viewing key".to_owned(),
            };
            let binary_response =
                handle_get_claimable_contests(self.deps.as_ref(), self.env.clone(), command)?;
            let query_response: QueryResponse = from_binary(&binary_response)?;
            match query_response {
                QueryResponse::UsersBets(response) => Ok(response),
                _ => panic!("Expected Users Bets response but received something else"),
            }
        }

        pub fn users_claimable_contests_has_length(&mut self, expected_length: usize) {
            let users_bets = self.query_users_claimable_contests().unwrap();
            assert_eq!(users_bets.contests_bets.len(), expected_length);
        }

        pub fn users_claimable_contests_includes_contest(&mut self, file_number: &u8) {
            if let Ok(contest_info) = get_contest_open(*file_number) {
                let users_bets: UsersBetsResponse = self.query_users_claimable_contests().unwrap();

                for user_contest_bet_info in users_bets.contests_bets.iter() {
                    if user_contest_bet_info.contest_info.get_time_of_close()
                        == contest_info.get_time_of_close()
                    {
                        return;
                    }
                }

                // If we reach this point, no matching entry was found
                assert!(
                    false,
                    "Users claimable contests does not contain the contest id"
                );
            } else {
                assert!(false, "Contest Info not found")
            }
        }

        fn query_users_last_ten_contests(&mut self) -> StdResult<UsersBetsResponse> {
            let command = GetUsersLastTenBets {
                user: self.info.sender.clone(),
                viewing_key: "valid viewing key".to_owned(),
            };
            let binary_response =
                handle_users_last_ten_bets(self.deps.as_ref(), self.env.clone(), command)?;
            let query_response: QueryResponse = from_binary(&binary_response)?;
            match query_response {
                QueryResponse::UsersBets(response) => Ok(response),
                _ => panic!("Expected Users Bets response but received something else"),
            }
        }

        pub fn users_last_ten_contests_has_length(&mut self, expected_length: usize) {
            let users_bets = self.query_users_last_ten_contests().unwrap();
            assert_eq!(users_bets.contests_bets.len(), expected_length);
        }

        pub fn users_last_ten_contests_includes_contest(&mut self, file_number: &u8) {
            if let Ok(contest_info) = get_contest_open(*file_number) {
                let users_bets: UsersBetsResponse = self.query_users_last_ten_contests().unwrap();

                for user_contest_bet_info in users_bets.contests_bets.iter() {
                    if user_contest_bet_info.contest_info.get_time_of_close()
                        == contest_info.get_time_of_close()
                    {
                        return;
                    }
                }

                // If we reach this point, no matching entry was found
                assert!(
                    false,
                    "Users claimable contests does not contain the contest id"
                );
            } else {
                assert!(false, "Contest Info not found")
            }
        }

        pub fn users_last_ten_contests_does_not_include_contest(&mut self, file_number: &u8) {
            if let Ok(contest_info) = get_contest_open(*file_number) {
                let users_bets: UsersBetsResponse = self.query_users_last_ten_contests().unwrap();

                for user_contest_bet_info in users_bets.contests_bets.iter() {
                    if user_contest_bet_info.contest_info.get_time_of_close()
                        == contest_info.get_time_of_close()
                    {
                        panic!("Expected contest to not be included")
                    }
                }

                // If we reach this point, no matching entry was found
                return;
            } else {
                return;
            }
        }

        fn query_last_ten_contests(&mut self) -> StdResult<ContestDataListResponse> {
            let binary_response =
                handle_get_last_ten_contests(self.deps.as_ref(), self.env.clone())?;
            let query_response: QueryResponse = from_binary(&binary_response)?;
            match query_response {
                QueryResponse::ContestDataList(response) => Ok(response),
                _ => panic!("Expected Users Bets response but received something else"),
            }
        }

        pub fn last_ten_contests_has_length(&mut self, expected_length: &u32) {
            let response = self.query_last_ten_contests().unwrap();
            // Check if the number of contests matches the expected number, if provided.
            assert_eq!(
                *expected_length as usize,
                response.contests.len(),
                "Number of contests does not match the expected number"
            );
        }

        pub fn last_ten_contests_includes_contest(&mut self, file_number: &u8) {
            if let Ok(contest_info) = get_contest_open(*file_number) {
                let contests_data_list_response: ContestDataListResponse =
                    self.query_last_ten_contests().unwrap();

                for contest in contests_data_list_response.contests {
                    if contest.contest_info.get_time_of_close() == contest_info.get_time_of_close()
                    {
                        return;
                    }
                }

                // If we reach this point, no matching entry was found
                assert!(false, "Last ten contests does not contain the contest id");
            } else {
                assert!(false, "Contest Info not found")
            }
        }

        pub fn last_ten_contests_does_not_include_contest(&mut self, file_number: &u8) {
            if let Ok(contest_info) = get_contest_open(*file_number) {
                let contests_data_list_response: ContestDataListResponse =
                    self.query_last_ten_contests().unwrap();

                for contest in contests_data_list_response.contests {
                    if contest.contest_info.get_time_of_close() == contest_info.get_time_of_close()
                    {
                        panic!("Expected contest to not be included")
                    }
                }

                // If we reach this point, no matching entry was found
                return;
            } else {
                return;
            }
        }

        fn query_users_list_of_bets(&mut self, ids: Vec<u32>) -> StdResult<UsersBetsResponse> {
            let command = GetUsersListOfBets {
                user: self.info.sender.clone(),
                viewing_key: "valid viewing key".to_owned(),
                contest_ids: ids,
            };
            let binary_response =
                handle_get_users_list_of_bets(self.deps.as_ref(), self.env.clone(), command)?;
            let query_response: QueryResponse = from_binary(&binary_response)?;
            match query_response {
                QueryResponse::UsersBets(response) => Ok(response),
                _ => panic!("Expected Users Bets response but received something else"),
            }
        }

        pub fn users_list_of_bets_has_length(&mut self, ids: Vec<u32>, expected_length: usize) {
            let users_bets = self.query_users_list_of_bets(ids).unwrap();
            assert_eq!(users_bets.contests_bets.len(), expected_length);
        }

        pub fn users_list_of_bets_includes_contest(&mut self, ids: Vec<u32>, file_number: &u8) {
            if let Ok(contest_info) = get_contest_open(*file_number) {
                let users_bets: UsersBetsResponse = self.query_users_list_of_bets(ids).unwrap();

                for user_contest_bet_info in users_bets.contests_bets.iter() {
                    if user_contest_bet_info.contest_info.get_time_of_close()
                        == contest_info.get_time_of_close()
                    {
                        return;
                    }
                }

                // If we reach this point, no matching entry was found
                assert!(
                    false,
                    "Users list of bets contests does not contain the contest id"
                );
            } else {
                assert!(false, "Contest Info not found")
            }
        }
    }
}
