#[cfg(test)]
mod tests {
    use crate::tests::test_env::tests::TestEnv;

    ////////TESTS////////

    #[test]
    fn get_user_bet_after_contest_creation() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let contest_file = 1;
        let amount_to_bet = 100;
        let outcome_to_bet_on = 1;
        test_env.create_open_contest_success(&contest_file, &outcome_to_bet_on, &amount_to_bet);
        test_env.get_user_bet_success(
            &contest_file,
            Some(&amount_to_bet),
            Some(&outcome_to_bet_on),
            Some(&false),
        )
    }

    #[test]
    fn get_user_bet_after_bet() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let contest_file = 1;
        let amount_to_bet = 100;
        let outcome_to_bet_on = 1;
        test_env.create_open_contest_success(&contest_file, &outcome_to_bet_on, &amount_to_bet);
        test_env.bet_on_contest_success(&contest_file, &outcome_to_bet_on, &amount_to_bet);

        test_env.get_user_bet_success(
            &contest_file,
            Some(&(amount_to_bet + amount_to_bet)),
            Some(&outcome_to_bet_on),
            Some(&false),
        )
    }

    #[test]
    fn get_user_bet_after_create_and_bet_many() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let amount_to_bet = 100;
        let outcome_to_bet_on = 1;

        let contest_files = vec![1, 2, 3, 4, 5]; // Example vector of contest file numbers.

        for file_number in contest_files.iter() {
            test_env.create_open_contest_success(file_number, &outcome_to_bet_on, &amount_to_bet);
            test_env.bet_on_contest_success(&file_number, &outcome_to_bet_on, &amount_to_bet);
        }

        for file_number in contest_files.iter() {
            test_env.get_user_bet_success(
                &file_number,
                Some(&(amount_to_bet + amount_to_bet)),
                Some(&outcome_to_bet_on),
                Some(&false),
            )
        }
    }
}
