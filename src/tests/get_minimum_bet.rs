#[cfg(test)]
mod tests {
    use crate::tests::test_env::tests::TestEnv;

    ////////TESTS////////

    #[test]
    fn get_minimum_bet_on_initialize() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        test_env.get_minimum_bet_success(None)
    }

    #[test]
    fn get_minimum_bet_after_set() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let minimum_bet = 100;
        test_env.set_minimum_bet_success(&minimum_bet);

        test_env.get_minimum_bet_success(Some(&minimum_bet))
    }

    #[test]
    fn get_minimum_bet_after_contest_creation() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let minimum_bet = 100;
        test_env.set_minimum_bet_success(&minimum_bet);

        let contest_file = 1;
        test_env.create_open_contest_success(&contest_file, &1, &100);

        test_env.get_minimum_bet_success(Some(&minimum_bet))
    }

    #[test]
    fn get_minimum_bet_after_bet() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let minimum_bet = 100;
        test_env.set_minimum_bet_success(&minimum_bet);

        let contest_file = 1;
        test_env.create_open_contest_success(&contest_file, &1, &100);
        test_env.bet_on_contest_success(&contest_file, &1, &100);

        test_env.get_minimum_bet_success(Some(&minimum_bet))
    }

    #[test]
    fn get_minimum_bet_other_user() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        test_env.set_sender("user2".to_owned());
        test_env.get_minimum_bet_success(None);
    }
}
