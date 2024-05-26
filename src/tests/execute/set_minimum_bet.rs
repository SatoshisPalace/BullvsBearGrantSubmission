#[cfg(test)]
mod tests {
    use crate::{
        data::state::FeePercent,
        tests::{
            constants::{BASE_FEE_PERCENT_DENOMINATOR, BASE_FEE_PERCENT_NUMERATOR},
            test_env::tests::TestEnv,
        },
    };

    ////////TESTS////////
    #[test]
    fn set_min_before_any_contests() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        test_env.set_minimum_bet_success(&100)
    }

    #[test]
    fn set_min_before_after_first_contest() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        test_env.create_open_contest_success(&contest_file, &1, &100);
        test_env.set_minimum_bet_success(&100)
    }

    #[test]
    fn set_min_before_after_bet() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        test_env.create_open_contest_success(&contest_file, &1, &100);
        test_env.bet_on_contest_success(&contest_file, &1, &100);
        test_env.set_minimum_bet_success(&100)
    }

    #[test]
    fn set_min_bet_many_times() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        test_env.create_open_contest_success(&contest_file, &1, &100);
        test_env.set_minimum_bet_success(&1);
        test_env.set_minimum_bet_success(&2);
        test_env.set_minimum_bet_success(&3);
        test_env.set_minimum_bet_success(&4);
        test_env.set_minimum_bet_success(&5);
        test_env.set_minimum_bet_success(&6);
        test_env.set_minimum_bet_success(&7);
        test_env.set_minimum_bet_success(&8);
    }

    #[test]
    fn cannot_set_min_bet_if_not_admin() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        test_env.set_sender("user2".to_owned());
        test_env.set_minimum_bet_fail(&100);
    }
}
