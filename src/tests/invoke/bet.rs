#[cfg(test)]
mod tests {
    use crate::{
        data::state::FeePercent,
        tests::{
            constants::{BASE_FEE_PERCENT_DENOMINATOR, BASE_FEE_PERCENT_NUMERATOR},
            test_env::tests::TestEnv,
        },
    };

    ////////TESTS////////
    #[test]
    fn user_bets_again() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);
        test_env.bet_on_contest_success(&contest_file, &1, &10000000);
    }

    #[test]
    fn user_bets_on_invalid_ticker() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 11;
        test_env.first_bet_on_contest_fail(&contest_file, &1, &100);
    }

    #[test]
    fn user_bets_on_invalid_outcome() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        test_env.first_bet_on_contest_fail(&contest_file, &3, &100);
    }

    #[test]
    fn user_bets_twice() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);
        test_env.bet_on_contest_success(&contest_file, &1, &10000000);
    }

    #[test]
    fn two_users_bet() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&contest_file, &1, &10000000);
    }

    #[test]
    fn many_users_bet() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        // Assuming the first user creates the contest
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);

        // Loop to make 20 different users bet on the contest
        for user_number in 1..=20 {
            let user_id = format!("user{}", user_number);
            test_env.set_sender(user_id);
            test_env.bet_on_contest_success(&contest_file, &1, &10000000);
        }
    }

    #[test]
    fn user_cannot_bet_on_opposite_sides() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        // User1 creates a contest
        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);

        // User2 bets on the contest successfully the first time
        let user2 = "user2".to_owned();
        test_env.set_sender(user2.clone());
        test_env.bet_on_contest_success(&contest_file, &1, &10000000);

        // User2 attempts to bet on the same contest a second time, which should fail
        test_env.set_sender(user2);
        test_env.bet_on_contest_fail(&contest_file, &2, &10000000);
    }

    #[test]
    fn bet_minimum() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        // User1 creates a contest
        let minimum_bet = 100;
        test_env.set_minimum_bet_success(&minimum_bet);

        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);
        test_env.bet_on_contest_success(&contest_file, &1, &100);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&contest_file, &1, &100)
    }

    #[test]
    fn bet_minimum_plus_1() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        // User1 creates a contest
        let minimum_bet = 100;
        test_env.set_minimum_bet_success(&minimum_bet);

        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &101);
        test_env.bet_on_contest_success(&contest_file, &1, &101);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&contest_file, &1, &101)
    }

    #[test]
    fn cannot_bet_minimum_minus_1() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let minimum_bet = 100;
        test_env.set_minimum_bet_success(&minimum_bet);

        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);
        test_env.bet_on_contest_fail(&contest_file, &1, &99);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_fail(&contest_file, &1, &99)
    }
}
