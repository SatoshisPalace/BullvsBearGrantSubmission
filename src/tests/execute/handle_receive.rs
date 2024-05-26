#[cfg(test)]
mod tests {
    use cosmwasm_std::Addr;

    use crate::{
        data::state::FeePercent,
        tests::{
            constants::{BASE_FEE_PERCENT_DENOMINATOR, BASE_FEE_PERCENT_NUMERATOR},
            test_env::tests::TestEnv,
        },
    };

    #[test]
    fn handle_receive_valid_bet() {
        let mut test_env = TestEnv::new();

        test_env.initialize(FeePercent::new(
            BASE_FEE_PERCENT_NUMERATOR,
            BASE_FEE_PERCENT_DENOMINATOR,
        ));

        test_env.handle_receive_success(&1, &1, &100, Addr::unchecked("Snip20 Address"));
    }
}
