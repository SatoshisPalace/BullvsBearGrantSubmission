#[cfg(test)]
mod tests {
    use crate::{
        data::state::FeePercent,
        tests::{
            constants::{
                AFTER_TIME_OF_10_CLOSE, AFTER_TIME_OF_11_CLOSE, AFTER_TIME_OF_1_CLOSE,
                AFTER_TIME_OF_2_CLOSE, AFTER_TIME_OF_3_CLOSE, AFTER_TIME_OF_4_CLOSE,
                AFTER_TIME_OF_5_CLOSE, AFTER_TIME_OF_6_CLOSE, AFTER_TIME_OF_7_CLOSE,
                AFTER_TIME_OF_8_CLOSE, AFTER_TIME_OF_9_CLOSE, AFTER_TIME_OF_RESOLVE,
                BASE_FEE_PERCENT_DENOMINATOR, BASE_FEE_PERCENT_NUMERATOR,
            },
            test_env::tests::TestEnv,
        },
    };

    ////////TESTS////////
    #[test]
    fn total_number_with_no_contests() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));

        test_env.get_number_of_bets(None);
    }

    #[test]
    fn total_number_with_one_contest() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);

        test_env.get_number_of_bets(Some(&1));
    }

    #[test]
    fn includes_resolved_contests() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);
        test_env.get_number_of_bets(Some(&1));
    }

    #[test]
    fn total_number_bets_below_minimum_are_ignored() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let minimum_bet = 100;
        test_env.set_minimum_bet_success(&minimum_bet);

        let mut contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_fail(&contest_file, &1, &99);

        test_env.set_time(AFTER_TIME_OF_1_CLOSE);

        test_env.set_sender("creator".to_owned());
        contest_file = 2;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&contest_file, &1, &100);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        test_env.get_number_of_bets(Some(&3));
    }

    #[test]
    fn functions_properly_with_many_contests() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let mut contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);
        test_env.set_time(AFTER_TIME_OF_1_CLOSE);

        contest_file = 2;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);
        test_env.set_time(AFTER_TIME_OF_2_CLOSE);
        test_env.get_number_of_bets(Some(&2));

        test_env.claim_multiple_success(vec![&1], Some(&100));
        contest_file = 3;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);
        test_env.set_time(AFTER_TIME_OF_3_CLOSE);

        test_env.get_number_of_bets(Some(&3));
        test_env.claim_multiple_success(vec![&2], Some(&100));

        contest_file = 4;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);

        test_env.set_time(AFTER_TIME_OF_4_CLOSE);

        test_env.claim_multiple_success(vec![&3], Some(&100));

        contest_file = 5;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);

        test_env.set_time(AFTER_TIME_OF_5_CLOSE);
        test_env.get_number_of_bets(Some(&5));

        contest_file = 6;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);
        test_env.set_time(AFTER_TIME_OF_6_CLOSE);
        test_env.claim_multiple_success(vec![&4, &5], Some(&200));

        contest_file = 7;
        test_env.first_bet_on_contest_success(&contest_file, &2, &100);
        test_env.set_time(AFTER_TIME_OF_7_CLOSE);

        contest_file = 8;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);
        test_env.set_time(AFTER_TIME_OF_8_CLOSE);
        test_env.get_number_of_bets(Some(&8));

        contest_file = 9;
        test_env.first_bet_on_contest_success(&contest_file, &2, &100);
        test_env.set_time(AFTER_TIME_OF_9_CLOSE);

        contest_file = 10;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);
        test_env.set_time(AFTER_TIME_OF_10_CLOSE);
        test_env.get_number_of_bets(Some(&10));

        contest_file = 11;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);
        test_env.set_time(AFTER_TIME_OF_11_CLOSE);
        test_env.get_number_of_bets(Some(&11));

        contest_file = 12;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);
        test_env.set_time(AFTER_TIME_OF_RESOLVE);
        test_env.get_number_of_bets(Some(&12));
    }
}
