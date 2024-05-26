#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use cosmwasm_std::Decimal;
    use sp_secret_toolkit::price_feed::{
        data::price_posting::PricePosting,
        response::response_types::prices_by_ids::PricesByIdsResponse,
    };

    use crate::services::integrations::price_feed_service::pricefeed::{
        configure_mock, reset_mock_result, set_oracle_result, MockConfig,
    };
    use crate::tests::{
        constants::{
            AFTER_TIME_OF_1_CLOSE, AFTER_TIME_OF_2_CLOSE, AFTER_TIME_OF_3_CLOSE,
            AFTER_TIME_OF_4_CLOSE, AFTER_TIME_OF_RESOLVE,
        },
        test_env::tests::TestEnv,
    };
    use crate::{
        data::state::FeePercent,
        tests::constants::{BASE_FEE_PERCENT_DENOMINATOR, BASE_FEE_PERCENT_NUMERATOR},
    };

    ////////TESTS////////
    #[test]
    fn cannot_claim_open_contest() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);

        test_env.claim_failure(&contest_file);
    }

    #[test]
    fn cannot_closed_awaiting_results_contest() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);

        test_env.set_time(AFTER_TIME_OF_1_CLOSE);

        test_env.claim_failure(&contest_file);
    }

    #[test]
    fn claim_contest_no_opposition() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        let amount_bet = 100;
        test_env.first_bet_on_contest_success(&contest_file, &1, &amount_bet);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        test_env.claim_success(&contest_file, Some(&amount_bet));
    }

    #[test]
    fn claim_after_expiration_window_no_opposition() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        let amount_bet = 100;
        test_env.first_bet_on_contest_success(&contest_file, &1, &amount_bet);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);
        configure_mock(MockConfig::ReturnError(true));

        test_env.claim_success(&contest_file, Some(&amount_bet));
    }

    #[test]
    fn claim_contest_bets_on_both_sides() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &1, &100);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&contest_file, &2, &100);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);
        test_env.claim_failure(&contest_file);

        test_env.set_sender("creator".to_owned());
        test_env.claim_success(&contest_file, Some(&198));
    }

    #[test]
    fn claim_contest_bets_on_both_sides_side_2() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &2, &100);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&contest_file, &1, &100);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        test_env.set_sender("creator".to_owned());

        let oracle_result = PricesByIdsResponse {
            prices: vec![
                PricePosting::new(Decimal::from_str("58205.43").unwrap(), 1571797500),
                PricePosting::new(Decimal::from_str("58205.29").unwrap(), 1571797800),
            ],
        };
        set_oracle_result(Some(oracle_result));

        test_env.claim_success(&contest_file, None);
        reset_mock_result();
    }

    #[test]
    fn claim_contest_bets_on_both_sides_side_1_price_returned() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &2, &100);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&contest_file, &1, &100);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        test_env.set_sender("creator".to_owned());

        let oracle_result = PricesByIdsResponse {
            prices: vec![PricePosting::new(
                Decimal::from_str("58205.43").unwrap(),
                1571797500,
            )],
        };
        set_oracle_result(Some(oracle_result));

        test_env.claim_failure(&contest_file);
        reset_mock_result();
    }

    #[test]
    fn claim_contest_bets_on_both_sides_same_price() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        test_env.first_bet_on_contest_success(&contest_file, &2, &100);

        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&contest_file, &1, &100);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        test_env.set_sender("creator".to_owned());

        let oracle_result = PricesByIdsResponse {
            prices: vec![
                PricePosting::new(Decimal::from_str("58205.29").unwrap(), 1571797500),
                PricePosting::new(Decimal::from_str("58205.29").unwrap(), 1571797800),
            ],
        };
        set_oracle_result(Some(oracle_result));

        test_env.claim_success(&contest_file, Some(&100));
        reset_mock_result();
    }

    #[test]
    fn claim_multiple_contests() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        test_env.first_bet_on_contest_success(&1, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&1, &2, &100);
        test_env.claim_failure(&1);

        test_env.set_time(AFTER_TIME_OF_1_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&2, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&2, &2, &100);
        test_env.claim_failure(&2);

        test_env.set_time(AFTER_TIME_OF_2_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&3, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&3, &2, &100);
        test_env.claim_failure(&3);

        test_env.set_time(AFTER_TIME_OF_3_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&4, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&4, &2, &100);
        test_env.claim_failure(&4);

        test_env.set_time(AFTER_TIME_OF_4_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&5, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&5, &2, &100);
        test_env.claim_failure(&5);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        test_env.set_sender("creator".to_owned());
        test_env.claim_success(&5, Some(&198));
        test_env.claim_success(&4, Some(&198));
        test_env.claim_success(&3, Some(&198));
        test_env.claim_success(&2, Some(&198));
        test_env.claim_success(&1, Some(&198));
    }

    #[test]
    fn claim_multiple_contests_2_fee() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(2, BASE_FEE_PERCENT_DENOMINATOR));

        test_env.first_bet_on_contest_success(&1, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&1, &2, &100);
        test_env.claim_failure(&1);

        test_env.set_time(AFTER_TIME_OF_1_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&2, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&2, &2, &100);
        test_env.claim_failure(&2);

        test_env.set_time(AFTER_TIME_OF_2_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&3, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&3, &2, &100);
        test_env.claim_failure(&3);

        test_env.set_time(AFTER_TIME_OF_3_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&4, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&4, &2, &100);
        test_env.claim_failure(&4);

        test_env.set_time(AFTER_TIME_OF_4_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&5, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&5, &2, &100);
        test_env.claim_failure(&5);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        test_env.claim_failure(&5);
        test_env.claim_failure(&4);
        test_env.claim_failure(&3);
        test_env.claim_failure(&2);
        test_env.claim_failure(&1);

        test_env.set_sender("creator".to_owned());
        test_env.claim_success(&5, Some(&196));
        test_env.claim_success(&4, Some(&196));
        test_env.claim_success(&3, Some(&196));
        test_env.claim_success(&2, Some(&196));
        test_env.claim_success(&1, Some(&196));
    }

    #[test]
    fn claim_multiple_contests_01_fee() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(1, BASE_FEE_PERCENT_DENOMINATOR * 10));

        test_env.first_bet_on_contest_success(&1, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&1, &2, &100);
        test_env.claim_failure(&1);

        test_env.set_time(AFTER_TIME_OF_1_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&2, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&2, &2, &100);
        test_env.claim_failure(&2);

        test_env.set_time(AFTER_TIME_OF_2_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&3, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&3, &2, &100);
        test_env.claim_failure(&3);

        test_env.set_time(AFTER_TIME_OF_3_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&4, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&4, &2, &100);
        test_env.claim_failure(&4);

        test_env.set_time(AFTER_TIME_OF_4_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&5, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&5, &2, &100);
        test_env.claim_failure(&5);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        test_env.claim_failure(&5);
        test_env.claim_failure(&4);
        test_env.claim_failure(&3);
        test_env.claim_failure(&2);
        test_env.claim_failure(&1);

        test_env.set_sender("creator".to_owned());
        test_env.claim_success(&5, Some(&199));
        test_env.claim_success(&4, Some(&199));
        test_env.claim_success(&3, Some(&199));
        test_env.claim_success(&2, Some(&199));
        test_env.claim_success(&1, Some(&199));
    }

    #[test]
    fn claim_multiple_contests_25_fee() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(25, BASE_FEE_PERCENT_DENOMINATOR * 10));

        test_env.first_bet_on_contest_success(&1, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&1, &2, &100);
        test_env.claim_failure(&1);

        test_env.set_time(AFTER_TIME_OF_1_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&2, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&2, &2, &100);
        test_env.claim_failure(&2);

        test_env.set_time(AFTER_TIME_OF_2_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&3, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&3, &2, &100);
        test_env.claim_failure(&3);

        test_env.set_time(AFTER_TIME_OF_3_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&4, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&4, &2, &100);
        test_env.claim_failure(&4);

        test_env.set_time(AFTER_TIME_OF_4_CLOSE);
        test_env.set_sender("creator".to_owned());
        test_env.first_bet_on_contest_success(&5, &1, &100);
        test_env.set_sender("user2".to_owned());
        test_env.bet_on_contest_success(&5, &2, &100);
        test_env.claim_failure(&5);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        test_env.claim_failure(&5);
        test_env.claim_failure(&4);
        test_env.claim_failure(&3);
        test_env.claim_failure(&2);
        test_env.claim_failure(&1);

        test_env.set_sender("creator".to_owned());
        test_env.claim_success(&5, Some(&195));
        test_env.claim_success(&4, Some(&195));
        test_env.claim_success(&3, Some(&195));
        test_env.claim_success(&2, Some(&195));
        test_env.claim_success(&1, Some(&195));
    }

    #[test]
    fn cannot_claim_twice() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        test_env.first_bet_on_contest_success(&1, &1, &100);

        test_env.set_time(AFTER_TIME_OF_RESOLVE);

        test_env.claim_success(&1, Some(&100));

        test_env.claim_failure(&1);
    }

    #[test]
    fn cannot_claim_contest_user_did_not_bet() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        test_env.first_bet_on_contest_success(&1, &1, &100);

        test_env.set_time(AFTER_TIME_OF_2_CLOSE);

        test_env.set_sender("user2".to_owned());

        test_env.claim_failure(&1);
    }
}
