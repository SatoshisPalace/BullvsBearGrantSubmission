#[cfg(test)]
mod tests {
    use crate::tests::test_env::tests::TestEnv;

    ////////TESTS////////

    #[test]
    fn get_snip20_on_initialize() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        test_env.get_snip20_success()
    }

    #[test]
    fn get_snip20_after_contest_creation() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);
        test_env.get_snip20_success()
    }

    #[test]
    fn get_snip20_after_bet() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);
        test_env.bet_on_contest_success(&contest_file, &1, &100);
        test_env.get_snip20_success()
    }

    #[test]
    fn get_snip20_other_user() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        test_env.set_sender("user2".to_owned());
        test_env.get_snip20_success()
    }
}
