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
    pub years: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WithdrawAfterTax {
    pub yearly_spending: f64,
    pub years: usize,
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
    pub withdraw_after_tax: WithdrawAfterTax,
    pub expense_ratio: f64,
    pub report: Vec<ReportField>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AccountType {
    Total,
    PreTax,
    Roth,
    AfterTax,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Measure {
    Median,
    // TODO: FifthPercentile,
    Worst,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum InflationAdjustment {
    Real,
    Nominal,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ReportField {
    YearsElapsed,
    WorstYears,
    Value(Measure, AccountType, InflationAdjustment),
    BondPercent(AccountType),
    CapGainsPercent,
    StartingYear(Measure),
    InterestAndDividends,
    StocksSold,
    CapitalGains,
}

impl ReportField {
    pub fn title(&self) -> String {
        match self {
            ReportField::YearsElapsed => String::from("Year"),
            ReportField::WorstYears => String::from("Worst Starting Years"),
            ReportField::CapGainsPercent => String::from("CG%"),
            ReportField::BondPercent(a) => format!("Bond%,\n{:#?}", a),
            ReportField::Value(_, _, _) => format!("{:#?}", self),
            ReportField::StartingYear(m) => format!("Starting\nyear of\n{:#?}", m),
            ReportField::InterestAndDividends => String::from("I&D"),
            ReportField::StocksSold => String::from("Sold"),
            ReportField::CapitalGains => String::from("Cap\nGains"),
        }
    }
}
