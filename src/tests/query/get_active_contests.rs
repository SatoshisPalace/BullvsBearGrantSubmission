#[cfg(test)]
mod tests {
    use crate::tests::{
        constants::{AFTER_TIME_OF_CLOSE, AFTER_TIME_OF_RESOLVE},
        test_env::tests::TestEnv,
    };

    ////////TESTS////////
    #[test]
    fn get_active_contests_single() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let contest_file = 1;
        test_env.create_open_contest_success(&contest_file, &1, &100);
        test_env.get_active_contests_success(None, None, None, 1);
    }

    #[test]
    fn get_active_contests_many() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let contest_files = vec![1, 2, 3, 4, 5]; // Example vector of contest file numbers.

        for file_number in contest_files.iter() {
            test_env.create_open_contest_success(file_number, &1, &100);
        }

        test_env.get_active_contests_success(None, None, None, contest_files.len());
    }

    #[test]
    fn get_active_contests_ignores_after_time_of_resolve() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let contest_files = vec![1, 2, 3, 4, 5]; // Example vector of contest file numbers.

        for file_number in contest_files.iter() {
            test_env.create_open_contest_success(file_number, &1, &100);
        }

        test_env.set_time(AFTER_TIME_OF_RESOLVE);
        test_env.get_active_contests_success(None, None, None, 0);
    }

    #[test]
    fn get_active_contests_ignores_after_time_of_close() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let contest_files = vec![1, 2, 3, 4, 5]; // Example vector of contest file numbers.

        for file_number in contest_files.iter() {
            test_env.create_open_contest_success(file_number, &1, &100);
        }

        test_env.set_time(AFTER_TIME_OF_CLOSE);
        test_env.get_active_contests_success(None, None, None, 0);
    }

    #[test]
    fn get_active_contests_many_other_user() {
        let mut test_env = TestEnv::new();
        test_env.initialize();

        let contest_files = vec![1, 2, 3, 4, 5]; // Example vector of contest file numbers.

        for file_number in contest_files.iter() {
            test_env.create_open_contest_success(file_number, &1, &100);
        }

        test_env.set_sender("user2".to_owned());

        test_env.get_active_contests_success(None, None, None, contest_files.len());
    }
}
