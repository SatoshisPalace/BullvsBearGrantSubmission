#[cfg(test)]
mod tests {
    use crate::tests::test_env::tests::TestEnv;

    ////////TESTS////////
    #[test]
    fn user_creates_contest() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let contest_file = 1;
        test_env.create_open_contest_success(&contest_file, &1, &100);
    }

    #[test]
    fn user_creates_2_contests() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let mut contest_file = 1;
        test_env.create_open_contest_success(&contest_file, &1, &100);

        contest_file = 2;
        test_env.create_open_contest_success(&contest_file, &1, &100);
    }

    #[test]
    fn different_users_create_contests() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let mut contest_file = 1;
        test_env.create_open_contest_success(&contest_file, &1, &100);

        test_env.set_sender("user2".to_owned());
        contest_file = 2;
        test_env.create_open_contest_success(&contest_file, &1, &100);
    }

    #[test]
    fn differet_users_create_5_contests() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let mut contest_file = 1;
        test_env.create_open_contest_success(&contest_file, &1, &100);

        test_env.set_sender("user2".to_owned());
        contest_file = 2;
        test_env.create_open_contest_success(&contest_file, &1, &100);

        test_env.set_sender("user3".to_owned());
        contest_file = 3;
        test_env.create_open_contest_success(&contest_file, &1, &100);

        test_env.set_sender("user4".to_owned());
        contest_file = 4;
        test_env.create_open_contest_success(&contest_file, &1, &100);

        test_env.set_sender("user5".to_owned());
        contest_file = 5;
        test_env.create_open_contest_success(&contest_file, &1, &100);
    }

    #[test]
    fn cannot_create_contest_closed_awaiting_results() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let mut contest_file = 1;
        test_env.create_closed_waiting_results_contest_failure(&contest_file, &1, &100);

        contest_file = 2;
        test_env.create_open_contest_success(&contest_file, &1, &100);
    }

    #[test]
    fn cannot_create_contest_with_invalid_signature() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let contest_file = 1;
        test_env.create_invalid_signature_contest_failure(&contest_file, &1, &100);
    }

    #[test]
    fn cannot_create_contest_closed_claimable() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let contest_file = 1;
        test_env.create_closed_claimable_contest_failure(&contest_file, &1, &100);
    }

    #[test]
    fn cannot_bet_minimum_minus_1() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        // User1 creates a contest
        let minimum_bet = 100;
        test_env.set_minimum_bet_success(&minimum_bet);

        let contest_file = 1;
        test_env.create_open_contest_failure(&contest_file, &1, &99);
    }
}
