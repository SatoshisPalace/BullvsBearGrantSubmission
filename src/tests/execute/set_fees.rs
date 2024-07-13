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
}
