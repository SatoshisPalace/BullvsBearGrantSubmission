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
        test_env.create_open_contest_success(&contest_file, &1, &100);

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
        test_env.create_open_contest_success(&contest_file, &1, &100);

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
        test_env.create_open_contest_success(&contest_file, &1, &100);

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
        test_env.create_open_contest_success(&contest_file, &1, &100);

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
        test_env.create_open_contest_success(&contest_file, &1, &100);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&contest_file, &2, &100);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);
        test_env.claim_failure(&contest_file);

        test_env.set_sender("creator".to_owned());
        test_env.claim_success(&contest_file, Some(&198));

        test_env.set_sender("Not Owner".to_owned());

        test_env.claim_fees_failure();
    }
}
