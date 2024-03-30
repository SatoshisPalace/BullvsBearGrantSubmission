#[cfg(not(feature = "testing"))]
pub mod viewing_keys1 {
    use cosmwasm_std::{Addr, QuerierWrapper, StdResult};
    use sp_secret_toolkit::master_viewing_key::{
        response::IsViewingKeyValidResponse, MasterViewingKey,
    };
    pub fn assert_valid_viewing_key(
        storage: &dyn cosmwasm_std::Storage,
        querier: &QuerierWrapper,
        address: &Addr,
        viewing_key: &String,
    ) -> StdResult<IsViewingKeyValidResponse> {
        let master_viewing_key_contract = MasterViewingKey::singleton_load(storage)?;
        master_viewing_key_contract.assert_viewing_key_is_valid(querier, address, viewing_key)
    }
}
#[allow(unused)]
#[cfg(feature = "testing")]
pub mod viewing_keys {
    use cosmwasm_std::{Addr, QuerierWrapper, StdResult};
    use sp_secret_toolkit::master_viewing_key::response::IsViewingKeyValidResponse;

    pub fn assert_valid_viewing_key(
        storage: &dyn cosmwasm_std::Storage,
        querier: &QuerierWrapper,
        address: &Addr,
        viewing_key: &String,
    ) -> StdResult<IsViewingKeyValidResponse> {
        Ok(IsViewingKeyValidResponse { validity: true })
    }
}
