#[cfg(test)]
pub mod tests {
    use crate::cryptography::cryptography::is_valid_signature;
    use crate::cryptography::error::CryptographyError;
    use cosmwasm_std::testing::mock_dependencies;

    #[test]
    fn invalid_signature() {
        let deps = mock_dependencies();

        let public_key = "04eec6a876668ffb7031f9b9ade7c0c4bc47681ac27fec532bfd5c94fb3dd71d675a363d7036ba8d831a499b12e4f04c8741b90e3c4f3c6b64dd1104132d49498c";
        let message = "Unsigned Message";
        let signature = "f05eeb907cfa5b82742995fd471c73fdd320e7086af5b8bc3e2818675a7f5c19307661956b7d01fea44ddb32dbc8b352a465622bdd6f68171904cd00a4886889";

        let actual = is_valid_signature(&deps.api, public_key, message, signature);
        let expected = Err(CryptographyError::InvalidSignature);
        assert_eq!(expected, actual);
    }
    #[test]
    fn valid_signature() {
        let deps = mock_dependencies();

        let public_key = "04eec6a876668ffb7031f9b9ade7c0c4bc47681ac27fec532bfd5c94fb3dd71d675a363d7036ba8d831a499b12e4f04c8741b90e3c4f3c6b64dd1104132d49498c";
        let message = "Signed Message";
        let signature = "d9c2219d426b182929b6d2f41b9e06256133b99d9dd0ecd9bac8e612cff298a1693d1161a69f3374670ea19818ace6a8da9cad74a272343ed5f3fd4c27eb3849";

        let actual = is_valid_signature(&deps.api, public_key, message, signature);
        let expected = Ok(());
        assert_eq!(expected, actual);
    }
}
