use serde::{Deserialize, Serialize};
use std::option::Option;

#[derive(Serialize, Deserialize, Debug)]
pub struct InitialAllocation {
    pub allocation: Allocation,
    pub cost_basis: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YearlyContribution {
    pub pre_tax: Allocation,
    pub roth: Allocation,
    pub after_tax: Allocation,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Allocation {
    pub value: f64,
    pub bond_percent: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitialState {
    pub pre_tax: InitialAllocation,
    pub roth: InitialAllocation,
    pub after_tax: InitialAllocation,
    pub contributions: YearlyContribution,
    pub expense_ratio: f64,
}
