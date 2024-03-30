use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sp_secret_toolkit::macros::{identifiable::Identifiable, keymap::KeymapStorage};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema, KeymapStorage)]
pub struct UserInfo {
    address: Addr,
    contests: Vec<String>, // Using Vec<String> to preserve insertion order
}

impl UserInfo {
    // Constructor function to create a new UserInfo with an empty list of contests
    pub fn new(address: &Addr) -> Self {
        UserInfo {
            address: address.clone(),
            contests: Vec::new(),
        }
    }

    // Add a contest ID to the list
    pub fn add_contest(&mut self, contest_id: String) -> bool {
        if !self.contests.contains(&contest_id) {
            self.contests.push(contest_id);
            true
        } else {
            false
        }
    }

    // Retrieve all contests as a Vec<String>
    pub fn get_contests(&self) -> Vec<String> {
        self.contests.clone()
    }

    // Retrieve a paginated subset of contests
    pub fn get_contests_page(&self, page_num: usize, page_len: usize) -> Vec<String> {
        self.contests
            .iter()
            .skip(page_num * page_len)
            .take(page_len)
            .cloned()
            .collect()
    }
}

impl Identifiable for UserInfo {
    type ID = Addr; // Or another type that implements Serialize + DeserializeOwned

    fn id(&self) -> Self::ID {
        self.address.clone()
    }
}
