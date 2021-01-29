// This test attempts to replicate Table 3 of the Trinity study, which
// calculates success rates using market data from 1926 to 1995. It's
// not an exact match (we have more data, and our methodology is not
// exactly the same), but it's pretty close.

use isim::config::*;
use isim::scenario::Scenario;

use serde_yaml;
use std::env;
use std::fmt::Write;
use std::fs;
use std::path;

#[test]
fn trinity_study() {
    let config = path::Path::new(env::var("CARGO_MANIFEST_DIR").unwrap().as_str())
        .join("examples")
        .join("4_percent_rule.yaml");
    let mut config: InitialState = serde_yaml::from_str(
        fs::read_to_string(config)
            .expect("Failed to read config")
            .as_str(),
    )
    .unwrap();
    config.report.clear();
    const BOND_FRACTION: [f64; 5] = [0.0, 0.25, 0.5, 0.75, 1.0];
    const WITHDRAWAL_RATE: [f64; 10] = [0.03, 0.04, 0.05, 0.06, 0.07, 0.08, 0.09, 0.10, 0.11, 0.12];
    const YEARS: [usize; 4] = [15, 20, 25, 30];
    let mut output = String::from("");
    write!(output, "Withdrawal rate:").unwrap();
    for w in WITHDRAWAL_RATE.iter() {
        write!(output, "   {:2.0}%", 100.0 * w).unwrap();
    }
    write!(output, "\n").unwrap();
    for bf in BOND_FRACTION.iter() {
        write!(output, "{:.0}% bonds\n", 100.0 * bf).unwrap();
        config.initial_balance.roth.bond_percent = 100.0 * bf;
        // TODO: Instead of rerunning the whole simulation, stop at the indicated years.
        for y in YEARS.iter() {
            write!(output, "  {} years      ", y).unwrap();
            for w in WITHDRAWAL_RATE.iter() {
                config.phases.get_mut(0).unwrap().config =
                    PhaseType::SimpleWithdrawAndRebalance(SimpleWithdrawal {
                        amount: config.initial_balance.roth.value * w,
                        bond_percent: 100.0 * bf,
                    });
                config.phases.get_mut(0).unwrap().years = *y;
                let mut scenario = Scenario::new(&config);
                scenario.run();
                write!(output, "  {:4.0}", 100.0 * scenario.success_ratio()).unwrap();
            }
            write!(output, "\n").unwrap();
        }
    }
    print!("{}", output);
    let golden = path::Path::new(env::var("CARGO_MANIFEST_DIR").unwrap().as_str())
        .join("tests")
        .join("testdata")
        .join("trinity_study_golden.txt");
    let golden = fs::read_to_string(golden).expect("Failed to read golden file");
    assert_eq!(output, golden);
}
