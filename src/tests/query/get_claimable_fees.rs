#[cfg(test)]
mod tests {
    use cosmwasm_std::Uint128;

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
    fn get_claimable_fees_after_single_contest_bets_and_claim() {
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

        test_env.get_claimable_fees(Some(&Uint128::from(2u64)));
    }

    #[test]
    fn get_claimable_fees_after_multiple_contest_bets_and_claim() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        test_env.create_open_contest_success(&contest_file, &1, &100);

        let contest_file_2 = 2;
        test_env.create_open_contest_success(&contest_file_2, &1, &1000);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&contest_file, &2, &100);
        test_env.bet_on_contest_success(&contest_file_2, &2, &1000);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);
        test_env.claim_failure(&contest_file);

        test_env.set_sender("creator".to_owned());
        test_env.claim_success(&contest_file, Some(&198));
        test_env.claim_success(&contest_file_2, Some(&1980));

        test_env.get_claimable_fees(Some(&Uint128::from(22u64)));
    }

    #[test]
    fn get_claimable_fees_after_multiple_contest_bets_and_claim_2() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(2, BASE_FEE_PERCENT_DENOMINATOR));
        let contest_file = 1;
        test_env.create_open_contest_success(&contest_file, &1, &100);

        let contest_file_2 = 2;
        test_env.create_open_contest_success(&contest_file_2, &1, &1000);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&contest_file, &2, &100);
        test_env.bet_on_contest_success(&contest_file_2, &2, &1000);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);
        test_env.claim_failure(&contest_file);

        test_env.set_sender("creator".to_owned());
        test_env.claim_success(&contest_file, Some(&196));
        test_env.claim_success(&contest_file_2, Some(&1960));

        test_env.get_claimable_fees(Some(&Uint128::from(44u64)));
    }

    #[test]
    fn get_claimable_fees_after_multiple_contest_bets_and_claim_01() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(BASE_FEE_PERCENT_NUMERATOR, 1000));
        let contest_file = 1;
        test_env.create_open_contest_success(&contest_file, &1, &100);

        let contest_file_2 = 2;
        test_env.create_open_contest_success(&contest_file_2, &1, &1000);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&contest_file, &2, &100);
        test_env.bet_on_contest_success(&contest_file_2, &2, &1000);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);
        test_env.claim_failure(&contest_file);

        test_env.set_sender("creator".to_owned());
        test_env.claim_success(&contest_file, Some(&199));
        test_env.claim_success(&contest_file_2, Some(&1998));

        test_env.get_claimable_fees(Some(&Uint128::from(3u64)));
    }

    #[test]
    fn get_claimable_fees_after_multiple_contest_bets_and_claim_25() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(25, BASE_FEE_PERCENT_DENOMINATOR * 10));
        let contest_file = 1;
        test_env.create_open_contest_success(&contest_file, &1, &100);

        let contest_file_2 = 2;
        test_env.create_open_contest_success(&contest_file_2, &1, &1000);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&contest_file, &2, &100);
        test_env.bet_on_contest_success(&contest_file_2, &2, &1000);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);
        test_env.claim_failure(&contest_file);

        test_env.set_sender("creator".to_owned());
        test_env.claim_success(&contest_file, Some(&195));
        test_env.claim_success(&contest_file_2, Some(&1950));

        test_env.get_claimable_fees(Some(&Uint128::from(55u64)));
    }

    #[test]
    fn get_claimable_fees_after_bets_and_claim() {
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

        test_env.get_claimable_fees(None);
    }
}
