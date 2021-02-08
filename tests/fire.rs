use isim::config::*;
use isim::scenario::Scenario;

use serde_yaml;
use std::env;
use std::fmt::Write;
use std::fs;
use std::path;

const PRE_TAX_MAX: f64 = 19500.0;
const ROTH_MAX: f64 = 6000.0;

#[test]
fn fire() {
    let config = path::Path::new(env::var("CARGO_MANIFEST_DIR").unwrap().as_str())
        .join("examples")
        .join("early_retirement.yaml");
    let mut config: InitialState = serde_yaml::from_str(
        fs::read_to_string(config)
            .expect("Failed to read config")
            .as_str(),
    )
    .unwrap();
    config.report.clear();
    const SAVINGS_RATE: [f64; 11] = [0.25, 0.3, 0.35, 0.4, 0.45, 0.5, 0.55, 0.6, 0.65, 0.7, 0.75];
    const WORK_YEARS: [usize; 21] = [
        5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 22, 24, 26, 28, 30,
    ];

    let mut output = String::from("");
    write!(output, "Savings rate:").unwrap();
    for s in SAVINGS_RATE.iter() {
        write!(output, "   {:2.0}%", 100.0 * s).unwrap();
    }
    write!(output, "\nWorking years\n").unwrap();
    for y in WORK_YEARS.iter() {
        write!(output, "        {:2}   ", y).unwrap();
        config.phases[0].years = *y;
        for s in SAVINGS_RATE.iter() {
            let expenses =
                if let PhaseType::WithdrawTaxAware(c) = &config.phases.get(1).unwrap().config {
                    c.living_expenses
                } else {
                    panic!("Wrong config type: Expected phase 1 to be WithdrawTaxAware.");
                };
            if let PhaseType::Accumulation(c) = &mut config.phases.get_mut(0).unwrap().config {
                let savings = expenses * s / (1.0 - s);
                if savings < PRE_TAX_MAX {
                    c.pre_tax = savings;
                    c.roth = 0.0;
                    c.after_tax = 0.0;
                } else if savings < PRE_TAX_MAX + ROTH_MAX {
                    c.pre_tax = PRE_TAX_MAX;
                    c.roth = savings - PRE_TAX_MAX;
                    c.after_tax = 0.0;
                } else {
                    c.pre_tax = PRE_TAX_MAX;
                    c.roth = ROTH_MAX;
                    c.after_tax = savings - PRE_TAX_MAX - ROTH_MAX;
                }
            //println!("savings rate = {}, savings = {}, expenses = {}", s, savings, expenses);
            } else {
                panic!("Wrong config type: Expected phase 0 to be Accumulation.");
            }
            //println!("{:#?}", config);
            let mut scenario = Scenario::new(&config);
            scenario.run();
            write!(output, "  {:4.0}", 100.0 * scenario.success_ratio()).unwrap();
        }
        write!(output, "\n").unwrap();
    }
    print!("{}", output);
    let golden = path::Path::new(env::var("CARGO_MANIFEST_DIR").unwrap().as_str())
        .join("tests")
        .join("testdata")
        .join("fire_golden.txt");
    let golden = fs::read_to_string(golden).expect("Failed to read golden file");
    assert_eq!(output, golden);
}
