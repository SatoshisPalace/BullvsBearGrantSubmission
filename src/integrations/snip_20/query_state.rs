use cosmwasm_std::{to_binary, Binary, StdResult, Storage};

use super::{error::Snip20Error, snip_20_contract::Snip20Contract, snip_20_state::config_read};

pub fn get_snip_20_contract(
    storage: &dyn Storage,
    index: usize,
) -> Result<Snip20Contract, Snip20Error> {
    let state = config_read(storage).load()?;
    match state.known_snip_20.get(index) {
        Some(snip20_contract) => Ok(snip20_contract.clone()),
        None => Err(Snip20Error::InvalidIndex(index)),
    }
}

pub fn get_registered_snip_20s(storage: &dyn Storage) -> StdResult<Binary> {
    let state = config_read(storage).load()?;
    to_binary(&state.known_snip_20)
}

pub fn check_known_snip_20(
    storage: &dyn Storage,
    contract_addr: &String,
) -> Result<(), Snip20Error> {
    let state = config_read(storage).load()?;
    if !state
        .known_snip_20
        .iter()
        .any(|snip20| snip20.address == *contract_addr)
    {
        return Err(Snip20Error::UnknownSnip20(contract_addr.to_string()));
    }
    Ok(())
}
