use cosmwasm_std::{DepsMut, Addr, Storage};

use super::{error::Snip20Error, snip_20_state::{config, Snip20State}, snip_20_contract::Snip20Contract};

pub fn initialize_snip_20_state(
    storage: &mut dyn Storage,
){
    let snip_20_state = Snip20State {
        known_snip_20: vec![],
    };
    config(storage).save(&snip_20_state).unwrap();
}

pub fn add_snip_20(
    deps: DepsMut,
    contract_address: Addr,
    code_hash: String
) -> Result<(), Snip20Error>{
    let mut conf = config(deps.storage);
    let mut state = conf.load()?;

    let snip20 = Snip20Contract{
        address: contract_address,
        code_hash,
    };

    if !state.known_snip_20.contains(&snip20) {
        state.known_snip_20.push(snip20.clone());
    }
    conf.save(&state)?;
    Ok(())
}