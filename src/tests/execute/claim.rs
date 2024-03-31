#[cfg(test)]
mod tests {
    use crate::tests::{constants::AFTER_TIME_OF_RESOLVE, test_env::tests::TestEnv};

    ////////TESTS////////
    #[test]
    fn cannot_claim_open_contest() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let contest_file = 1;
        test_env.create_open_contest_success(&contest_file, &1, &100);

        test_env.claim_failure(&contest_file);
    }

    #[test]
    fn cannot_closed_awaiting_results_contest() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let contest_file = 1;
        test_env.create_open_contest_success(&contest_file, &1, &100);

        test_env.set_time(4102462800);

        test_env.claim_failure(&contest_file);
    }

    #[test]
    fn claim_contest_no_opposition() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let contest_file = 1;
        let amount_bet = 100;
        test_env.create_open_contest_success(&contest_file, &1, &amount_bet);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        test_env.claim_success(&contest_file, Some(&amount_bet));
    }

    #[test]
    fn claim_contest_bets_on_both_sides() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let contest_file = 1;
        test_env.create_open_contest_success(&contest_file, &1, &100);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&contest_file, &2, &100);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);
        test_env.claim_failure(&contest_file);

        test_env.set_sender("creator".to_owned());
        test_env.claim_success(&contest_file, None);
    }

    #[test]
    fn claim_multiple_contests() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        test_env.create_open_contest_success(&1, &1, &100);
        test_env.create_open_contest_success(&2, &1, &100);
        test_env.create_open_contest_success(&3, &1, &100);
        test_env.create_open_contest_success(&4, &1, &100);
        test_env.create_open_contest_success(&5, &1, &100);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&1, &2, &100);
        test_env.bet_on_contest_success(&2, &2, &100);
        test_env.bet_on_contest_success(&3, &2, &100);
        test_env.bet_on_contest_success(&4, &2, &100);
        test_env.bet_on_contest_success(&5, &2, &100);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        test_env.claim_failure(&5);
        test_env.claim_failure(&4);
        test_env.claim_failure(&3);
        test_env.claim_failure(&2);
        test_env.claim_failure(&1);

        test_env.set_sender("creator".to_owned());
        test_env.claim_success(&5, None);
        test_env.claim_success(&4, None);
        test_env.claim_success(&3, None);
        test_env.claim_success(&2, None);
        test_env.claim_success(&1, None);
    }

    #[test]
    fn cannot_claim_twice() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        test_env.create_open_contest_success(&1, &1, &100);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        test_env.claim_success(&1, None);

        test_env.claim_failure(&1);
    }
}
