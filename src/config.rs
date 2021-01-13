use serde::{Deserialize, Serialize};
use std::option::Option;

#[derive(Serialize, Deserialize, Debug)]
pub struct InitialAllocation {
    pub balance: f64,
    pub bond_percent: f64,
    pub cost_basis: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YearlyContribution {
    pub pre_tax: f64,
    pub roth: f64,
    pub after_tax: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitialState {
    pub pre_tax: InitialAllocation,
    pub roth: InitialAllocation,
    pub after_tax: InitialAllocation,
    pub contributions: YearlyContribution,
}
