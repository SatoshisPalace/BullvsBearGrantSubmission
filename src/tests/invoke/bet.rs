#[cfg(test)]
mod tests {
    use crate::tests::test_env::tests::TestEnv;

    ////////TESTS////////
    #[test]
    fn user_bets_again() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let contest_file = 1;
        test_env.create_open_contest_success(&contest_file, &1, &100);
        test_env.bet_on_contest_success(&contest_file, &1, &10000000);
    }

    #[test]
    fn user_bets_twice() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let contest_file = 1;
        test_env.create_open_contest_success(&contest_file, &1, &100);
        test_env.bet_on_contest_success(&contest_file, &1, &10000000);
    }

    #[test]
    fn two_users_bet() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let contest_file = 1;
        test_env.create_open_contest_success(&contest_file, &1, &100);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&contest_file, &1, &10000000);
    }

    #[test]
    fn many_users_bet() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let contest_file = 1;
        // Assuming the first user creates the contest
        test_env.create_open_contest_success(&contest_file, &1, &100);

        // Loop to make 20 different users bet on the contest
        for user_number in 1..=20 {
            let user_id = format!("user{}", user_number);
            test_env.set_sender(user_id);
            test_env.bet_on_contest_success(&contest_file, &1, &10000000);
        }
    }

    #[test]
    fn user_bets_on_two_contests() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        // User1 creates two contests
        let contest_file_1 = 1;
        let contest_file_2 = 2;
        test_env.create_open_contest_success(&contest_file_1, &1, &100);
        test_env.create_open_contest_success(&contest_file_2, &2, &200);

        // User2 bets on both contests
        let user2 = "user2".to_owned();
        test_env.set_sender(user2.clone());
        test_env.bet_on_contest_success(&contest_file_1, &1, &10000000);

        // Assuming set_sender needs to be called again if the user interacts more than once
        test_env.set_sender(user2);
        test_env.bet_on_contest_success(&contest_file_2, &2, &20000000);
    }

    #[test]
    fn user_cannot_bet_on_opposite_sides() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        // User1 creates a contest
        let contest_file = 1;
        test_env.create_open_contest_success(&contest_file, &1, &100);

        // User2 bets on the contest successfully the first time
        let user2 = "user2".to_owned();
        test_env.set_sender(user2.clone());
        test_env.bet_on_contest_success(&contest_file, &1, &10000000);

        // User2 attempts to bet on the same contest a second time, which should fail
        test_env.set_sender(user2);
        test_env.bet_on_contest_fail(&contest_file, &2, &10000000);
    }

    #[test]
    fn user_cannot_bet_on_contest_closed_awaiting_results() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        // User1 creates a contest
        let contest_file = 1;
        test_env.create_closed_waiting_results_contest_failure(&contest_file, &1, &100);
    }

    #[test]
    fn cannot_bet_on_contest_after_close() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        // User1 creates a contest
        let contest_file = 1;
        test_env.create_open_contest_success(&contest_file, &1, &100);

        test_env.set_time(4102462801);
        test_env.bet_on_contest_fail(&contest_file, &1, &200)
    }

    #[test]
    fn cannot_bet_on_contest_after_claimable() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        // User1 creates a contest
        let contest_file = 1;
        test_env.create_open_contest_success(&contest_file, &1, &100);

        test_env.set_time(4102549201);
        test_env.bet_on_contest_fail(&contest_file, &1, &200)
    }

    #[test]
    fn bet_minimum() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        // User1 creates a contest
        let minimum_bet = 100;
        test_env.set_minimum_bet_success(&minimum_bet);

        let contest_file = 1;
        test_env.create_open_contest_success(&contest_file, &1, &100);
        test_env.bet_on_contest_success(&contest_file, &1, &100);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&contest_file, &1, &100)
    }

    #[test]
    fn bet_minimum_plus_1() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        // User1 creates a contest
        let minimum_bet = 100;
        test_env.set_minimum_bet_success(&minimum_bet);

        let contest_file = 1;
        test_env.create_open_contest_success(&contest_file, &1, &101);
        test_env.bet_on_contest_success(&contest_file, &1, &101);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&contest_file, &1, &101)
    }

    #[test]
    fn cannot_bet_minimum_minus_1() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let minimum_bet = 100;
        test_env.set_minimum_bet_success(&minimum_bet);

        let contest_file = 1;
        test_env.create_open_contest_success(&contest_file, &1, &100);
        test_env.bet_on_contest_fail(&contest_file, &1, &99);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_fail(&contest_file, &1, &99)
    }
}
