use crate::account::Account;
use crate::config::{InitialState, YearlyContribution};
use crate::histret::{HistoricalYear, RETURNS};

use num_format::{Locale, ToFormattedString};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Instance {
    start: usize,
    pre_tax: Account,
    roth: Account,
    after_tax: Account,
    inflation: f64,
}

// TODO: pub enum GrowthModel { Fixed, HistoricalPath, RandomYear }
// TODO: pub enum TaxStrategy { Taxed(Account), Untaxed(Account) }

#[derive(Debug)]
pub struct Scenario {
    year: usize,
    instances: Vec<Instance>,
    contributions: YearlyContribution,
    expense_ratio: f64,
}

impl Scenario {
    pub fn new(is: InitialState) -> Scenario {
        let mut s = Scenario {
            year: 0,
            instances: Vec::with_capacity(RETURNS.len()),
            contributions: is.contributions,
            expense_ratio: is.expense_ratio / 100.0,
        };
        let pre_tax = Account::from_allocation(&is.pre_tax);
        let roth = Account::from_allocation(&is.roth);
        let after_tax = Account::from_allocation(&is.after_tax);
        for i in 0..RETURNS.len() {
            s.instances.push(Instance {
                start: i,
                pre_tax: pre_tax.clone(),
                roth: roth.clone(),
                after_tax: after_tax.clone(),
                inflation: 1.0,
            });
        }
        s
    }
    pub fn next(&mut self) -> usize {
        let y = self.year;
        self.instances.retain(|x| x.start + y < RETURNS.len());
        for i in &mut self.instances {
            // For now, discard the interest and dividends in the after-tax account.
            // This is because it is included implicitly in our estimate of
            // yearly contributions.
            i.grow_and_reinvest(&RETURNS[self.year + i.start], self.expense_ratio);
            i.contribute(&self.contributions);
            i.inflation *= 1.0 + &RETURNS[self.year + i.start].inflation;
        }
        self.year += 1;
        self.len()
    }
    pub fn print_header(&mut self) {
        println!("    -------- Median --------  ---- Pre-Tax ----  ----- Roth ------  ------- After Tax ------");
        println!("Year      Value  Bond%  Year       Value  Bond%       Value  Bond%       Value  Bond%    CG%    5th %ile Worst Years");
    }
    pub fn report(&mut self) {
        self.instances.sort_by(|a, b| {
            (a.value() / a.inflation)
                .partial_cmp(&(b.value() / b.inflation))
                .unwrap()
        });
        let median = &self.instances[self.instances.len() / 2];
        let bad = &self.instances[self.instances.len() / 20];
        println!("{:>3.0}{:>12}{:>6.1}%{:>6.0}{:>12}{:>6.1}%{:>12}{:>6.0}%{:>12}{:>6.1}%{:>6.1}%{:>12}{:>5.0},{:>5.0},{:>5.0}",
                 self.year,
                 cfmt(median.value() / median.inflation),
                 100.0 * median.bond_value() / median.value(),
                 RETURNS[median.start].year,
                 cfmt(median.pre_tax.value() / median.inflation),
                 median.pre_tax.bond_percent(),
                 cfmt(median.roth.value() / median.inflation),
                 median.roth.bond_percent(),
                 cfmt(median.after_tax.value() / median.inflation),
                 median.after_tax.bond_percent(),
                 100.0 * median.after_tax.capital_gains() / median.after_tax.value(),
                 cfmt(bad.value() / bad.inflation),
                 RETURNS[self.instances[0].start].year,
                 RETURNS[self.instances[1].start].year,
                 RETURNS[self.instances[2].start].year);
    }
    pub fn len(&self) -> usize {
        self.instances.len()
    }
}

impl Instance {
    pub fn grow_and_reinvest(&mut self, r: &HistoricalYear, e: f64) -> f64 {
        self.pre_tax.grow_and_reinvest(r, e);
        self.roth.grow_and_reinvest(r, e);
        self.after_tax.grow(r, e)
    }
    pub fn contribute(&mut self, c: &YearlyContribution) {
        // Pre-tax contributions are 100% bonds.
        // Roth and after-tax are 100% stocks.
        self.pre_tax.invest(0.0, c.pre_tax);
        self.roth.invest(c.roth, 0.0);
        self.after_tax.invest(c.after_tax, 0.0);
    }
    pub fn value(&self) -> f64 {
        self.pre_tax.value() + self.roth.value() + self.after_tax.value()
    }
    pub fn bond_value(&self) -> f64 {
        self.pre_tax.bonds.value + self.roth.bonds.value + self.after_tax.bonds.value
    }
    pub fn capital_gains(&self) -> f64 {
        self.after_tax.capital_gains()
    }
}

fn cfmt(x: f64) -> String {
    "$".to_owned() + &(x.round() as i64).to_formatted_string(&Locale::en)
}
