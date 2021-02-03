use serde::{Deserialize, Serialize};
use std::option::Option;

#[derive(Serialize, Deserialize, Debug)]
pub struct InitialBalance {
    pub pre_tax: Allocation,
    pub roth: Allocation,
    pub after_tax: Allocation,
    pub after_tax_cost_basis: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Phase {
    pub config: PhaseType,
    pub years: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PhaseType {
    Accumulation(YearlyContribution),
    Growth,
    SimpleWithdrawAndRebalance(SimpleWithdrawal),
    WithdrawTaxAware(TaxAwareWithdrawal),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SimpleWithdrawal {
    pub amount: f64,
    pub bond_percent: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaxAwareWithdrawal {
    pub living_expenses: f64,
    pub birth_year: i32,
    pub bond_percent: f64,
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
    pub initial_balance: InitialBalance,
    pub phases: Vec<Phase>,
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
    StocksBought,
    CapitalGains,
    SuccessRate,
    RequiredMinimumDistribution,
    Taxes,
    Cash,
    ExpensesDoubleCheck,
    TaxRate,
}

impl ReportField {
    pub fn title(&self) -> String {
        match self {
            ReportField::YearsElapsed => String::from("Year"),
            ReportField::WorstYears => String::from("Worst Years"),
            ReportField::CapGainsPercent => String::from("CG%"),
            ReportField::BondPercent(a) => format!("Bond%,\n{:#?}", a),
            ReportField::Value(_, _, _) => format!("{:#?}", self),
            ReportField::StartingYear(m) => format!("Starting\nyear of\n{:#?}", m),
            ReportField::InterestAndDividends => String::from("I&D"),
            ReportField::StocksSold => String::from("Sold"),
            ReportField::StocksBought => String::from("Bought"),
            ReportField::CapitalGains => String::from("Cap\nGains"),
            ReportField::SuccessRate => String::from("Success\nrate"),
            ReportField::RequiredMinimumDistribution => String::from("RMD"),
            ReportField::Taxes => String::from("Taxes"),
            ReportField::Cash => String::from("Cash"),
            ReportField::ExpensesDoubleCheck => String::from("Calculated\nExpenses"),
            ReportField::TaxRate => String::from("Tax\nrate"),
        }
    }
}
