use isim::assert_eq_decimal_places;
use isim::config::InitialState;
use isim::scenario::Scenario;

use serde_yaml;
use std::env;
use std::fs;
use std::path;

#[cfg(test)]
fn config(name: &str) -> InitialState {
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
    config
}

#[test]
fn bond_growth() {
    let config = config("bond_growth.yaml");
    let mut scenario = Scenario::new(&config);
    scenario.run();
    let rv = scenario
        .median_instance()
        .inflation_adjusted(scenario.median_instance().value());
    assert_eq!(rv.round(), 1244.0);
    assert_eq!(scenario.length_years(), 20);
    assert_eq_decimal_places!(
        (rv / 1000.0).powf(1.0 / (scenario.length_years() as f64)),
        1.011,
        3
    );
}

#[test]
fn expense_ratio() {
    let config = config("expense_ratio.yaml");
    let mut scenario = Scenario::new(&config);
    scenario.run();
    let rv = scenario
        .median_instance()
        .inflation_adjusted(scenario.median_instance().value());
    assert_eq!(rv.round(), 3235.0);
    assert_eq!(scenario.length_years(), 20);
    assert_eq_decimal_places!(
        (rv / 1000.0).powf(1.0 / (scenario.length_years() as f64)),
        1.06,
        3
    );
}

#[test]
fn stock_growth() {
    let config = config("stock_growth.yaml");
    let mut scenario = Scenario::new(&config);
    scenario.run();
    let rv = scenario
        .median_instance()
        .inflation_adjusted(scenario.median_instance().value());
    assert_eq!(rv.round(), 3915.0);
    assert_eq!(scenario.length_years(), 20);
    assert_eq_decimal_places!(
        (rv / 1000.0).powf(1.0 / (scenario.length_years() as f64)),
        1.071,
        3
    );
}

#[test]
fn mixed_growth() {
    let config = config("mixed_growth.yaml");
    let mut scenario = Scenario::new(&config);
    scenario.run();
    let rv = scenario
        .median_instance()
        .inflation_adjusted(scenario.median_instance().value());
    assert_eq!(rv.round(), 3364.0);
    assert_eq!(scenario.length_years(), 20);
    assert_eq_decimal_places!(
        (rv / 1000.0).powf(1.0 / (scenario.length_years() as f64)),
        1.063,
        3
    );
}

#[test]
fn four_percent_rule() {
    let config = config("4_percent_rule.yaml");
    let mut scenario = Scenario::new(&config);
    scenario.run();
}
