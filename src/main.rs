mod asset;
mod histret;
mod scenario;

fn main() {
    let asset = asset::Asset::new(100.0);
    let mut scenario = scenario::Scenario::new(asset);
    while scenario.next() >= 5 {
        scenario.report();
    }
}
