use std::collections::BTreeSet;

use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sp_secret_toolkit::macros::{identifiable::Identifiable, keymap::KeymapStorage};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema, KeymapStorage)]
pub struct UserInfo {
    address: Addr,
    contests: BTreeSet<u32>, // Using BTreeSet instead of Vec
}

impl UserInfo {
    // Constructor function to create a new UserInfo with an empty set of contests
    pub fn new(address: &Addr) -> Self {
        UserInfo {
            address: address.clone(),
            contests: BTreeSet::new(),
        }
    }

    // Add a contest ID to the set
    pub fn add_contest(&mut self, contest_id: &u32) -> bool {
        // Changed &u32 to u32, as we don't need to clone u32
        self.contests.insert(contest_id.clone())
    }

    // Retrieve all contests as a Vec<u32>, since sets don't directly support slicing
    pub fn get_contests(&self) -> Vec<u32> {
        self.contests.iter().cloned().collect()
    }

    // Retrieve a paginated subset of contests
    pub fn get_contests_page(&self, page_num: usize, page_len: usize) -> Vec<u32> {
        self.contests
            .iter()
            .cloned()
            .skip(page_num * page_len)
            .take(page_len)
            .collect()
    }
}

impl Identifiable for UserInfo {
    type ID = Addr; // Or another type that implements Serialize + DeserializeOwned

    fn id(&self) -> Self::ID {
        self.address.clone()
    }
}
