#[cfg(test)]
mod tests {
    use crate::{
        msgs::query::commands::get_users_bets::UsersBetsQueryFilters,
        tests::{
            constants::{AFTER_TIME_OF_CLOSE, AFTER_TIME_OF_RESOLVE},
            test_env::tests::TestEnv,
        },
    };

    ////////TESTS////////
    #[test]
    fn user_creates_with_one_bet() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let contest_file = 1;
        test_env.create_open_contest_success(&contest_file, &1, &100);
        test_env.users_bets_has_length(None, 1);
        test_env.users_bets_includes_contest(&contest_file, None);
    }

    #[test]
    fn user_bets_on_contest_without_creation() {
        let mut test_env = TestEnv::new();
        test_env.initialize();
        let contest_file = 1;

        test_env.create_open_contest_success(&contest_file, &1, &100);
        test_env.users_bets_has_length(None, 1);

        test_env.set_sender("user2".to_owned());

        test_env.bet_on_contest_success(&contest_file, &2, &100);
        test_env.users_bets_has_length(None, 1);
        test_env.users_bets_includes_contest(&contest_file, None);
    }

    #[test]
    fn invalid_bets_are_ignored() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let contest_file = 1;
        test_env.create_open_contest_success(&contest_file, &1, &1);

        test_env.create_invalid_signature_contest_failure(&contest_file, &1, &1);
        test_env.users_bets_has_length(None, 1);
    }

    #[test]
    fn bets_below_minimum_are_ignored() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let minimum_bet = 100;
        test_env.set_minimum_bet_success(&minimum_bet);

        let mut contest_file = 1;
        test_env.create_open_contest_success(&contest_file, &1, &100);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_fail(&contest_file, &1, &99);

        test_env.set_sender("creator".to_owned());
        contest_file = 2;
        test_env.create_open_contest_success(&contest_file, &1, &100);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&contest_file, &1, &100);
        test_env.users_bets_has_length(None, 1);
    }

    #[test]
    fn filter_claimable_does_not_include_open_contests() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let contest_file = 1;
        test_env.create_open_contest_success(&contest_file, &1, &100);

        let filters = vec![UsersBetsQueryFilters::Claimable];
        test_env.users_bets_has_length(Some(filters), 0);
    }

    #[test]
    fn filter_claimable_does_not_include_closed_awaiting_results_contests() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let contest_file = 1;
        test_env.create_open_contest_success(&contest_file, &1, &100);

        test_env.set_time(AFTER_TIME_OF_CLOSE);
        let filters = vec![UsersBetsQueryFilters::Claimable];
        test_env.users_bets_has_length(Some(filters), 0);
    }

    #[test]
    fn filter_claimable_includes_claimable_contests() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let contest_file = 1;
        test_env.create_open_contest_success(&contest_file, &1, &100);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);
        let filters = vec![UsersBetsQueryFilters::Claimable];
        test_env.users_bets_has_length(Some(filters), 1);
    }

    #[test]
    fn filter_claimable_does_not_include_losses() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let contest_file = 1;
        test_env.create_open_contest_success(&contest_file, &1, &100);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&contest_file, &2, &100);
        test_env.set_time(AFTER_TIME_OF_RESOLVE);
        let filters = vec![UsersBetsQueryFilters::Claimable];
        test_env.users_bets_has_length(Some(filters), 0);
    }

    #[test]
    fn filter_claimable_includes_many() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let mut contest_file = 1;
        test_env.create_open_contest_success(&contest_file, &1, &100);

        contest_file = 2;
        test_env.create_open_contest_success(&contest_file, &1, &100);

        contest_file = 3;
        test_env.create_open_contest_success(&contest_file, &1, &100);

        contest_file = 4;
        test_env.create_open_contest_success(&contest_file, &1, &100);

        contest_file = 5;
        test_env.create_open_contest_success(&contest_file, &1, &100);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);
        let filters = vec![UsersBetsQueryFilters::Claimable];
        test_env.users_bets_has_length(Some(filters), 5);
    }
}
