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
    fn get_fee_percent_on_initialize() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));

        test_env.get_fee_percent(Some(&FeePercent::new(1, 100)))
    }

    #[test]
    fn get_fee_percent_on_initialize_other_values_random() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR + 5,
            BASE_FEE_PERCENT_DENOMINATOR - 7,
        ));

        test_env.get_fee_percent(Some(&FeePercent::new(6, 93)))
    }

    #[test]
    fn get_fee_percent_on_initialize_other_values_2() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR + 1,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));

        test_env.get_fee_percent(Some(&FeePercent::new(2, 100)))
    }

    #[test]
    fn get_fee_percent_on_initialize_other_values_01() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR * 10,
        ));

        test_env.get_fee_percent(Some(&FeePercent::new(1, 1000)))
    }

    #[test]
    fn get_fee_percent_on_initialize_other_values_25() {
        let mut test_env = TestEnv::new();
        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR + 24,
            BASE_FEE_PERCENT_DENOMINATOR * 10,
        ));

        test_env.get_fee_percent(Some(&FeePercent::new(25, 1000)))
    }
}
