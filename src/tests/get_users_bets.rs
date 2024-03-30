#[cfg(test)]
mod tests {
    use crate::tests::test_env::tests::TestEnv;

    ////////TESTS////////
    #[test]
    fn user_creates_with_one_bet() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let contest_file = 1;
        test_env.create_open_contest_success(&contest_file, &1, &100);
        test_env.users_bets_has_length(1);
        test_env.users_bets_includes_contest(&contest_file);
    }

    #[test]
    fn user_bets_on_contest_without_creation() {
        let mut test_env = TestEnv::new();
        test_env.initialize();
        let contest_file = 1;

        test_env.create_open_contest_success(&contest_file, &1, &100);
        test_env.users_bets_has_length(1);

        test_env.set_sender("user2".to_owned());

        test_env.bet_on_contest_success(&contest_file, &2, &100);
        test_env.users_bets_has_length(1);
        test_env.users_bets_includes_contest(&contest_file);
    }

    #[test]
    fn invalid_bets_are_ignored() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let contest_file = 1;
        test_env.create_open_contest_success(&contest_file, &1, &1);

        test_env.create_invalid_signature_contest_failure(&contest_file, &1, &1);
        test_env.users_bets_has_length(1);
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
        test_env.users_bets_has_length(1);
    }
}
