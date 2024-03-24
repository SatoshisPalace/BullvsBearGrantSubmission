// contest_info_service.rs
use cosmwasm_std::{DepsMut, Env, Storage};
use sp_secret_toolkit::cryptography::signing::is_valid_signature;

use crate::{
    contest::{
        data::contest_info::ContestInfo, error::real_contest_info_error::RealContestInfoError,
    },
    state::State,
};

pub fn create_new_contest(
    deps: &mut DepsMut,
    env: &Env,
    contest_info: &ContestInfo,
    contest_info_signature_hex: &String,
) -> Result<(), RealContestInfoError> {
    contest_info.assert_time_of_close_not_passed(env.block.time.seconds())?;
    contest_info.validate_contest()?;

    validate_contest_info_signature(deps, contest_info, contest_info_signature_hex)?;

    // Contests cannot be recreated
    let contest_id = contest_info.id();
    if ContestInfo::keymap_verify_exists(deps.storage, &contest_id).is_ok() {
        return Err(RealContestInfoError::ContestAlreadyExist(contest_id).into());
    }

    contest_info.keymap_save(deps.storage)?;
    Ok(())
}

fn validate_contest_info_signature(
    deps: &mut DepsMut,
    contest_info: &ContestInfo,
    contest_info_signature_hex: &String,
) -> Result<(), RealContestInfoError> {
    let state = State::singleton_load(deps.storage)?;

    let contest_info_json: String = contest_info.to_json();
    is_valid_signature(
        deps.api,
        state.get_satoshis_palace_signing_address().as_str(),
        &contest_info_json,
        &contest_info_signature_hex,
    )?;
    Ok(())
}

pub fn get_contest_info(
    storage: &dyn Storage,
    contest_id: &u32,
) -> Result<ContestInfo, RealContestInfoError> {
    match ContestInfo::keymap_get_by_id(storage, contest_id) {
        Some(contest_info) => Ok(contest_info),
        None => Err(RealContestInfoError::ContestNotFound(*contest_id)),
    }
}

pub fn get_contest_infos_for_ids(
    storage: &dyn Storage,
    contest_ids: &Vec<u32>,
) -> Result<Vec<ContestInfo>, RealContestInfoError> {
    let mut contest_infos: Vec<ContestInfo> = Vec::new();

    for contest_id in contest_ids {
        let contest_info = get_contest_info(storage, contest_id)?;
        contest_infos.push(contest_info);
    }

    Ok(contest_infos)
}

pub fn get_contest_infos_for_ids_ignore_missing(
    storage: &dyn Storage,
    contest_ids: &Vec<u32>,
) -> Vec<ContestInfo> {
    let mut contest_infos: Vec<ContestInfo> = Vec::new();

    for contest_id in contest_ids {
        match ContestInfo::keymap_get_by_id(storage, contest_id) {
            Some(contest_info) => {
                contest_infos.push(contest_info);
            }
            None => continue,
        }
    }

    return contest_infos;
}

pub fn assert_contest_ready_to_be_claimed(
    storage: &dyn Storage,
    env: &Env,
    contest_id: &u32,
) -> Result<ContestInfo, RealContestInfoError> {
    let contest_info: ContestInfo = ContestInfo::keymap_get_by_id(storage, &contest_id).unwrap();
    contest_info.assert_time_of_resolve_is_passed(env.block.time.seconds())?;
    Ok(contest_info)
}
