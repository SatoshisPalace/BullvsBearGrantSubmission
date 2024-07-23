#[cfg(test)]
mod tests {
    use crate::{
        data::state::FeePercent,
        tests::{
            constants::{
                AFTER_TIME_OF_RESOLVE, BASE_FEE_PERCENT_DENOMINATOR, BASE_FEE_PERCENT_NUMERATOR,
            },
            test_env::tests::TestEnv,
        },
    };

    ////////TESTS////////
    #[test]
    fn claim_fee() {
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

        test_env.claim_fees_success(Some(&2));
    }

    #[test]
    fn claim_fee_2() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(2, BASE_FEE_PERCENT_DENOMINATOR));
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
    fn claim_fee_01() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR * 10,
        ));
        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&contest_file, &2, &100);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);
        test_env.claim_failure(&contest_file);

        test_env.set_sender("creator".to_owned());
        test_env.claim_success(&contest_file, Some(&199));

        test_env.claim_fees_success(Some(&1));
    }

    #[test]
    fn claim_fee_25() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(25, BASE_FEE_PERCENT_DENOMINATOR * 10));
        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&contest_file, &2, &100);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);
        test_env.claim_failure(&contest_file);

        test_env.set_sender("creator".to_owned());
        test_env.claim_success(&contest_file, Some(&195));

        test_env.claim_fees_success(Some(&5));
    }

    #[test]
    fn cannot_claim_fee_non_owner() {
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

        test_env.set_sender("Not Owner".to_owned());

        test_env.claim_fees_failure();
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
            print!("{}", user);
            test_env.claim_success(&contest_file, Some(&amount_bet));
        }

        // Creator tries to claim fees
        // Option 1: Expecting an error
        test_env.claim_fees_failure();

        // Option 2: Expecting zero fee to claim (commented out)
        // test_env.claim_fees_success(Some(&0));
    }
}
