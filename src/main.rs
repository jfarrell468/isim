use isim::scenario;

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
    //println!("{}", serde_yaml::to_string(&config).unwrap());
    let mut scenario = scenario::Scenario::new(&config);
    scenario.run();
    scenario.report();
}
