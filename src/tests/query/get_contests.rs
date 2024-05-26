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
    fn get_contests_single() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_file = 1;
        test_env.create_open_contest_success(&contest_file, &1, &100);
        test_env.get_contests_by_ids_success(&vec![contest_file], Some(&1));
    }

    #[test]
    fn get_contests_many() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let contest_files = vec![1, 2, 3, 4, 5]; // Example vector of contest file numbers.

        for file_number in contest_files.iter() {
            test_env.create_open_contest_success(file_number, &1, &100);
        }

        test_env.get_contests_by_ids_success(&contest_files, Some(&5)); // Expecting 5 contests across the files, if each file is supposed to hold one contest.
    }

    #[test]
    fn get_contests_missing_are_ignored() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));
        let mut contest_files = vec![1, 2, 3, 4]; // Example vector of contest file numbers.

        for file_number in contest_files.iter() {
            test_env.create_open_contest_success(file_number, &1, &100);
        }
        contest_files.insert(4, 5);
        test_env.get_contests_by_ids_success(&contest_files, Some(&4)); // Expecting 5 contests across the files, if each file is supposed to hold one contest.
    }
}
