use std::{fs, io, path::Path};

use crate::data::contest_info::ContestInfo;

fn read_contest_data(folder: &str, file_number: u8) -> io::Result<ContestInfo> {
    let path = Path::new(folder).join(format!("{}.json", file_number));
    let data = fs::read_to_string(path)?;

    let json: serde_json::Value = serde_json::from_str(&data)?;
    let contest_info: ContestInfo = serde_json::from_value(json["contest_info"].clone())?;

    Ok(contest_info)
}

// Example functions to read from each specific folder
pub fn get_contest_closed_awaiting_results(file_number: u8) -> io::Result<ContestInfo> {
    read_contest_data(
        "./src/tests/contest_data/closed_awaiting_results",
        file_number,
    )
}

pub fn get_contest_closed_claimable(file_number: u8) -> io::Result<ContestInfo> {
    read_contest_data("./src/tests/contest_data/closed_claimable", file_number)
}

pub fn get_contest_open(file_number: u8) -> io::Result<ContestInfo> {
    read_contest_data("./src/tests/contest_data/open", file_number)
}