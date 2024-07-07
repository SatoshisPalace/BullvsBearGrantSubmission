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
    fn total_number_with_one_contest() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);

        test_env.last_ten_contests_has_length(&1);
        test_env.last_ten_contests_includes_contest(&contest_file);
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
        test_env.last_ten_contests_has_length(&1);
        test_env.last_ten_contests_includes_contest(&contest_file);
    }

    #[test]
    fn functions_properly_with_more_than_ten() {
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
        test_env.last_ten_contests_has_length(&2);
        test_env.last_ten_contests_includes_contest(&1);
        test_env.claim_multiple_success(vec![&1], Some(&100));
        contest_file = 3;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);
        test_env.set_time(AFTER_TIME_OF_3_CLOSE);

        test_env.last_ten_contests_has_length(&3);
        test_env.claim_multiple_success(vec![&2], Some(&100));

        contest_file = 4;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);

        test_env.set_time(AFTER_TIME_OF_4_CLOSE);
        test_env.last_ten_contests_has_length(&4);

        test_env.claim_multiple_success(vec![&3], Some(&100));

        contest_file = 5;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);

        test_env.set_time(AFTER_TIME_OF_5_CLOSE);
        test_env.last_ten_contests_has_length(&5);

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
        test_env.last_ten_contests_has_length(&8);

        contest_file = 9;
        test_env.first_bet_on_contest_success(&contest_file, &2, &100);
        test_env.set_time(AFTER_TIME_OF_9_CLOSE);

        contest_file = 10;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);
        test_env.set_time(AFTER_TIME_OF_10_CLOSE);
        test_env.last_ten_contests_has_length(&10);

        contest_file = 11;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);
        test_env.set_time(AFTER_TIME_OF_11_CLOSE);
        test_env.last_ten_contests_has_length(&10);
        test_env.last_ten_contests_includes_contest(&contest_file);

        contest_file = 1;
        test_env.last_ten_contests_does_not_include_contest(&contest_file);

        contest_file = 12;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);
        test_env.set_time(AFTER_TIME_OF_RESOLVE);
        test_env.last_ten_contests_has_length(&10);
        test_env.last_ten_contests_includes_contest(&contest_file);

        contest_file = 2;
        test_env.last_ten_contests_does_not_include_contest(&contest_file);
    }
}
