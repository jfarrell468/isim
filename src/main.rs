mod account;
mod asset;
mod config;
mod histret;
mod scenario;

use serde_yaml;
use std::fs;

fn main() {
    let config = serde_yaml::from_str(
        fs::read_to_string("config.yaml")
            .expect("Failed to read config.yaml")
            .as_str(),
    )
    .unwrap();
    //println!("{:#?}", config);
    let mut scenario = scenario::Scenario::new(config);
    scenario.print_header();
    scenario.report();
    for _ in 0..10 {
        scenario.next();
        scenario.report();
    }
}
