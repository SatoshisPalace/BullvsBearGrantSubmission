#[cfg(test)]
mod tests {
    use crate::{
        data::state::FeePercent,
        tests::{
            constants::{
                AFTER_TIME_OF_1_CLOSE, AFTER_TIME_OF_2_CLOSE, AFTER_TIME_OF_3_CLOSE,
                AFTER_TIME_OF_4_CLOSE, AFTER_TIME_OF_RESOLVE, BASE_FEE_PERCENT_DENOMINATOR,
                BASE_FEE_PERCENT_NUMERATOR,
            },
            test_env::tests::TestEnv,
        },
    };

    ////////TESTS////////
    #[test]
    fn users_list_of_bets_with_one_bet() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);
        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        test_env.users_list_of_bets_has_length(vec![0], 1);
        test_env.users_list_of_bets_includes_contest(vec![0], &contest_file);
    }

    #[test]
    fn users_list_of_bets_multiple_bets_on_same_contest_are_treated_as_one() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);
        test_env.bet_on_contest_success(&contest_file, &1, &100);
        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        test_env.users_list_of_bets_has_length(vec![0], 1);
        test_env.users_list_of_bets_includes_contest(vec![0], &contest_file);
    }

    #[test]
    fn users_list_of_bets_below_minimum_are_ignored() {
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

        test_env.users_list_of_bets_has_length(vec![0, 1], 1);
        test_env.users_list_of_bets_includes_contest(vec![0, 1], &contest_file);
    }

    #[test]
    fn includes_closed_awaiting_results_contests() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);

        test_env.set_time(AFTER_TIME_OF_1_CLOSE);
        test_env.users_list_of_bets_has_length(vec![0], 1);
        test_env.users_list_of_bets_includes_contest(vec![0], &contest_file);
    }

    #[test]
    fn includes_claimable_contests() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);
        test_env.users_list_of_bets_has_length(vec![0], 1);
        test_env.users_list_of_bets_includes_contest(vec![0], &contest_file);
    }

    #[test]
    fn includes_many() {
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

        contest_file = 3;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);
        test_env.set_time(AFTER_TIME_OF_3_CLOSE);

        contest_file = 4;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);
        test_env.set_time(AFTER_TIME_OF_4_CLOSE);

        contest_file = 5;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);
        test_env.users_list_of_bets_has_length(vec![0, 1, 2, 3, 4, 5], 5);
        test_env.users_list_of_bets_includes_contest(vec![0, 1, 2, 3, 4, 5], &contest_file);
    }

    #[test]
    fn functions_properly_after_claim_multiple_steps() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let mut contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);
        test_env.set_time(AFTER_TIME_OF_1_CLOSE);

        test_env.ensure_index_incrementing(None);
        contest_file = 2;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);
        test_env.set_time(AFTER_TIME_OF_2_CLOSE);

        test_env.claim_multiple_success(vec![&1], Some(&100));

        contest_file = 3;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);
        test_env.set_time(AFTER_TIME_OF_3_CLOSE);

        test_env.claim_multiple_success(vec![&2], Some(&100));

        contest_file = 4;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);

        test_env.set_time(AFTER_TIME_OF_4_CLOSE);

        test_env.claim_multiple_success(vec![&3], Some(&100));

        contest_file = 5;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);
        test_env.users_list_of_bets_has_length(vec![0, 1, 2, 3, 4, 5], 5);
        test_env.users_list_of_bets_includes_contest(vec![0, 1, 2, 3, 4, 5], &contest_file);

        contest_file = 3;
        test_env.users_list_of_bets_has_length(vec![2, 3], 2);
        test_env.users_list_of_bets_includes_contest(vec![2, 3], &contest_file);
        contest_file = 4;
        test_env.users_list_of_bets_includes_contest(vec![2, 3], &contest_file);
    }
}
