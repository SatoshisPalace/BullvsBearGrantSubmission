use cosmwasm_std::{DepsMut, Uint128, Addr};
use crate::{cryptography::cryptography::is_valid_signature, state::config_read};
use super::{data::{contest_info::{save_contest, ContestInfo, verify_contest, get_contest}, bets::{verify_bet, save_bet}, contest_bet_summary::{update_contest_bet_summary, ContestBetSummary, save_contest_bet_summary}}, error::ContestError};

pub fn try_create_contest<'a>(
    deps: &mut DepsMut,
    contest_info: &ContestInfo,
    contest_info_signature_hex: &String,
) -> Result<(), ContestError > {
    let state = config_read(deps.storage).load()?;

    //Validate Signature
    let contest_info_json: String = contest_info.to_json();
    is_valid_signature(
        deps.api,
        state.satoshis_palace.as_str(),
        &contest_info_json,
        &contest_info_signature_hex,
    )?;

    // Contests cannot be recreated
    let contest_id = contest_info.id();
    if let Some(_) = get_contest(deps.storage, contest_id) {
        return Err(ContestError::ContestAlreadyExist(contest_id));
    }

    save_contest(deps.storage, &contest_info);

    // Initialize and save the ContestBetSummary
    let contest_bet_summary = ContestBetSummary::initialize(contest_info);
    save_contest_bet_summary(deps.storage, contest_info.id, &contest_bet_summary)?;

    Ok(())
}

pub fn try_bet_on_contest(
	deps: &mut DepsMut,
	contest_id: u32,
	outcome_id: u8,
	sender: Option<Addr>,
	amount: Option<Uint128>
)-> Result<(), ContestError > {
	verify_bet(&sender, amount)?;
	verify_contest(deps.storage, contest_id, outcome_id)?;
	save_bet(deps.storage, sender.unwrap(), contest_id, amount.unwrap(), outcome_id)?;
	update_contest_bet_summary(deps.storage, contest_id, amount.unwrap(), outcome_id)?;
	Ok(())
}