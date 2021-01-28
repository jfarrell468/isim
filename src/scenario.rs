use crate::account::Account;
use crate::config::{
    AccountType, InflationAdjustment, InitialState, Measure, Phase, PhaseType, ReportField,
    YearlyContribution,
};
use crate::histret::{HistoricalYear, RETURNS};
use crate::report::Report;
use crate::tax::how_much_to_sell;

use cli_table::format::Justify;
use cli_table::Cell;
use itertools::join;
use num_format::{Locale, ToFormattedString};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Instance {
    start: usize,
    pub pre_tax: Account,
    pub roth: Account,
    pub after_tax: Account,
    pub inflation: f64,
    pub id: f64,
    pub stocks_sold: f64,
    pub cg: f64,
}

// TODO: pub enum GrowthModel { Fixed, HistoricalPath, RandomYear }
// TODO: pub enum TaxStrategy { Taxed(Account), Untaxed(Account) }

#[derive(Debug)]
pub struct Scenario<'a> {
    year: usize,
    instances: Vec<Instance>,
    phases: &'a Vec<Phase>,
    expense_ratio: f64,
    report: Report<'a>,
}

impl Scenario<'_> {
    pub fn new(is: &InitialState) -> Scenario {
        let mut s = Scenario {
            year: 0,
            instances: Vec::with_capacity(RETURNS.len()),
            phases: &is.phases,
            expense_ratio: is.expense_ratio / 100.0,
            report: Report::new(&is.report),
        };
        let pre_tax = Account::from_allocation(&is.initial_balance.pre_tax);
        let roth = Account::from_allocation(&is.initial_balance.roth);
        let after_tax = Account::from_allocation_and_basis(
            &is.initial_balance.after_tax,
            is.initial_balance.after_tax_cost_basis.unwrap_or(0.0),
        );
        for i in 0..RETURNS.len() {
            s.instances.push(Instance {
                start: i,
                pre_tax: pre_tax.clone(),
                roth: roth.clone(),
                after_tax: after_tax.clone(),
                inflation: 1.0,
                id: 0.0,
                stocks_sold: 0.0,
                cg: 0.0,
            });
        }
        s
    }
    pub fn run(&mut self) {
        self.report.row(self.row());
        for i in 0..self.phases.len() {
            for _ in 0..self.phases[i].years {
                self.next(i);
                self.report.row(self.row());
            }
        }
    }
    pub fn report(&mut self) {
        self.report.print();
    }
    fn next(&mut self, i: usize) -> usize {
        let c = &self.phases[i].config;
        let y = self.year;
        self.instances.retain(|x| x.start + y < RETURNS.len());
        for i in &mut self.instances {
            match c {
                PhaseType::Accumulation(contributions) => {
                    i.grow_and_reinvest(&RETURNS[self.year + i.start], self.expense_ratio);
                    // For now, discard the interest and dividends in the after-tax account.
                    // This is because it is included implicitly in our estimate of
                    // yearly contributions.
                    i.contribute(contributions);
                }
                PhaseType::Growth => {
                    i.grow_and_reinvest(&RETURNS[self.year + i.start], self.expense_ratio);
                }
                PhaseType::SimpleWithdrawAndRebalance(w) => {
                    let total_sales = w.amount * i.inflation;
                    let bond_sales = total_sales * w.bond_percent / 100.0;
                    i.roth.bonds.sell_preserving_cg_ratio(bond_sales);
                    i.roth
                        .stocks
                        .sell_preserving_cg_ratio(total_sales - bond_sales);
                    i.grow_and_reinvest(&RETURNS[self.year + i.start], self.expense_ratio);
                    let bond_target = i.roth.value() * w.bond_percent / 100.0;
                    let bond_sales = i.roth.bonds.value - bond_target;
                    if bond_sales > 0.0 {
                        i.roth.bonds.sell_preserving_cg_ratio(bond_sales);
                        i.roth.stocks.invest(bond_sales);
                    } else {
                        i.roth.stocks.sell_preserving_cg_ratio(-bond_sales);
                        i.roth.bonds.invest(-bond_sales);
                    }
                }
                PhaseType::WithdrawTaxAware(w) => {
                    let id = i.grow_and_reinvest(&RETURNS[self.year + i.start], self.expense_ratio);
                    // We should really withdraw at the start of the year. But we
                    // don't know what our interest and dividends will be, so we can't
                    // calculate taxes.
                    // TODO: Add support for cash savings account.
                    i.withdraw_after_tax(w.living_expenses, id);
                    // TODO: Calculate failure rate.
                }
            }
            i.inflation *= 1.0 + &RETURNS[self.year + i.start].inflation;
        }
        self.year += 1;
        self.instances
            .sort_by(|a, b| (a.real_value()).partial_cmp(&b.real_value()).unwrap());
        self.instances.len()
    }
    fn row(&self) -> Vec<cli_table::CellStruct> {
        let mut r: Vec<cli_table::CellStruct> = Vec::new();
        for f in self.report.config {
            r.push(match f {
                ReportField::YearsElapsed => self.years_elapsed().cell(),
                ReportField::WorstYears => join(&self.worst_starting_years(), ", ").cell(),
                ReportField::Value(m, a, i) => {
                    let inst = match m {
                        Measure::Median => self.median_instance(),
                        Measure::Worst => self.worst_instance(),
                    };
                    let v = match a {
                        AccountType::Total => inst.value(),
                        AccountType::PreTax => inst.pre_tax.value(),
                        AccountType::Roth => inst.roth.value(),
                        AccountType::AfterTax => inst.after_tax.value(),
                    };
                    cfmt(match i {
                        InflationAdjustment::Real => v / inst.inflation,
                        InflationAdjustment::Nominal => v,
                    })
                    .cell()
                    .justify(Justify::Right)
                }
                ReportField::BondPercent(a) => {
                    let inst = self.median_instance();
                    pfmt(match a {
                        AccountType::Total => inst.bond_value() / inst.value(),
                        AccountType::PreTax => inst.pre_tax.bond_fraction(),
                        AccountType::Roth => inst.roth.bond_fraction(),
                        AccountType::AfterTax => inst.after_tax.bond_fraction(),
                    })
                    .cell()
                    .justify(Justify::Right)
                }
                ReportField::CapGainsPercent => {
                    let i = self.median_instance();
                    pfmt(i.capital_gains() / i.after_tax.value())
                        .cell()
                        .justify(Justify::Right)
                }
                ReportField::StartingYear(m) => match m {
                    Measure::Median => self.median_instance().starting_year().cell(),
                    Measure::Worst => self.worst_instance().starting_year().cell(),
                },
                ReportField::InterestAndDividends => {
                    cfmt(self.median_instance().id / self.median_instance().inflation).cell()
                }
                ReportField::StocksSold => {
                    cfmt(self.median_instance().stocks_sold / self.median_instance().inflation)
                        .cell()
                }
                ReportField::CapitalGains => {
                    cfmt(self.median_instance().cg / self.median_instance().inflation).cell()
                }
                ReportField::SuccessRate => pfmt(self.success_ratio()).cell(),
            });
        }
        r
    }
    pub fn worst_starting_years(&self) -> [i32; 3] {
        [
            self.instances[0].starting_year(),
            self.instances[1].starting_year(),
            self.instances[2].starting_year(),
        ]
    }
    pub fn years_elapsed(&self) -> usize {
        self.year
    }
    pub fn median_instance(&self) -> &Instance {
        &self.instances[self.instances.len() / 2]
    }
    pub fn worst_instance(&self) -> &Instance {
        &self.instances[0]
    }
    pub fn length_years(&self) -> usize {
        self.phases.iter().map(|x| x.years).sum()
    }
    pub fn success_ratio(&self) -> f64 {
        self.instances.iter().filter(|x| x.value() > 0.0).count() as f64
            / self.instances.len() as f64
    }
}

impl Instance {
    pub fn grow_and_reinvest(&mut self, r: &HistoricalYear, e: f64) -> f64 {
        self.pre_tax.grow_and_reinvest(r, e);
        self.roth.grow_and_reinvest(r, e);
        self.id = self.after_tax.grow(r, e);
        self.id
    }
    pub fn contribute(&mut self, c: &YearlyContribution) {
        self.pre_tax.invest_allocation(&c.pre_tax);
        self.roth.invest_allocation(&c.roth);
        self.after_tax.invest_allocation(&c.after_tax);
    }
    pub fn withdraw_after_tax(&mut self, living_expenses: f64, id: f64) {
        let sell = how_much_to_sell(
            living_expenses,
            id / self.inflation,
            self.after_tax.capital_gains() / self.after_tax.value(),
        );
        self.stocks_sold = sell * self.inflation;
        self.cg = self.stocks_sold * self.after_tax.stocks.capital_gains_ratio();
        self.after_tax.sell_stocks(self.stocks_sold);
    }
    pub fn value(&self) -> f64 {
        self.pre_tax.value() + self.roth.value() + self.after_tax.value()
    }
    pub fn bond_value(&self) -> f64 {
        self.pre_tax.bonds.value + self.roth.bonds.value + self.after_tax.bonds.value
    }
    pub fn real_value(&self) -> f64 {
        self.value() / self.inflation
    }
    pub fn capital_gains(&self) -> f64 {
        self.after_tax.capital_gains()
    }
    pub fn starting_year(&self) -> i32 {
        RETURNS[self.start].year
    }
}

fn cfmt(x: f64) -> String {
    format!("${}", (x.round() as i64).to_formatted_string(&Locale::en))
}

fn pfmt(x: f64) -> String {
    format!("{:.1}%", 100.0 * x)
}
