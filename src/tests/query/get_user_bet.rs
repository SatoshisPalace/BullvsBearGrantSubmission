#[cfg(test)]
mod tests {
    use crate::tests::{
        constants::{
            AFTER_TIME_OF_1_CLOSE, AFTER_TIME_OF_2_CLOSE, AFTER_TIME_OF_3_CLOSE,
            AFTER_TIME_OF_4_CLOSE, AFTER_TIME_OF_5_CLOSE,
        },
        test_env::tests::TestEnv,
    };
    use crate::{
        data::state::FeePercent,
        tests::constants::{BASE_FEE_PERCENT_DENOMINATOR, BASE_FEE_PERCENT_NUMERATOR},
    };

    ////////TESTS////////

    #[test]
    fn get_user_bet_after_contest_creation() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        let amount_to_bet = 100;
        let outcome_to_bet_on = 1;
        test_env.first_bet_on_contest_success(&contest_file, &outcome_to_bet_on, &amount_to_bet);
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
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        let amount_to_bet = 100;
        let outcome_to_bet_on = 1;
        test_env.first_bet_on_contest_success(&contest_file, &outcome_to_bet_on, &amount_to_bet);
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
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let amount_to_bet = 100;
        let outcome_to_bet_on = 1;

        let contest_files = vec![1, 2, 3, 4, 5]; // Example vector of contest file numbers.

        test_env.first_bet_on_contest_success(
            &contest_files[0],
            &outcome_to_bet_on,
            &amount_to_bet,
        );
        test_env.bet_on_contest_success(&contest_files[0], &outcome_to_bet_on, &amount_to_bet);
        test_env.set_time(AFTER_TIME_OF_1_CLOSE);

        test_env.first_bet_on_contest_success(
            &contest_files[1],
            &outcome_to_bet_on,
            &amount_to_bet,
        );
        test_env.bet_on_contest_success(&contest_files[1], &outcome_to_bet_on, &amount_to_bet);
        test_env.set_time(AFTER_TIME_OF_2_CLOSE);

        test_env.first_bet_on_contest_success(
            &contest_files[2],
            &outcome_to_bet_on,
            &amount_to_bet,
        );
        test_env.bet_on_contest_success(&contest_files[2], &outcome_to_bet_on, &amount_to_bet);
        test_env.set_time(AFTER_TIME_OF_3_CLOSE);

        test_env.first_bet_on_contest_success(
            &contest_files[3],
            &outcome_to_bet_on,
            &amount_to_bet,
        );
        test_env.bet_on_contest_success(&contest_files[3], &outcome_to_bet_on, &amount_to_bet);
        test_env.set_time(AFTER_TIME_OF_4_CLOSE);

        test_env.first_bet_on_contest_success(
            &contest_files[4],
            &outcome_to_bet_on,
            &amount_to_bet,
        );
        test_env.bet_on_contest_success(&contest_files[4], &outcome_to_bet_on, &amount_to_bet);
        test_env.set_time(AFTER_TIME_OF_5_CLOSE);

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
