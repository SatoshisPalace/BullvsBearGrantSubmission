use cosmwasm_std::Addr;
use secret_toolkit::storage::{Item, Keymap};

use super::contest_info::ContestId;

static USERS_CONTESTS_MAP: Keymap<u32, ContestId> = Keymap::new(b"users_contests_map");

static LAST_CLAIMED_INDEX: Item<u32> = Item::new(b"users_last_claimed_index");

pub static TOTAL_USERS: Item<u32> = Item::new(b"TOTAL_USERS");

pub fn get_users_contest_map(user: &Addr) -> Keymap<u32, ContestId> {
    USERS_CONTESTS_MAP.add_suffix(user.as_bytes())
}

pub fn get_users_last_claimed_index<'a>(user: &'a Addr) -> Item<'a, u32> {
    LAST_CLAIMED_INDEX.add_suffix(user.as_bytes())
}
