use serde::Deserialize;
use serde_json::Value;
use std::io::Write;
use std::{collections::HashMap, fs::File};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Owner {
    name: String,
    display_name: String,
}

#[derive(Deserialize)]
struct Tracker {
    owner: Owner,
    #[serde(flatten)]
    _ignored: Value,
}

#[derive(Deserialize)]
struct BlockList {
    trackers: HashMap<String, Tracker>,
    #[serde(flatten)]
    _ignored: Value,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let block_list = reqwest::blocking::get(
        "https://staticcdn.duckduckgo.com/trackerblocking/appTB/1.0/blocklist.json",
    )?
    .json::<BlockList>()?;
    let mut block_list_file = File::create("./block_list.txt")?;
    for (hostname, tracker) in block_list.trackers {
        writeln!(block_list_file, "# {}", tracker.owner.display_name)?;
        writeln!(block_list_file, "0.0.0.0 *.{}", hostname)?;
        println!("added {} to blocklist", tracker.owner.name);
    }
    Ok(())
}
