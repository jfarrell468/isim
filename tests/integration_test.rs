use isim::scenario;

use serde_yaml;
use std::env;
use std::fs;
use std::path;

#[cfg(test)]
fn run_example(name: &str) {
    let config = path::Path::new(env::var("CARGO_MANIFEST_DIR").unwrap().as_str())
        .join("examples")
        .join(name);
    let config = serde_yaml::from_str(
        fs::read_to_string(config)
            .expect("Failed to read config")
            .as_str(),
    )
    .unwrap();
    println!("{:#?}", config);
    let mut scenario = scenario::Scenario::new(config);
    scenario.run();
}

#[test]
fn bond_growth() {
    run_example("bond_growth.yaml")
}

#[test]
fn expense_ratio() {
    run_example("expense_ratio.yaml")
}

#[test]
fn stock_growth() {
    run_example("stock_growth.yaml")
}
