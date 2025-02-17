#[cfg(test)]
mod tests {
    use crate::tests::{
        constants::{
            AFTER_TIME_OF_1_CLOSE, AFTER_TIME_OF_2_CLOSE, AFTER_TIME_OF_3_CLOSE,
            AFTER_TIME_OF_4_CLOSE,
        },
        test_env::tests::TestEnv,
    };
    use crate::{
        data::state::FeePercent,
        tests::constants::{BASE_FEE_PERCENT_DENOMINATOR, BASE_FEE_PERCENT_NUMERATOR},
    };

    ////////TESTS////////
    #[test]
    fn get_contest_after_creation() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);
        test_env.get_contest_success(&1);
    }

    #[test]
    fn get_contest_many_times() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        test_env.first_bet_on_contest_success(&1, &1, &100);
        test_env.set_time(AFTER_TIME_OF_1_CLOSE);
        test_env.first_bet_on_contest_success(&2, &1, &100);
        test_env.set_time(AFTER_TIME_OF_2_CLOSE);
        test_env.first_bet_on_contest_success(&3, &1, &100);
        test_env.set_time(AFTER_TIME_OF_3_CLOSE);
        test_env.first_bet_on_contest_success(&4, &1, &100);
        test_env.set_time(AFTER_TIME_OF_4_CLOSE);
        test_env.first_bet_on_contest_success(&5, &1, &100);

        test_env.get_contest_success(&1);
        test_env.get_contest_success(&2);
        test_env.get_contest_success(&3);
        test_env.get_contest_success(&4);
        test_env.get_contest_success(&5);
    }

    #[test]
    fn get_contest_after_bet() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&contest_file, &2, &100);

        test_env.get_contest_success(&1);
    }
}
