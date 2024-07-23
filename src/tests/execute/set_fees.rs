#[cfg(test)]
mod tests {
    use crate::{
        data::state::FeePercent,
        tests::{
            constants::{
                AFTER_TIME_OF_1_CLOSE, AFTER_TIME_OF_2_CLOSE, AFTER_TIME_OF_3_CLOSE,
                AFTER_TIME_OF_RESOLVE, BASE_FEE_PERCENT_DENOMINATOR, BASE_FEE_PERCENT_NUMERATOR,
            },
            test_env::tests::TestEnv,
        },
    };

    ////////TESTS////////
    #[test]
    fn claim_fee_post_update() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let new_fee = FeePercent::new(2, 100);

        test_env.set_fee_success(new_fee);

        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&contest_file, &2, &100);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);
        test_env.claim_failure(&contest_file);

        test_env.set_sender("creator".to_owned());
        test_env.claim_success(&contest_file, Some(&196));

        test_env.claim_fees_success(Some(&4));
    }

    #[test]
    fn claim_fee_post_update_to_0() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let new_fee = FeePercent::new(0, 0);

        test_env.set_fee_success(new_fee);

        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&contest_file, &2, &100);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);
        test_env.claim_failure(&contest_file);

        test_env.set_sender("creator".to_owned());
        test_env.claim_success(&contest_file, Some(&200));

        test_env.claim_fees_success(Some(&0));
    }

    #[test]
    fn fail_post_update_not_owner() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let new_fee = FeePercent::new(2, 100);
        test_env.set_sender("user2".to_owned());
        test_env.set_fee_fail(new_fee);
    }

    #[test]
    fn init_fee_0() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(0, 0));
        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&contest_file, &2, &100);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);
        test_env.claim_failure(&contest_file);

        test_env.set_sender("creator".to_owned());
        test_env.claim_success(&contest_file, Some(&200));

        test_env.claim_fees_success(Some(&0));
    }

    #[test]
    fn init_fee_0_changes_1() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(0, 0));
        let mut contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&contest_file, &2, &100);

        let new_fee = FeePercent::new(1, 100);
        test_env.set_sender("creator".to_owned());
        test_env.set_fee_success(new_fee);

        test_env.set_time(AFTER_TIME_OF_1_CLOSE);
        test_env.bet_on_contest_success(&2, &1, &100);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&2, &2, &100);
        test_env.set_time(AFTER_TIME_OF_2_CLOSE);

        test_env.claim_failure(&contest_file);

        test_env.set_sender("creator".to_owned());

        test_env.claim_success(&contest_file, Some(&200));

        contest_file = 2;

        test_env.claim_fees_success(Some(&0));

        test_env.set_time(AFTER_TIME_OF_3_CLOSE);

        test_env.claim_success(&contest_file, Some(&198));

        test_env.claim_fees_success(Some(&2));
    }

    #[test]
    fn claim_contest_with_opposition_10_users_and_fee_change_on_multiple_files() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(0, 1)); // Set initial fee to 0%
        let amount_bet = 100;
        let users = vec![
            "user1", "user2", "user3", "user4", "user5", "user6", "user7", "user8", "user9",
            "user10",
        ];

        // Contest File 1
        let contest_file_1 = 1;

        // First half of users bet on one side of Contest File 1
        for (i, user) in users.iter().take(5).enumerate() {
            test_env.set_sender(user.to_string());
            if i == 0 {
                test_env.first_bet_on_contest_success(&contest_file_1, &1, &amount_bet);
            } else {
                test_env.bet_on_contest_success(&contest_file_1, &1, &amount_bet);
            }
        }

        // Change the fee to 1% after the first half have bet
        let new_fee = FeePercent::new(1, 100);
        test_env.set_sender("creator".to_owned());
        test_env.set_fee_success(new_fee);

        // Remaining users bet on the other side of Contest File 1
        for user in users.iter().skip(5) {
            test_env.set_sender(user.to_string());
            test_env.bet_on_contest_success(&contest_file_1, &2, &amount_bet);
        }

        // Set time to after the resolution time for Contest File 1
        test_env.set_time(AFTER_TIME_OF_1_CLOSE);
        // Contest File 2
        let contest_file_2 = 2;

        // All users bet on Contest File 2
        for (i, user) in users.iter().enumerate() {
            test_env.set_sender(user.to_string());
            if i == 0 {
                test_env.first_bet_on_contest_success(&contest_file_2, &1, &amount_bet);
            } else if i < users.len() / 2 {
                test_env.bet_on_contest_success(&contest_file_2, &1, &amount_bet);
            } else {
                test_env.bet_on_contest_success(&contest_file_2, &2, &amount_bet);
            }
        }

        // Set time to after the close time for Contest File 2 and resolve time for file 1
        test_env.set_time(AFTER_TIME_OF_2_CLOSE);

        // Users who bet on the winning side of Contest File 1 claim their winnings
        for (i, user) in users.iter().enumerate() {
            test_env.set_sender(user.to_string());
            if i < users.len() / 2 {
                // Users on the winning side should succeed in claiming
                test_env.claim_success(
                    &contest_file_1,
                    Some(&(((users.len() as u128) * amount_bet) / (users.len() / 2) as u128)),
                ); // Example calculation for expected amount
            } else {
                // Users on the losing side should fail in claiming
                test_env.claim_failure(&contest_file_1);
            }
        }

        // Set time to after the resolve time for Contest File 2
        test_env.set_time(AFTER_TIME_OF_3_CLOSE);

        // Users who bet on the winning side of Contest File 2 claim their winnings, with 1% fee applied
        let total_bets = (users.len() as u128) * amount_bet;
        let winners = users.len() / 2;
        let amount_per_winner = total_bets / winners as u128;
        let amount_after_fee = amount_per_winner - (amount_per_winner / 100); // Apply 1% fee

        for (i, user) in users.iter().enumerate() {
            test_env.set_sender(user.to_string());
            if i < users.len() / 2 {
                // Users on the winning side should succeed in claiming
                test_env.claim_success(&contest_file_2, Some(&amount_after_fee));
            } else {
                // Users on the losing side should fail in claiming
                test_env.claim_failure(&contest_file_2);
            }
        }
    }

    #[test]
    fn claim_contest_with_skewed_opposition_and_fee_change() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(0, 1)); // Set initial fee to 0%
        let amount_bet = 100;
        let users = vec![
            "user1", "user2", "user3", "user4", "user5", "user6", "user7", "user8", "user9",
            "user10",
        ];

        // Contest File 1
        let contest_file_1 = 1;

        // 3 users bet on side 1, 7 users bet on side 2
        for (i, user) in users.iter().enumerate() {
            test_env.set_sender(user.to_string());
            if i == 0 {
                test_env.first_bet_on_contest_success(&contest_file_1, &1, &amount_bet);
            } else if i < 3 {
                test_env.bet_on_contest_success(&contest_file_1, &1, &amount_bet);
            } else {
                test_env.bet_on_contest_success(&contest_file_1, &2, &amount_bet);
            }
        }

        // Change the fee to 1% after the first 3 users have bet
        let new_fee = FeePercent::new(1, 100);
        test_env.set_sender("creator".to_owned());
        test_env.set_fee_success(new_fee);

        // Set time to after the resolution time for Contest File 1
        test_env.set_time(AFTER_TIME_OF_1_CLOSE);

        // Calculate the total pool and the expected amount for each winning user
        let total_pool = users.len() as u128 * amount_bet;
        let expected_payout_per_winner = total_pool / 3; // Since there are 3 winners

        // Contest File 2
        let contest_file_2 = 2;

        // 3 users bet on side 1, 7 users bet on side 2
        for (i, user) in users.iter().enumerate() {
            test_env.set_sender(user.to_string());
            if i == 0 {
                test_env.first_bet_on_contest_success(&contest_file_2, &1, &amount_bet);
            } else if i < 3 {
                test_env.bet_on_contest_success(&contest_file_2, &1, &amount_bet);
            } else {
                test_env.bet_on_contest_success(&contest_file_2, &2, &amount_bet);
            }
        }

        // Set time to after the close time for Contest File 2 and resolution time for Contest File 1
        test_env.set_time(AFTER_TIME_OF_2_CLOSE);

        // Users who bet on the winning side claim their winnings
        for (i, user) in users.iter().enumerate() {
            test_env.set_sender(user.to_string());
            if i < 3 {
                // Users on the winning side should succeed in claiming
                test_env.claim_success(
                    &contest_file_1,
                    Some(&expected_payout_per_winner), // Each winning user gets their share of the total pool
                );
            } else {
                // Users on the losing side should fail in claiming
                test_env.claim_failure(&contest_file_1);
            }
        }

        test_env.set_time(AFTER_TIME_OF_3_CLOSE);

        // Users who bet on the winning side of Contest File 2 claim their winnings, with 1% fee applied
        let total_bets = (users.len() as u128) * amount_bet;
        let winners = 3; // Only 3 winners in this skewed scenario
        let amount_per_winner = total_bets / winners as u128;
        let amount_after_fee = amount_per_winner - (amount_per_winner / 100); // Apply 1% fee

        for (i, user) in users.iter().enumerate() {
            test_env.set_sender(user.to_string());
            if i < 3 {
                // Users on the winning side should succeed in claiming
                test_env.claim_success(&contest_file_2, Some(&amount_after_fee));
            } else {
                // Users on the losing side should fail in claiming
                test_env.claim_failure(&contest_file_2);
            }
        }
    }
}
