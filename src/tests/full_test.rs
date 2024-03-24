#[cfg(test)]
pub mod tests {
    use cosmwasm_std::{
        testing::{mock_dependencies, MockApi, MockQuerier},
        OwnedDeps, Uint128,
    };

    use crate::{
        integrations::oracle::oracle::reset_query_contest_result_call_count,
        msg::InvokeMsg,
        tests::{
            bet_contest_test::tests::_bet_contest_test_with_sender_outcome,
            claim_rewards_test::tests::{
                claim_after_nullification_test, losing_claim_test, winning_claim_test,
            },
            contract_init_test::tests::_initialize_test,
            create_contest_test::tests::{
                _create_contest_with_sender_test, _get_valid_create_contest_msg_with_params,
            },
        },
    };

    ////////TESTS////////
    #[test]
    fn claim_rewards_successful_contest_multi_user_full0() {
        let mut deps = mock_dependencies();
        claim_rewards_successful_contest_multi_user(
            &mut deps,
            Uint128::from(10u128),
            Uint128::from(20u128),
            Uint128::from(32u128),
            Uint128::from(72u128),
        );
    }

    #[test]
    fn claim_rewards_successful_contest_multi_user_full1() {
        let mut deps = mock_dependencies();
        claim_rewards_successful_contest_multi_user(
            &mut deps,
            Uint128::from(100u128),
            Uint128::from(100u128),
            Uint128::from(100u128),
            Uint128::from(100u128),
        );
    }

    #[test]
    fn claim_rewards_successful_contest_multi_user_full2() {
        let mut deps = mock_dependencies();
        claim_rewards_successful_contest_multi_user(
            &mut deps,
            Uint128::from(200u128),
            Uint128::from(300u128),
            Uint128::from(300u128),
            Uint128::from(200u128),
        );
    }

    #[test]
    fn claim_rewards_successful_contest_multi_user_full3() {
        let mut deps = mock_dependencies();
        claim_rewards_successful_contest_multi_user(
            &mut deps,
            Uint128::from(1u128),
            Uint128::from(100u128),
            Uint128::from(100u128),
            Uint128::from(100u128),
        );
    }

    #[test]
    fn claim_rewards_nullified_contest_multi_user_full() {
        let contest_creator_bet_on_1: &str = "Contest Creator";
        let contest_creator_bet_amount = Uint128::from(100u128);
        let contest_better1_bet_on_1: &str = "Contest Better1 bet on 1";
        let contest_better1_bet_amount = Uint128::from(100u128);
        let contest_better2_bet_on_2: &str = "Contest Better3 bet on 2";
        let contest_better2_bet_amount = Uint128::from(100u128);
        let contest_better3_bet_on_2: &str = "Contest Better2 bet on 2";
        let contest_better3_bet_amount = Uint128::from(100u128);

        let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> =
            mock_dependencies();

        // Initialize and create a contest
        _initialize_test(&mut deps);

        let create_contest_msg = _get_valid_create_contest_msg_with_params(
            contest_creator_bet_on_1,
            contest_creator_bet_amount,
        );
        let binding = create_contest_msg.clone();
        let (contest_info, _, _) = match &binding {
            InvokeMsg::CreateContest {
                contest_info,
                outcome_id,
                amount,
                ..
            } => (contest_info, outcome_id, amount.unwrap()),
            _ => panic!("Expected CreateContest message"),
        };

        let contest_id = contest_info.id; // Extract the contest_id
        _create_contest_with_sender_test(&mut deps, create_contest_msg, contest_creator_bet_on_1);

        // Bet on the contest
        _bet_contest_test_with_sender_outcome(
            &mut deps,
            contest_better1_bet_on_1,
            contest_info.id,
            1,
            contest_better1_bet_amount,
        );
        _bet_contest_test_with_sender_outcome(
            &mut deps,
            contest_better2_bet_on_2,
            contest_info.id,
            2,
            contest_better2_bet_amount,
        );
        _bet_contest_test_with_sender_outcome(
            &mut deps,
            contest_better3_bet_on_2,
            contest_info.id,
            2,
            contest_better3_bet_amount,
        );
        reset_query_contest_result_call_count(); // Reset call count before the test

        // Try to claim rewards before time_of_resolve
        claim_after_nullification_test(
            &mut deps,
            contest_id,
            contest_creator_bet_on_1,
            contest_creator_bet_amount.into(),
        );

        claim_after_nullification_test(
            &mut deps,
            contest_id,
            contest_better1_bet_on_1,
            contest_better1_bet_amount.into(),
        );

        claim_after_nullification_test(
            &mut deps,
            contest_id,
            contest_better2_bet_on_2,
            contest_better2_bet_amount.into(),
        );
        claim_after_nullification_test(
            &mut deps,
            contest_id,
            contest_better3_bet_on_2,
            contest_better3_bet_amount.into(),
        );
    }

    ////////// Inner Tests ////////////
    pub fn claim_rewards_successful_contest_multi_user(
        deps: &mut OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier>,
        contest_creator_bet_amount: Uint128,
        contest_better1_bet_amount: Uint128,
        contest_better2_bet_amount: Uint128,
        contest_better3_bet_amount: Uint128,
    ) {
        let contest_creator_bet_on_1: &str = "Contest Creator";
        let contest_better1_bet_on_1: &str = "Contest Better1 bet on 1";
        let contest_better2_bet_on_2: &str = "Contest Better3 bet on 2";
        let contest_better3_bet_on_2: &str = "Contest Better2 bet on 2";

        // Initialize and create a contest
        _initialize_test(deps);

        let create_contest_msg = _get_valid_create_contest_msg_with_params(
            contest_creator_bet_on_1,
            contest_creator_bet_amount,
        );
        let binding = create_contest_msg.clone();
        let (contest_info, _, _) = match &binding {
            InvokeMsg::CreateContest {
                contest_info,
                outcome_id,
                amount,
                ..
            } => (contest_info, outcome_id, amount.unwrap()),
            _ => panic!("Expected CreateContest message"),
        };

        let contest_id = contest_info.id; // Extract the contest_id
        _create_contest_with_sender_test(deps, create_contest_msg, contest_creator_bet_on_1);

        // Bet on the contest
        _bet_contest_test_with_sender_outcome(
            deps,
            contest_better1_bet_on_1,
            contest_info.id,
            1,
            contest_better1_bet_amount,
        );
        _bet_contest_test_with_sender_outcome(
            deps,
            contest_better2_bet_on_2,
            contest_info.id,
            2,
            contest_better2_bet_amount,
        );
        _bet_contest_test_with_sender_outcome(
            deps,
            contest_better3_bet_on_2,
            contest_info.id,
            2,
            contest_better3_bet_amount,
        );

        // Claim rewards for each participant
        // Side 1 bettors (winning)
        winning_claim_test(
            deps,
            contest_id,
            contest_creator_bet_on_1,
            contest_creator_bet_amount.u128(),
            1,
        );
        winning_claim_test(
            deps,
            contest_id,
            contest_better1_bet_on_1,
            contest_better1_bet_amount.u128(),
            1,
        );

        // Side 2 bettors (losing)
        losing_claim_test(
            deps,
            contest_id,
            contest_better2_bet_on_2,
            contest_better2_bet_amount.u128(),
            1,
        );
        losing_claim_test(
            deps,
            contest_id,
            contest_better3_bet_on_2,
            contest_better3_bet_amount.u128(),
            1,
        );
    }
}
