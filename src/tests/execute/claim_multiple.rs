#[cfg(test)]
mod tests {
    use crate::data::state::FeePercent;
    use crate::tests::{
        constants::{
            AFTER_TIME_OF_1_CLOSE, AFTER_TIME_OF_2_CLOSE, AFTER_TIME_OF_3_CLOSE,
            AFTER_TIME_OF_4_CLOSE, AFTER_TIME_OF_RESOLVE, BASE_FEE_PERCENT_DENOMINATOR,
            BASE_FEE_PERCENT_NUMERATOR,
        },
        test_env::tests::TestEnv,
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
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);

        test_env.claim_multiple_failure(vec![&contest_file]);
    }

    #[test]
    fn cannot_closed_awaiting_results_contest() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);

        test_env.set_time(AFTER_TIME_OF_1_CLOSE);

        test_env.claim_multiple_failure(vec![&contest_file]);
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
        test_env.first_bet_on_contest_success(&contest_file, &1, &amount_bet);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        test_env.claim_multiple_success(vec![&contest_file], Some(&amount_bet));
    }

    #[test]
    fn claim_contest_bets_on_both_sides() {
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
        test_env.claim_multiple_failure(vec![&contest_file]);

        test_env.set_sender("creator".to_owned());
        test_env.claim_multiple_success(vec![&contest_file], Some(&198));
    }

    #[test]
    fn claim_multiple_contests() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        test_env.first_bet_on_contest_success(&1, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&1, &2, &100);
        test_env.claim_failure(&1);

        test_env.set_time(AFTER_TIME_OF_1_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&2, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&2, &2, &100);
        test_env.claim_failure(&2);

        test_env.set_time(AFTER_TIME_OF_2_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&3, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&3, &2, &100);
        test_env.claim_failure(&3);

        test_env.set_time(AFTER_TIME_OF_3_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&4, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&4, &2, &100);
        test_env.claim_failure(&4);

        test_env.set_time(AFTER_TIME_OF_4_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&5, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&5, &2, &100);
        test_env.claim_failure(&5);

        test_env.set_time(AFTER_TIME_OF_1_CLOSE);
        test_env.claim_multiple_failure(vec![&1, &2, &3, &4, &5]);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        test_env.set_sender("creator".to_owned());
        test_env.claim_multiple_success(vec![&1, &2, &3, &4, &5], Some(&990));
    }

    #[test]
    fn claim_multiple_contests_2_fee() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(2, BASE_FEE_PERCENT_DENOMINATOR));
        test_env.first_bet_on_contest_success(&1, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&1, &2, &100);
        test_env.claim_failure(&1);

        test_env.set_time(AFTER_TIME_OF_1_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&2, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&2, &2, &100);
        test_env.claim_failure(&2);

        test_env.set_time(AFTER_TIME_OF_2_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&3, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&3, &2, &100);
        test_env.claim_failure(&3);

        test_env.set_time(AFTER_TIME_OF_3_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&4, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&4, &2, &100);
        test_env.claim_failure(&4);

        test_env.set_time(AFTER_TIME_OF_4_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&5, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&5, &2, &100);
        test_env.claim_failure(&5);

        test_env.set_time(AFTER_TIME_OF_1_CLOSE);
        test_env.claim_multiple_failure(vec![&1, &2, &3, &4, &5]);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        test_env.claim_multiple_failure(vec![&1, &2, &3, &4, &5]);

        test_env.set_sender("creator".to_owned());
        test_env.claim_multiple_success(vec![&1, &2, &3, &4, &5], Some(&980));
    }

    #[test]
    fn claim_multiple_contests_01_fee() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR * 10,
        ));
        test_env.first_bet_on_contest_success(&1, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&1, &2, &100);
        test_env.claim_failure(&1);

        test_env.set_time(AFTER_TIME_OF_1_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&2, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&2, &2, &100);
        test_env.claim_failure(&2);

        test_env.set_time(AFTER_TIME_OF_2_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&3, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&3, &2, &100);
        test_env.claim_failure(&3);

        test_env.set_time(AFTER_TIME_OF_3_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&4, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&4, &2, &100);
        test_env.claim_failure(&4);

        test_env.set_time(AFTER_TIME_OF_4_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&5, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&5, &2, &100);
        test_env.claim_failure(&5);

        test_env.set_time(AFTER_TIME_OF_1_CLOSE);
        test_env.claim_multiple_failure(vec![&1, &2, &3, &4, &5]);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        test_env.claim_multiple_failure(vec![&1, &2, &3, &4, &5]);

        test_env.set_sender("creator".to_owned());
        test_env.claim_multiple_success(vec![&1, &2, &3, &4, &5], Some(&995));
    }

    #[test]
    fn claim_multiple_contests_25_fee() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(25, BASE_FEE_PERCENT_DENOMINATOR * 10));
        test_env.first_bet_on_contest_success(&1, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&1, &2, &100);
        test_env.claim_failure(&1);

        test_env.set_time(AFTER_TIME_OF_1_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&2, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&2, &2, &100);
        test_env.claim_failure(&2);

        test_env.set_time(AFTER_TIME_OF_2_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&3, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&3, &2, &100);
        test_env.claim_failure(&3);

        test_env.set_time(AFTER_TIME_OF_3_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&4, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&4, &2, &100);
        test_env.claim_failure(&4);

        test_env.set_time(AFTER_TIME_OF_4_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&5, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&5, &2, &100);
        test_env.claim_failure(&5);

        test_env.set_time(AFTER_TIME_OF_1_CLOSE);
        test_env.claim_multiple_failure(vec![&1, &2, &3, &4, &5]);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        test_env.claim_multiple_failure(vec![&1, &2, &3, &4, &5]);

        test_env.set_sender("creator".to_owned());
        test_env.claim_multiple_success(vec![&1, &2, &3, &4, &5], Some(&975));
    }

    #[test]
    fn cannot_claim_twice() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        test_env.claim_multiple_success(vec![&contest_file], Some(&100));

        test_env.claim_multiple_failure(vec![&contest_file]);
    }
}
