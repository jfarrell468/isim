use crate::account::Account;
use crate::config::{AccountType, InflationAdjustment, InitialState, Measure, Phase, ReportField};
use crate::histret::RETURNS;
use crate::instance::Instance;
use crate::report::Report;

use cli_table::format::Justify;
use cli_table::Cell;
use itertools::join;
use num_format::{Locale, ToFormattedString};
use std::fmt::Debug;

// TODO: pub enum GrowthModel { Fixed, HistoricalPath, RandomYear }
// TODO: pub enum TaxStrategy { Taxed(Account), Untaxed(Account) }

#[derive(Debug)]
pub struct Scenario<'a> {
    year: usize,
    instances: Vec<Instance>,
    phases: &'a Vec<Phase>,
    report: Report<'a>,
}

impl Scenario<'_> {
    pub fn new(is: &InitialState) -> Scenario {
        let mut s = Scenario {
            year: 0,
            instances: Vec::with_capacity(RETURNS.len()),
            phases: &is.phases,
            report: Report::new(&is.report),
        };
        let pre_tax = Account::from_allocation(&is.initial_balance.pre_tax);
        let roth = Account::from_allocation(&is.initial_balance.roth);
        let after_tax = Account::from_allocation_and_basis(
            &is.initial_balance.after_tax,
            is.initial_balance.after_tax_cost_basis.unwrap_or(0.0),
        );
        for i in 0..RETURNS.len() {
            s.instances.push(Instance::new(
                i,
                pre_tax.clone(),
                roth.clone(),
                after_tax.clone(),
                is.expense_ratio / 100.0,
            ));
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
        self.instances.retain(|x| x.start() + y < RETURNS.len());
        for i in &mut self.instances {
            i.next(y, c, &RETURNS[self.year + i.start()]);
        }
        self.year += 1;
        self.instances.sort_by(|a, b| {
            (a.inflation_adjusted(a.value(&AccountType::Total)))
                .partial_cmp(&b.inflation_adjusted(b.value(&AccountType::Total)))
                .unwrap()
        });
        self.instances.len()
    }
    fn starting_year(&self, i: &Instance) -> i32 {
        RETURNS[i.start()].year
    }
    fn row(&self) -> Vec<cli_table::CellStruct> {
        let mut r: Vec<cli_table::CellStruct> = Vec::new();
        for f in self.report.config {
            r.push(match f {
                ReportField::YearsElapsed => self.years_elapsed().cell(),
                ReportField::WorstYears => join(&self.worst_starting_years(), ", ").cell(),
                ReportField::Value(m, a, inf) => {
                    let i = match m {
                        Measure::Median => self.median_instance(),
                        Measure::Worst => self.worst_instance(),
                    };
                    let v = i.value(a);
                    cfmt(match inf {
                        InflationAdjustment::Real => i.inflation_adjusted(v),
                        InflationAdjustment::Nominal => v,
                    })
                    .cell()
                    .justify(Justify::Right)
                }
                ReportField::BondPercent(a) => pfmt(self.median_instance().bond_fraction(a))
                    .cell()
                    .justify(Justify::Right),
                ReportField::CapGainsPercent => {
                    let i = self.median_instance();
                    pfmt(i.capital_gains() / i.value(&AccountType::AfterTax))
                        .cell()
                        .justify(Justify::Right)
                }
                ReportField::StartingYear(m) => match m {
                    Measure::Median => self.starting_year(self.median_instance()).cell(),
                    Measure::Worst => self.starting_year(self.worst_instance()).cell(),
                },
                ReportField::InterestAndDividends => cfmt(
                    self.median_instance()
                        .inflation_adjusted(self.median_instance().income.id),
                )
                .cell(),
                ReportField::StocksSold => cfmt(
                    self.median_instance()
                        .inflation_adjusted(self.median_instance().income.stocks_sold),
                )
                .cell(),
                ReportField::StocksBought => cfmt(
                    self.median_instance()
                        .inflation_adjusted(self.median_instance().income.stocks_bought),
                )
                .cell(),
                ReportField::CapitalGains => cfmt(
                    self.median_instance()
                        .inflation_adjusted(self.median_instance().income.cg),
                )
                .cell(),
                ReportField::SuccessRate => pfmt(self.success_ratio()).cell(),
                ReportField::RequiredMinimumDistribution => cfmt(
                    self.median_instance()
                        .inflation_adjusted(self.median_instance().income.pre_tax_withdrawal),
                )
                .cell(),
                ReportField::Taxes => cfmt(
                    self.median_instance()
                        .inflation_adjusted(self.median_instance().income.taxes),
                )
                .cell(),
                ReportField::TaxRate => {
                    let income = &self.median_instance().income;
                    pfmt(income.taxes / (income.id + income.cg + income.pre_tax_withdrawal)).cell()
                }
                ReportField::Cash => unimplemented!(),
                ReportField::ExpensesDoubleCheck => {
                    let i = self.median_instance();
                    cfmt(i.inflation_adjusted(
                        i.income.id + i.income.pre_tax_withdrawal + i.income.stocks_sold
                            - i.income.taxes
                            - i.income.stocks_bought,
                    ))
                    .cell()
                }
            });
        }
        r
    }
    pub fn worst_starting_years(&self) -> [i32; 3] {
        [
            self.starting_year(&self.instances[0]),
            self.starting_year(&self.instances[1]),
            self.starting_year(&self.instances[2]),
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
        self.instances
            .iter()
            .filter(|x| x.value(&AccountType::Total) > 0.0)
            .count() as f64
            / self.instances.len() as f64
    }
}

fn cfmt(x: f64) -> String {
    format!("${}", (x.round() as i64).to_formatted_string(&Locale::en))
}

fn pfmt(x: f64) -> String {
    format!("{:.1}%", 100.0 * x)
}
