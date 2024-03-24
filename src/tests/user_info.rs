#[cfg(test)]
mod tests {
    use crate::tests::test_env::tests::TestEnv;

    ////////TESTS////////
    #[test]
    fn user_creates_with_one_bet() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let contest_id = 1;
        test_env.create_valid_contest_success(&contest_id, &1, &100);
        test_env.users_bets_has_length(1);
        test_env.users_bets_includes_id(&contest_id);
    }

    #[test]
    fn user_bets_on_contest_without_creation() {
        let mut test_env = TestEnv::new();
        test_env.initialize();
        let contest_id = 1;

        test_env.create_valid_contest_success(&contest_id, &1, &100);
        test_env.users_bets_has_length(1);

        test_env.set_sender("user2".to_owned());

        test_env.bet_on_contest_success(&contest_id, &2, &100);
        test_env.users_bets_has_length(1);
        test_env.users_bets_includes_id(&contest_id);
    }
}
