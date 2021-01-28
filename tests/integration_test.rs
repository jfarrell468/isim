use isim::config::InitialState;
use isim::scenario::Scenario;

use serde_yaml;
use std::env;
use std::fs;
use std::path;

macro_rules! assert_eq_decimal_places {
    ($left:expr, $right:expr, $precision:expr $(,)?) => {
        let x = 10.0_f64.powi($precision);
        assert_eq!(($left * x).round() / x, ($right * x).round() / x);
    };
}

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
    assert_eq!(scenario.median_instance().real_value().round(), 1244.0);
    assert_eq!(scenario.length_years(), 20);
    assert_eq_decimal_places!(
        (scenario.median_instance().real_value() / 1000.0)
            .powf(1.0 / (scenario.length_years() as f64)),
        1.011,
        3
    );
}

#[test]
fn expense_ratio() {
    let config = config("expense_ratio.yaml");
    let mut scenario = Scenario::new(&config);
    scenario.run();
    assert_eq!(scenario.median_instance().real_value().round(), 3235.0);
    assert_eq!(scenario.length_years(), 20);
    assert_eq_decimal_places!(
        (scenario.median_instance().real_value() / 1000.0)
            .powf(1.0 / (scenario.length_years() as f64)),
        1.06,
        3
    );
}

#[test]
fn stock_growth() {
    let config = config("stock_growth.yaml");
    let mut scenario = Scenario::new(&config);
    scenario.run();
    assert_eq!(scenario.median_instance().real_value().round(), 3915.0);
    assert_eq!(scenario.length_years(), 20);
    assert_eq_decimal_places!(
        (scenario.median_instance().real_value() / 1000.0)
            .powf(1.0 / (scenario.length_years() as f64)),
        1.071,
        3
    );
}
