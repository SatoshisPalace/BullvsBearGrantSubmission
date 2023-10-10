use super::{
    data::{
        bets::{get_bet, save_bet, verify_bet, UserContest},
        contest_bet_summary::{
            get_contest_bet_summary, save_contest_bet_summary, update_contest_bet_summary,
            ContestBetSummary,
        },
        contest_info::{get_contest, save_contest, verify_contest, ContestInfo},
    },
    error::ContestError,
};
use crate::{
    cryptography::cryptography::is_valid_signature, integrations::snip_20::snip_20::send,
    state::config_read,
};
use cosmwasm_std::{Addr, DepsMut, Response, StdResult, Uint128};

pub fn try_create_contest<'a>(
    deps: &mut DepsMut,
    contest_info: &ContestInfo,
    contest_info_signature_hex: &String,
) -> Result<(), ContestError> {
    let state = config_read(deps.storage).load()?;

    //Validate Signature
    let contest_info_json: String = contest_info.to_json();
    deps.api.debug("----------------");
    deps.api.debug(contest_info_json.as_str());
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
    amount: Option<Uint128>,
) -> Result<(), ContestError> {
    verify_bet(&sender, amount)?;
    verify_contest(deps.storage, contest_id, outcome_id)?;

    let user_contest = UserContest {
        address: sender.clone().unwrap(),
        contest_id,
    };
    let bet = get_bet(deps.storage, &user_contest);
    match bet {
        Some(bet) => {
            // User has bet before
            if outcome_id != bet.outcome_id {
                return Err(ContestError::CannotBetOnBothSides);
            }
            save_bet(
                deps.storage,
                sender.clone().unwrap(),
                contest_id,
                amount.unwrap() + bet.amount,
                outcome_id,
            )?;
        }
        None => save_bet(
            // User has not bet before
            deps.storage,
            sender.unwrap(),
            contest_id,
            amount.unwrap(),
            outcome_id,
        )?,
    }

    update_contest_bet_summary(deps.storage, contest_id, amount.unwrap(), outcome_id)?;
    Ok(())
}

pub fn try_claim(deps: &mut DepsMut, contest_id: u32, sender: Addr) -> StdResult<Response> {
    // Create a UserContest instance
    let user_contest = UserContest {
        address: sender.clone(),
        contest_id,
    };

    // Check if there is a bet for the given UserContest
    let bet_option = get_bet(deps.storage, &user_contest);

    match bet_option {
        Some(bet) => {
            // TODO check if user won the contest or not ///////////////////////////////////
            let contest_bet_summary_option = get_contest_bet_summary(deps.storage, contest_id);
            match contest_bet_summary_option {
                Some(contest_bet_summary) => {
                    // Calculate the user's share
                    let user_share =
                        contest_bet_summary.calculate_user_share(bet.amount, bet.outcome_id)?;
                    return send(deps, sender.to_string(), user_share);
                }
                None => Err(ContestError::ContestNotFound(contest_id).into()),
            }
        }
        None => Err(ContestError::NoBetForUserContest { user_contest }.into()),
    }
}
