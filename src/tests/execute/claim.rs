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
    fn cannot_claim_open_contest() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        test_env.create_open_contest_success(&contest_file, &1, &100);

        test_env.claim_failure(&contest_file);
    }

    #[test]
    fn cannot_closed_awaiting_results_contest() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        test_env.create_open_contest_success(&contest_file, &1, &100);

        test_env.set_time(4102462800);

        test_env.claim_failure(&contest_file);
    }

    #[test]
    fn claim_contest_no_opposition() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        let amount_bet = 100;
        test_env.create_open_contest_success(&contest_file, &1, &amount_bet);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        test_env.claim_success(&contest_file, Some(&amount_bet));
    }

    #[test]
    fn claim_contest_bets_on_both_sides() {
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
    }

    #[test]
    fn claim_multiple_contests() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        test_env.create_open_contest_success(&1, &1, &100);
        test_env.create_open_contest_success(&2, &1, &100);
        test_env.create_open_contest_success(&3, &1, &100);
        test_env.create_open_contest_success(&4, &1, &100);
        test_env.create_open_contest_success(&5, &1, &100);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&1, &2, &100);
        test_env.bet_on_contest_success(&2, &2, &100);
        test_env.bet_on_contest_success(&3, &2, &100);
        test_env.bet_on_contest_success(&4, &2, &100);
        test_env.bet_on_contest_success(&5, &2, &100);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        test_env.claim_failure(&5);
        test_env.claim_failure(&4);
        test_env.claim_failure(&3);
        test_env.claim_failure(&2);
        test_env.claim_failure(&1);

        test_env.set_sender("creator".to_owned());
        test_env.claim_success(&5, Some(&198));
        test_env.claim_success(&4, Some(&198));
        test_env.claim_success(&3, Some(&198));
        test_env.claim_success(&2, Some(&198));
        test_env.claim_success(&1, Some(&198));
    }

    #[test]
    fn claim_multiple_contests_2_fee() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(2, BASE_FEE_PERCENT_DENOMINATOR));
        test_env.create_open_contest_success(&1, &1, &100);
        test_env.create_open_contest_success(&2, &1, &100);
        test_env.create_open_contest_success(&3, &1, &100);
        test_env.create_open_contest_success(&4, &1, &100);
        test_env.create_open_contest_success(&5, &1, &100);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&1, &2, &100);
        test_env.bet_on_contest_success(&2, &2, &100);
        test_env.bet_on_contest_success(&3, &2, &100);
        test_env.bet_on_contest_success(&4, &2, &100);
        test_env.bet_on_contest_success(&5, &2, &100);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        test_env.claim_failure(&5);
        test_env.claim_failure(&4);
        test_env.claim_failure(&3);
        test_env.claim_failure(&2);
        test_env.claim_failure(&1);

        test_env.set_sender("creator".to_owned());
        test_env.claim_success(&5, Some(&196));
        test_env.claim_success(&4, Some(&196));
        test_env.claim_success(&3, Some(&196));
        test_env.claim_success(&2, Some(&196));
        test_env.claim_success(&1, Some(&196));
    }

    #[test]
    fn claim_multiple_contests_01_fee() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(1, BASE_FEE_PERCENT_DENOMINATOR * 10));
        test_env.create_open_contest_success(&1, &1, &100);
        test_env.create_open_contest_success(&2, &1, &100);
        test_env.create_open_contest_success(&3, &1, &100);
        test_env.create_open_contest_success(&4, &1, &100);
        test_env.create_open_contest_success(&5, &1, &100);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&1, &2, &100);
        test_env.bet_on_contest_success(&2, &2, &100);
        test_env.bet_on_contest_success(&3, &2, &100);
        test_env.bet_on_contest_success(&4, &2, &100);
        test_env.bet_on_contest_success(&5, &2, &100);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        test_env.claim_failure(&5);
        test_env.claim_failure(&4);
        test_env.claim_failure(&3);
        test_env.claim_failure(&2);
        test_env.claim_failure(&1);

        test_env.set_sender("creator".to_owned());
        test_env.claim_success(&5, Some(&199));
        test_env.claim_success(&4, Some(&199));
        test_env.claim_success(&3, Some(&199));
        test_env.claim_success(&2, Some(&199));
        test_env.claim_success(&1, Some(&199));
    }

    #[test]
    fn claim_multiple_contests_25_fee() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(25, BASE_FEE_PERCENT_DENOMINATOR * 10));
        test_env.create_open_contest_success(&1, &1, &100);
        test_env.create_open_contest_success(&2, &1, &100);
        test_env.create_open_contest_success(&3, &1, &100);
        test_env.create_open_contest_success(&4, &1, &100);
        test_env.create_open_contest_success(&5, &1, &100);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&1, &2, &100);
        test_env.bet_on_contest_success(&2, &2, &100);
        test_env.bet_on_contest_success(&3, &2, &100);
        test_env.bet_on_contest_success(&4, &2, &100);
        test_env.bet_on_contest_success(&5, &2, &100);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        test_env.claim_failure(&5);
        test_env.claim_failure(&4);
        test_env.claim_failure(&3);
        test_env.claim_failure(&2);
        test_env.claim_failure(&1);

        test_env.set_sender("creator".to_owned());
        test_env.claim_success(&5, Some(&195));
        test_env.claim_success(&4, Some(&195));
        test_env.claim_success(&3, Some(&195));
        test_env.claim_success(&2, Some(&195));
        test_env.claim_success(&1, Some(&195));
    }

    #[test]
    fn cannot_claim_twice() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        test_env.create_open_contest_success(&1, &1, &100);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        test_env.claim_success(&1, Some(&100));

        test_env.claim_failure(&1);
    }
}
