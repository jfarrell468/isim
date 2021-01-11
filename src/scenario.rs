use crate::asset::*;
use crate::histret::*;

use std::fmt::Debug;

#[derive(Debug)]
pub struct Instance {
    start: usize,
    asset: Asset,
}

#[derive(Debug)]
pub struct Scenario {
    year: usize,
    instances: Vec<Instance>,
}

impl Scenario {
    pub fn new(asset: Asset) -> Scenario {
        let mut s = Scenario {
            year: 0,
            instances: Vec::with_capacity(RETURNS.len())
        };
        for i in 0..RETURNS.len() {
            s.instances.push(Instance { start: i, asset: asset.clone() });
        }
        s
    }
    pub fn next(&mut self) -> usize {
        let y = self.year;
        self.instances.retain(|x| x.start + y < RETURNS.len());
        for instance in &mut self.instances {
            let id = instance.asset.grow(&RETURNS[self.year + instance.start].stocks);
            instance.asset.invest(id);
        }
        self.year += 1;
        self.len()
    }
    pub fn report(&mut self) {
        self.instances.sort_by(|a,b| a.asset.value.partial_cmp(&b.asset.value).unwrap());
        println!("{:>12.2}{:>12.2}{:>12.2}{:>12.2}{:>12.2} {}, {}, {}, {}, {}",
                 self.year,
                 self.instances[self.instances.len() / 2].asset.value,
                 self.instances[self.instances.len() / 10].asset.value,
                 self.instances[0].asset.value,
                 self.instances.len(),
                 RETURNS[self.instances[0].start].year,
                 RETURNS[self.instances[1].start].year,
                 RETURNS[self.instances[2].start].year,
                 RETURNS[self.instances[3].start].year,
                 RETURNS[self.instances[4].start].year);
    }
    pub fn len(&self) -> usize {
        self.instances.len()
    }
}