mod account;
mod asset;
mod config;
mod histret;
mod scenario;
mod tax;

use serde_yaml;
use std::env;
use std::fs;

fn main() {
    let config = env::args().nth(1).expect("Config file not specified");
    let config = serde_yaml::from_str(
        fs::read_to_string(config)
            .expect("Failed to read config")
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
