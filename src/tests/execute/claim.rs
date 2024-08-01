#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use std::time::{Duration, Instant};

    use cosmwasm_std::Decimal;
    use sp_secret_toolkit::price_feed::{
        data::price_posting::PricePosting,
        response::response_types::prices_by_ids::PricesByIdsResponse,
    };

    use crate::services::integrations::price_feed_service::pricefeed::set_oracle_result;
    use crate::tests::{
        constants::{
            AFTER_TIME_OF_1_CLOSE, AFTER_TIME_OF_2_CLOSE, AFTER_TIME_OF_3_CLOSE,
            AFTER_TIME_OF_4_CLOSE, AFTER_TIME_OF_RESOLVE,
        },
        test_env::tests::TestEnv,
    };
    use crate::{
        data::state::FeePercent,
        tests::constants::{BASE_FEE_PERCENT_DENOMINATOR, BASE_FEE_PERCENT_NUMERATOR},
    };

    ////////TESTS////////
    #[test]
    fn cannot_claim_open_contest() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);

        test_env.claim_failure(&contest_file);
    }

    #[test]
    fn cannot_closed_awaiting_results_contest() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);

        test_env.set_time(AFTER_TIME_OF_1_CLOSE);

        test_env.claim_failure(&contest_file);
    }

    #[test]
    fn claim_contest_no_opposition() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        let amount_bet = 100;
        test_env.first_bet_on_contest_success(&contest_file, &1, &amount_bet);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        test_env.claim_success(&contest_file, Some(&amount_bet));
    }

    #[test]
    fn claim_contest_no_opposition_2_user() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        let amount_bet = 100;
        test_env.first_bet_on_contest_success(&contest_file, &1, &amount_bet);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&contest_file, &1, &amount_bet);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        test_env.claim_success(&contest_file, Some(&amount_bet));

        test_env.set_sender("creator".to_owned());
        test_env.claim_success(&contest_file, Some(&amount_bet));
    }

    #[test]
    fn claim_contest_no_opposition_10_user() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        let amount_bet = 100;
        let users = vec![
            "user1", "user2", "user3", "user4", "user5", "user6", "user7", "user8", "user9",
            "user10",
        ];

        // All users place bets
        for (i, user) in users.iter().enumerate() {
            test_env.set_sender(user.to_string());
            if i == 0 {
                test_env.first_bet_on_contest_success(&contest_file, &1, &amount_bet);
            } else {
                test_env.bet_on_contest_success(&contest_file, &1, &amount_bet);
            }
        }

        // Set time to after the resolution time
        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        // All users claim their winnings
        for user in &users {
            test_env.set_sender(user.to_string());
            test_env.claim_success(&contest_file, Some(&amount_bet));
        }
    }

    #[test]
    fn claim_contest_with_opposition_10_users() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(0, 1)); // Set fee to 0%
        let contest_file = 1;
        let amount_bet = 100;
        let users = vec![
            "user1", "user2", "user3", "user4", "user5", "user6", "user7", "user8", "user9",
            "user10",
        ];

        // Half the users bet on one side, half on the other
        for (i, user) in users.iter().enumerate() {
            test_env.set_sender(user.to_string());
            if i == 0 {
                test_env.first_bet_on_contest_success(&contest_file, &1, &amount_bet);
            } else if i < users.len() / 2 {
                test_env.bet_on_contest_success(&contest_file, &1, &amount_bet);
            } else {
                test_env.bet_on_contest_success(&contest_file, &2, &amount_bet);
            }
        }

        // Set time to after the resolution time
        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        // Users who bet on the winning side claim their winnings
        for (i, user) in users.iter().enumerate() {
            test_env.set_sender(user.to_string());
            if i < users.len() / 2 {
                // Users on the winning side should succeed in claiming
                test_env.claim_success(
                    &contest_file,
                    Some(&(((users.len() as u128) * amount_bet) / (users.len() / 2) as u128)),
                ); // Example calculation for expected amount
            } else {
                // Users on the losing side should fail in claiming
                test_env.claim_failure(&contest_file);
            }
        }
    }

    fn claim_contest_with_opposition_n_users(n: usize) {
        let start_total = Instant::now();

        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(0, 1)); // Set fee to 0%
        let contest_file = 1;
        let amount_bet = 100;
        let users: Vec<String> = (1..=n).map(|i| format!("user{}", i)).collect();

        let start_betting = Instant::now();
        let mut total_bet_time = Duration::new(0, 0);
        let mut bet_count = 0;

        // Half the users bet on one side, half on the other
        for (i, user) in users.iter().enumerate() {
            test_env.set_sender(user.to_string());
            let bet_start = Instant::now();
            if i == 0 {
                test_env.first_bet_on_contest_success(&contest_file, &1, &amount_bet);
            } else if i < users.len() / 2 {
                test_env.bet_on_contest_success(&contest_file, &1, &amount_bet);
            } else {
                test_env.bet_on_contest_success(&contest_file, &2, &amount_bet);
            }
            total_bet_time += bet_start.elapsed();
            bet_count += 1;
        }

        let duration_betting = start_betting.elapsed();
        println!("Time taken for betting: {:?}", duration_betting);
        println!(
            "Average time per bet: {:?}",
            total_bet_time / bet_count as u32
        );

        // Set time to after the resolution time
        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        let start_claiming = Instant::now();
        let mut total_claim_success_time = Duration::new(0, 0);
        let mut total_claim_failure_time = Duration::new(0, 0);
        let mut claim_success_count = 0;
        let mut claim_failure_count = 0;

        // Users who bet on the winning side claim their winnings
        for (i, user) in users.iter().enumerate() {
            test_env.set_sender(user.to_string());
            let claim_start = Instant::now();
            if i < users.len() / 2 {
                // Users on the winning side should succeed in claiming
                test_env.claim_success(
                    &contest_file,
                    Some(&(((users.len() as u128) * amount_bet) / (users.len() / 2) as u128)),
                ); // Example calculation for expected amount
                total_claim_success_time += claim_start.elapsed();
                claim_success_count += 1;
            } else {
                // Users on the losing side should fail in claiming
                test_env.claim_failure(&contest_file);
                total_claim_failure_time += claim_start.elapsed();
                claim_failure_count += 1;
            }
        }

        let duration_claiming = start_claiming.elapsed();
        println!("Time taken for claiming: {:?}", duration_claiming);
        println!(
            "Average time per successful claim: {:?}",
            total_claim_success_time / claim_success_count as u32
        );
        println!(
            "Average time per failed claim: {:?}",
            total_claim_failure_time / claim_failure_count as u32
        );

        let duration_total = start_total.elapsed();
        println!("Total time taken: {:?}", duration_total);
    }

    #[test]
    fn test_claim_contest_with_opposition_10_users() {
        // let iterations = 100;
        // for _ in 0..iterations {
        claim_contest_with_opposition_n_users(10);
        //     println!("-------------------------------");
        // }
    }

    #[test]
    fn test_claim_contest_with_opposition_100_users() {
        claim_contest_with_opposition_n_users(100);
    }

    #[test]
    fn test_claim_contest_with_opposition_1000_users() {
        claim_contest_with_opposition_n_users(1000);
    }

    #[test]
    fn test_claim_contest_with_opposition_10000_users() {
        // let iterations = 100;
        // for _ in 0..iterations {
        claim_contest_with_opposition_n_users(10000);
        //     println!("-------------------------------");
        // }
    }

    #[test]
    fn claim_contest_with_skewed_opposition() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(0, 1)); // Set fee to 0%
        let contest_file = 1;
        let amount_bet = 100;
        let users = vec![
            "user1", "user2", "user3", "user4", "user5", "user6", "user7", "user8", "user9",
            "user10",
        ];

        // 3 users bet on side 1, 7 users bet on side 2
        for (i, user) in users.iter().enumerate() {
            test_env.set_sender(user.to_string());
            if i == 0 {
                test_env.first_bet_on_contest_success(&contest_file, &1, &amount_bet);
            } else if i < 3 {
                test_env.bet_on_contest_success(&contest_file, &1, &amount_bet);
            } else {
                test_env.bet_on_contest_success(&contest_file, &2, &amount_bet);
            }
        }

        // Set time to after the resolution time
        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        // Calculate the total pool and the expected amount for each winning user
        let total_pool = users.len() as u128 * amount_bet;
        let expected_payout_per_winner = total_pool / 3; // Since there are 7 winners

        // Users who bet on the winning side claim their winnings
        for (i, user) in users.iter().enumerate() {
            test_env.set_sender(user.to_string());
            if i < 3 {
                // Users on the winning side should succeed in claiming
                test_env.claim_success(
                    &contest_file,
                    Some(&expected_payout_per_winner), // Each winning user gets their share of the total pool
                );
            } else {
                // Users on the losing side should fail in claiming
                test_env.claim_failure(&contest_file);
            }
        }
    }

    #[test]
    fn claim_contest_with_varied_bets_weighted_losers() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(0, 1)); // Set fee to 0%
        let contest_file = 1;
        let users = vec![
            ("user1", 50), // Less even and varied bet amounts
            ("user2", 200),
            ("user3", 75),
            ("user4", 150),
            ("user5", 300),
            ("user6", 450),
            ("user7", 125),
            ("user8", 175),
            ("user9", 225),
            ("user10", 500),
        ];

        // First 3 users bet on side 1, remaining 7 bet on side 2
        for (i, &(user, amount)) in users.iter().enumerate() {
            test_env.set_sender(user.to_string());
            if i == 0 {
                test_env.first_bet_on_contest_success(&contest_file, &1, &amount);
            } else if i < 3 {
                test_env.bet_on_contest_success(&contest_file, &1, &amount);
            } else {
                test_env.bet_on_contest_success(&contest_file, &2, &amount);
            }
        }

        // Set time to after the resolution time
        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        // Calculate the total pool and the total bet amounts on the winning side (side 1 in this case)
        let total_pool: u128 = users.iter().map(|&(_, amount)| amount).sum();
        let total_winning_bets: u128 = users.iter().take(3).map(|&(_, amount)| amount).sum();

        // Users who bet on the winning side claim their winnings
        for (i, &(user, amount)) in users.iter().enumerate() {
            test_env.set_sender(user.to_string());
            if i < 3 {
                // Calculate expected payout based on the user's share of the winning side
                let expected_payout = amount as u128 * total_pool / total_winning_bets;
                test_env.claim_success(&contest_file, Some(&expected_payout)); // Each winning user gets their share of the total pool
            } else {
                // Users on the losing side should fail in claiming
                test_env.claim_failure(&contest_file);
            }
        }
    }

    #[test]
    fn claim_contest_with_varied_bets_weighted_winners() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(0, 1)); // Set fee to 0%
        let contest_file = 1;
        let users = vec![
            ("user6", 450),
            ("user7", 125),
            ("user8", 175),
            ("user9", 225),
            ("user10", 500),
            ("user1", 50), // Less even and varied bet amounts
            ("user2", 200),
            ("user3", 75),
            ("user4", 150),
            ("user5", 300),
        ];

        // First 3 users bet on side 1, remaining 7 bet on side 2
        for (i, &(user, amount)) in users.iter().enumerate() {
            test_env.set_sender(user.to_string());
            if i == 0 {
                test_env.first_bet_on_contest_success(&contest_file, &1, &amount);
            } else if i < 3 {
                test_env.bet_on_contest_success(&contest_file, &1, &amount);
            } else {
                test_env.bet_on_contest_success(&contest_file, &2, &amount);
            }
        }

        // Set time to after the resolution time
        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        // Calculate the total pool and the total bet amounts on the winning side (side 1 in this case)
        let total_pool: u128 = users.iter().map(|&(_, amount)| amount).sum();
        let total_winning_bets: u128 = users.iter().take(3).map(|&(_, amount)| amount).sum();

        // Users who bet on the winning side claim their winnings
        for (i, &(user, amount)) in users.iter().enumerate() {
            test_env.set_sender(user.to_string());
            if i < 3 {
                // Calculate expected payout based on the user's share of the winning side
                let expected_payout = amount as u128 * total_pool / total_winning_bets;
                test_env.claim_success(&contest_file, Some(&expected_payout)); // Each winning user gets their share of the total pool
            } else {
                // Users on the losing side should fail in claiming
                test_env.claim_failure(&contest_file);
            }
        }
    }

    #[test]
    fn claim_contest_with_varied_bets_prime_total() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(0, 1)); // Set fee to 0%
        let contest_file = 1;
        let users = vec![
            ("user1", 101), // Varied bet amounts adding up to 1013
            ("user2", 103),
            ("user3", 107),
            ("user4", 109),
            ("user5", 113),
            ("user6", 127),
            ("user7", 131),
            ("user8", 137),
            ("user9", 139),
            ("user10", 146), // Total sum is 1013 (a prime number)
        ];

        // First 3 users bet on side 1, remaining 7 bet on side 2
        for (i, &(user, amount)) in users.iter().enumerate() {
            test_env.set_sender(user.to_string());
            if i == 0 {
                test_env.first_bet_on_contest_success(&contest_file, &1, &amount);
            } else if i < 3 {
                test_env.bet_on_contest_success(&contest_file, &1, &amount);
            } else {
                test_env.bet_on_contest_success(&contest_file, &2, &amount);
            }
        }

        // Set time to after the resolution time
        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        // Calculate the total pool and the total bet amounts on the winning side (side 1 in this case)
        let total_pool: u128 = users.iter().map(|&(_, amount)| amount).sum();
        let total_winning_bets: u128 = users.iter().take(3).map(|&(_, amount)| amount).sum();

        // Users who bet on the winning side claim their winnings
        for (i, &(user, amount)) in users.iter().enumerate() {
            test_env.set_sender(user.to_string());
            if i < 3 {
                // Calculate expected payout based on the user's share of the winning side
                let expected_payout = amount as u128 * total_pool / total_winning_bets;
                test_env.claim_success(&contest_file, Some(&expected_payout)); // Each winning user gets their share of the total pool
            } else {
                // Users on the losing side should fail in claiming
                test_env.claim_failure(&contest_file);
            }
        }
    }

    #[test]
    fn claim_after_expiration_window_no_opposition() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        let amount_bet = 100;
        test_env.first_bet_on_contest_success(&contest_file, &1, &amount_bet);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        test_env.claim_success(&contest_file, Some(&amount_bet));
    }

    #[test]
    fn claim_contest_bets_on_both_sides() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&contest_file, &2, &100);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);
        test_env.claim_failure(&contest_file);

        test_env.set_sender("creator".to_owned());
        test_env.claim_success(&contest_file, Some(&198));
    }

    #[test]
    fn claim_contest_bets_on_both_sides_side_2() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &2, &100);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&contest_file, &1, &100);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        test_env.set_sender("creator".to_owned());

        let oracle_result = PricesByIdsResponse {
            prices: vec![
                PricePosting::new(Decimal::from_str("58205.43").unwrap(), 1571797500),
                PricePosting::new(Decimal::from_str("58205.29").unwrap(), 1571797800),
            ],
        };
        set_oracle_result(Some(oracle_result));

        test_env.claim_success(&contest_file, None);
    }

    #[test]
    fn claim_contest_bets_on_both_sides_side_1_price_returned() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &2, &100);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&contest_file, &1, &100);

        test_env.set_time(AFTER_TIME_OF_2_CLOSE);

        test_env.set_sender("creator".to_owned());

        let oracle_result = PricesByIdsResponse {
            prices: vec![PricePosting::new(
                Decimal::from_str("58205.43").unwrap(),
                1571797500,
            )],
        };
        set_oracle_result(Some(oracle_result));

        test_env.claim_failure(&contest_file);
    }

    #[test]
    fn claim_contest_bets_on_both_sides_same_price() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &2, &100);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&contest_file, &1, &100);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        test_env.set_sender("creator".to_owned());

        let oracle_result = PricesByIdsResponse {
            prices: vec![
                PricePosting::new(Decimal::from_str("58205.29").unwrap(), 1571797500),
                PricePosting::new(Decimal::from_str("58205.29").unwrap(), 1571797800),
            ],
        };
        set_oracle_result(Some(oracle_result));

        test_env.claim_success(&contest_file, Some(&100));
    }

    #[test]
    fn claim_multiple_contests() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        test_env.first_bet_on_contest_success(&1, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&1, &2, &100);
        test_env.claim_failure(&1);

        test_env.set_time(AFTER_TIME_OF_1_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&2, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&2, &2, &100);
        test_env.claim_failure(&2);

        test_env.set_time(AFTER_TIME_OF_2_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&3, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&3, &2, &100);
        test_env.claim_failure(&3);

        test_env.set_time(AFTER_TIME_OF_3_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&4, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&4, &2, &100);
        test_env.claim_failure(&4);

        test_env.set_time(AFTER_TIME_OF_4_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&5, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&5, &2, &100);
        test_env.claim_failure(&5);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        test_env.set_sender("creator".to_owned());
        test_env.claim_success(&5, Some(&198));
        test_env.claim_success(&4, Some(&198));
        test_env.claim_success(&3, Some(&198));
        test_env.claim_success(&2, Some(&198));
        test_env.claim_success(&1, Some(&198));
    }

    #[test]
    fn claim_multiple_contests_2_fee() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(2, BASE_FEE_PERCENT_DENOMINATOR));

        test_env.first_bet_on_contest_success(&1, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&1, &2, &100);
        test_env.claim_failure(&1);

        test_env.set_time(AFTER_TIME_OF_1_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&2, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&2, &2, &100);
        test_env.claim_failure(&2);

        test_env.set_time(AFTER_TIME_OF_2_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&3, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&3, &2, &100);
        test_env.claim_failure(&3);

        test_env.set_time(AFTER_TIME_OF_3_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&4, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&4, &2, &100);
        test_env.claim_failure(&4);

        test_env.set_time(AFTER_TIME_OF_4_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&5, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&5, &2, &100);
        test_env.claim_failure(&5);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        test_env.claim_failure(&5);
        test_env.claim_failure(&4);
        test_env.claim_failure(&3);
        test_env.claim_failure(&2);
        test_env.claim_failure(&1);

        test_env.set_sender("creator".to_owned());
        test_env.claim_success(&5, Some(&196));
        test_env.claim_success(&4, Some(&196));
        test_env.claim_success(&3, Some(&196));
        test_env.claim_success(&2, Some(&196));
        test_env.claim_success(&1, Some(&196));
    }

    #[test]
    fn claim_multiple_contests_01_fee() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(1, BASE_FEE_PERCENT_DENOMINATOR * 10));

        test_env.first_bet_on_contest_success(&1, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&1, &2, &100);
        test_env.claim_failure(&1);

        test_env.set_time(AFTER_TIME_OF_1_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&2, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&2, &2, &100);
        test_env.claim_failure(&2);

        test_env.set_time(AFTER_TIME_OF_2_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&3, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&3, &2, &100);
        test_env.claim_failure(&3);

        test_env.set_time(AFTER_TIME_OF_3_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&4, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&4, &2, &100);
        test_env.claim_failure(&4);

        test_env.set_time(AFTER_TIME_OF_4_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&5, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&5, &2, &100);
        test_env.claim_failure(&5);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        test_env.claim_failure(&5);
        test_env.claim_failure(&4);
        test_env.claim_failure(&3);
        test_env.claim_failure(&2);
        test_env.claim_failure(&1);

        test_env.set_sender("creator".to_owned());
        test_env.claim_success(&5, Some(&199));
        test_env.claim_success(&4, Some(&199));
        test_env.claim_success(&3, Some(&199));
        test_env.claim_success(&2, Some(&199));
        test_env.claim_success(&1, Some(&199));
    }

    #[test]
    fn claim_multiple_contests_25_fee() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(25, BASE_FEE_PERCENT_DENOMINATOR * 10));

        test_env.first_bet_on_contest_success(&1, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&1, &2, &100);
        test_env.claim_failure(&1);

        test_env.set_time(AFTER_TIME_OF_1_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&2, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&2, &2, &100);
        test_env.claim_failure(&2);

        test_env.set_time(AFTER_TIME_OF_2_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&3, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&3, &2, &100);
        test_env.claim_failure(&3);

        test_env.set_time(AFTER_TIME_OF_3_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&4, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&4, &2, &100);
        test_env.claim_failure(&4);

        test_env.set_time(AFTER_TIME_OF_4_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&5, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&5, &2, &100);
        test_env.claim_failure(&5);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        test_env.claim_failure(&5);
        test_env.claim_failure(&4);
        test_env.claim_failure(&3);
        test_env.claim_failure(&2);
        test_env.claim_failure(&1);

        test_env.set_sender("creator".to_owned());
        test_env.claim_success(&5, Some(&195));
        test_env.claim_success(&4, Some(&195));
        test_env.claim_success(&3, Some(&195));
        test_env.claim_success(&2, Some(&195));
        test_env.claim_success(&1, Some(&195));
    }

    #[test]
    fn cannot_claim_twice() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        test_env.first_bet_on_contest_success(&1, &1, &100);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        test_env.claim_success(&1, Some(&100));

        test_env.claim_failure(&1);
    }

    #[test]
    fn cannot_claim_contest_user_did_not_bet() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        test_env.first_bet_on_contest_success(&1, &1, &100);

        test_env.set_time(AFTER_TIME_OF_2_CLOSE);

        test_env.set_sender("user2".to_owned());

        test_env.claim_failure(&1);
    }
}
