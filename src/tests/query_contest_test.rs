#[cfg(test)]

pub mod tests {
    use cosmwasm_std::{
        from_binary,
        testing::{mock_dependencies, mock_env, MockApi, MockQuerier, MockStorage},
        Empty, OwnedDeps,
    };

    use crate::{
        contest::responses::query::contest_response::{
            ContestInfoAndSummaryQueryResponse, ContestsQueryResponse,
        },
        contract::query,
        msg::QueryMsg,
        tests::{
            contract_init_test::tests::_initialize_test,
            create_contest_test::tests::{
                _create_contest_test, _create_invalid_contest_test,
                _get_invalid_create_contest_msg, _get_valid_contest_info,
                _get_valid_create_contest_msg,
            },
        },
    };

    ////////TESTS////////
    #[test]
    fn query_contest() {
        let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> =
            mock_dependencies();

        _initialize_test(&mut deps);

        let msg = _get_valid_create_contest_msg();

        _create_contest_test(&mut deps, msg);

        _query_contest_test(&mut deps);
    }

    #[test]
    fn query_contests() {
        let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> =
            mock_dependencies();

        _initialize_test(&mut deps);

        let msg = _get_valid_create_contest_msg();

        _create_contest_test(&mut deps, msg);

        _query_contests_test(&mut deps);
    }

    #[test]
    fn query_invalid_contest() {
        let mut deps: OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, MockQuerier> =
            mock_dependencies();

        _initialize_test(&mut deps);

        let msg = _get_invalid_create_contest_msg();

        _create_invalid_contest_test(&mut deps, msg);

        _query_invalid_contest_test(&mut deps);
    }

    ////////INNER TESTS////////
    pub fn _query_contest_test(deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>) {
        let msg = QueryMsg::GetContest {
            contest_id: _get_valid_contest_info().id,
        };

        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let contest_query_response: ContestInfoAndSummaryQueryResponse = from_binary(&res).unwrap();
        assert_eq!(1, contest_query_response.contest_info.id());
        assert_eq!(
            4102462800u64,
            contest_query_response.contest_info.time_of_close()
        );
        assert_eq!(
            4102462800u64,
            contest_query_response.contest_info.time_of_resolve()
        );
        assert_eq!(2, contest_query_response.contest_info.options().len());
    }

    pub fn _query_contests_test(deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>) {
        let msg = QueryMsg::GetContests {
            contest_ids: vec![_get_valid_contest_info().id], // Using just one contest_id for this test
        };

        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let contests_query_response: ContestsQueryResponse = from_binary(&res).unwrap();

        assert_eq!(1, contests_query_response.contests.len()); // Should have one contest in the response

        let contest_query_response = contests_query_response
            .contests
            .get(0)
            .expect("Expected a contest at index 0");
        assert_eq!(1, contest_query_response.contest_info.id());
        assert_eq!(
            4102462800u64,
            contest_query_response.contest_info.time_of_close()
        );
        assert_eq!(
            4102462800u64,
            contest_query_response.contest_info.time_of_resolve()
        );
        assert_eq!(2, contest_query_response.contest_info.options().len());
    }

    pub fn _query_invalid_contest_test(
        deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
    ) {
        let msg = QueryMsg::GetContest { contest_id: 0 };

        let res = query(deps.as_ref(), mock_env(), msg);
        assert!(res.is_err());
    }
}
