#[cfg(test)]
mod tests {
    use cosmwasm_std::Uint128;

    use crate::{
        data::state::FeePercent,
        tests::{
            constants::{AFTER_TIME_OF_1_CLOSE, AFTER_TIME_OF_3_CLOSE, AFTER_TIME_OF_RESOLVE},
            constants::{BASE_FEE_PERCENT_DENOMINATOR, BASE_FEE_PERCENT_NUMERATOR},
            test_env::tests::TestEnv,
        },
    };

    ////////TESTS////////
    #[test]
    fn claimable_value_before_close() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&contest_file, &2, &100);
        test_env.set_sender("creator".to_owned());

        test_env.query_claimable_value(Uint128::zero());
    }

    #[test]
    fn claimable_value_correct_1_contest() {
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
        test_env.set_sender("creator".to_owned());

        test_env.query_claimable_value(Uint128::from(198u32));
    }

    #[test]
    fn claimable_value_none_to_claim() {
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

        test_env.query_claimable_value(Uint128::zero());
    }

    #[test]
    fn get_claimable_value_after_multiple_contest_bets() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&contest_file, &2, &100);

        test_env.set_time(AFTER_TIME_OF_1_CLOSE);
        test_env.set_sender("creator".to_owned());

        let contest_file_2 = 2;
        test_env.first_bet_on_contest_success(&contest_file_2, &1, &1000);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&contest_file_2, &2, &1000);

        test_env.set_time(AFTER_TIME_OF_3_CLOSE);
        test_env.claim_failure(&contest_file);

        test_env.set_sender("creator".to_owned());

        test_env.query_claimable_value(Uint128::from(2178u64));
    }

    #[test]
    fn get_claimable_value_after_multiple_contest_bets_0() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&contest_file, &2, &100);

        test_env.set_time(AFTER_TIME_OF_1_CLOSE);
        test_env.set_sender("creator".to_owned());

        let contest_file_2 = 2;
        test_env.first_bet_on_contest_success(&contest_file_2, &1, &1000);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&contest_file_2, &2, &1000);

        test_env.set_time(AFTER_TIME_OF_3_CLOSE);
        test_env.claim_failure(&contest_file);

        test_env.query_claimable_value(Uint128::zero());
    }
}
