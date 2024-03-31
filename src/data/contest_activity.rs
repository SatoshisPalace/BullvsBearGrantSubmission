use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sp_secret_toolkit::macros::singleton::SingletonStorage;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema, SingletonStorage)]
pub struct ContestActivity {
    active_contest_ids: Vec<String>,
}

impl ContestActivity {
    // Initializes a new instance of ContestActivity with an empty vector.
    pub fn new() -> Self {
        ContestActivity {
            active_contest_ids: Vec::new(),
        }
    }

    // Returns a reference to the vector of active contest IDs.
    pub fn get_active_contests_ids(&self) -> &Vec<String> {
        &self.active_contest_ids
    }

    // Adds a new contest ID to the list of active contest IDs.
    pub fn add_contest(&mut self, contest_id: &String) {
        if !self.active_contest_ids.contains(contest_id) {
            self.active_contest_ids.push(contest_id.clone());
        }
    }

    // Removes a contest ID from the list of active contest IDs.
    pub fn remove_contest(&mut self, contest_id: &String) {
        self.active_contest_ids.retain(|x| x != contest_id);
    }
}
