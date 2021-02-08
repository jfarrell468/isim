use crate::account::Account;
use crate::config::{PhaseType, YearlyContribution};
use crate::histret::HistoricalYear;
use crate::rmd::rmd_fraction;
use crate::tax::tax;

use chrono::{Datelike, Utc};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Instance {
    // TODO: Remove start, and put it in scenario.
    start: usize,
    cash: f64,
    pre_tax: Account,
    roth: Account,
    after_tax: Account,
    expense_ratio: f64,
    inflation: f64,
    // TODO: Make private
    pub income: Income,
}

#[derive(Debug)]
pub struct Income {
    // TODO: Make private
    pub id: f64,
    pub rmd: f64,
    pub after_tax_bought: f64,

    pub after_tax_sold: f64,
    pub cg: f64,
    pub ira_sold: f64,
    pub roth_sold: f64,

    pub taxes: f64,
}

#[derive(Debug, PartialEq)]
pub struct ValueByAccount {
    pub pre_tax: f64,
    pub roth: f64,
    pub after_tax: f64,
}

impl Instance {
    pub fn new(
        start: usize,
        pre_tax: Account,
        roth: Account,
        after_tax: Account,
        expense_ratio: f64,
    ) -> Instance {
        Instance {
            start: start,
            cash: 0.0,
            pre_tax: pre_tax,
            roth: roth,
            after_tax: after_tax,
            expense_ratio: expense_ratio,
            inflation: 1.0,
            income: Income {
                id: 0.0,
                rmd: 0.0,
                after_tax_bought: 0.0,
                after_tax_sold: 0.0,
                cg: 0.0,
                ira_sold: 0.0,
                roth_sold: 0.0,
                taxes: 0.0,
            },
        }
    }
    pub fn start(&self) -> usize {
        self.start
    }
    pub fn inflation_adjusted(&self, v: f64) -> f64 {
        v / self.inflation
    }
    pub fn value(&self) -> f64 {
        self.pre_tax.value() + self.roth.value() + self.after_tax.value()
    }
    pub fn value_by_account(&self) -> ValueByAccount {
        ValueByAccount {
            pre_tax: self.pre_tax.value(),
            roth: self.roth.value(),
            after_tax: self.after_tax.value(),
        }
    }
    pub fn bond_fraction(&self) -> f64 {
        if self.value() > 0.0 {
            self.bond_value() / self.value()
        } else {
            0.0
        }
    }
    pub fn bond_fraction_by_account(&self) -> ValueByAccount {
        ValueByAccount {
            pre_tax: self.pre_tax.bond_fraction(),
            roth: self.roth.bond_fraction(),
            after_tax: self.after_tax.bond_fraction(),
        }
    }
    pub fn bond_value(&self) -> f64 {
        self.pre_tax.bonds.value + self.roth.bonds.value + self.after_tax.bonds.value
    }
    pub fn capital_gains(&self) -> f64 {
        self.after_tax.capital_gains()
    }
    pub fn goal_allocations(&self, b: f64) -> ValueByAccount {
        goal_allocations(&self.value_by_account(), b)
    }
    pub fn allocate_withdrawals(&self, a: f64) -> ValueByAccount {
        allocate_withdrawals(&self.value_by_account(), a)
    }

    // Returns taxable interest and dividends.
    pub fn grow_and_reinvest(&mut self, r: &HistoricalYear, e: f64) -> f64 {
        self.pre_tax.grow_and_reinvest(&r.stocks, &r.tbonds, e);
        self.roth.grow_and_reinvest(&r.stocks, &r.tbonds, e);
        self.after_tax.grow_and_reinvest(&r.stocks, &r.tbonds, e)
    }

    pub fn contribute(&mut self, c: &YearlyContribution) {
        let goal_allocations = goal_allocations(
            &ValueByAccount {
                pre_tax: self.pre_tax.value() + c.pre_tax,
                roth: self.roth.value() + c.roth,
                after_tax: self.after_tax.value() + c.after_tax,
            },
            c.target_bond_percent / 100.0,
        );
        self.pre_tax
            .invest_with_goal_allocation(c.pre_tax, goal_allocations.pre_tax);
        self.roth
            .invest_with_goal_allocation(c.roth, goal_allocations.roth);
        self.after_tax
            .invest_with_goal_allocation(c.after_tax, goal_allocations.after_tax);
    }

    // Returns taxable (regular income, cap gains). $a is the amount to withdraw, b is the target
    // bond fraction.
    pub fn withdraw(&mut self, a: f64, b: f64) -> (f64, f64) {
        assert!(b >= 0.0);
        assert!(b <= 1.0);
        let w = self.allocate_withdrawals(a);
        let target_allocation = goal_allocations(
            &ValueByAccount {
                pre_tax: self.pre_tax.value() - w.pre_tax,
                roth: self.roth.value() - w.roth,
                after_tax: self.after_tax.value() - w.after_tax,
            },
            b,
        );
        self.roth
            .sell_with_goal_allocation(w.roth, target_allocation.roth);
        self.pre_tax
            .sell_with_goal_allocation(w.pre_tax, target_allocation.pre_tax);
        let cg = self
            .after_tax
            .sell_with_goal_allocation(w.after_tax, target_allocation.after_tax);
        (w.pre_tax, cg)
    }

    pub fn next(&mut self, y: usize, c: &PhaseType, r: &HistoricalYear) {
        self.income = Income {
            id: 0.0,
            rmd: 0.0,
            after_tax_bought: 0.0,
            after_tax_sold: 0.0,
            cg: 0.0,
            ira_sold: 0.0,
            roth_sold: 0.0,
            taxes: 0.0,
        };
        match c {
            PhaseType::Accumulation(contributions) => {
                // TODO: Configure an overall target bond percent, and use that to determine
                // allocations for each account.
                self.grow_and_reinvest(r, self.expense_ratio);
                self.contribute(contributions);
                self.inflation *= 1.0 + r.inflation;
            }
            PhaseType::Growth => {
                self.grow_and_reinvest(r, self.expense_ratio);
                self.inflation *= 1.0 + r.inflation;
            }
            PhaseType::SimpleWithdrawAndRebalance(w) => {
                self.withdraw(w.amount * self.inflation, w.bond_percent / 100.0);
                self.grow_and_reinvest(r, self.expense_ratio);
                let allocations = self.goal_allocations(w.bond_percent / 100.0);
                self.pre_tax.rebalance(allocations.pre_tax);
                self.roth.rebalance(allocations.roth);
                self.inflation *= 1.0 + r.inflation;
            }
            PhaseType::WithdrawTaxAware(w) => {
                let b = w.bond_percent / 100.0;
                let new_inflation = self.inflation * (1.0 + r.inflation);
                let real_expenses = w.living_expenses * new_inflation;

                // RMDs are calculated at the beginning of the year.
                self.income.rmd = self.pre_tax.value()
                    * rmd_fraction(Utc::now().year() - w.birth_year + y as i32);
                self.pre_tax.sell_preserving_allocation(self.income.rmd);

                // Market growth. After-tax interest and dividends.
                self.income.id = self.grow_and_reinvest(r, self.expense_ratio);

                // TODO: Roth conversion.

                // Invest any money we have left over. Or, sell more to make up expenses.
                self.income.taxes =
                    new_inflation * tax((self.income.rmd + self.income.id) / new_inflation, 0.0);
                let money_left = self.income.rmd - self.income.taxes - real_expenses;
                self.income.after_tax_bought = money_left.max(0.0);
                if money_left >= 0.0 {
                    // We have money left over. Invest it in our after-tax account.
                    self.after_tax.invest_with_goal_allocation(
                        self.income.after_tax_bought,
                        goal_allocations(
                            &ValueByAccount {
                                pre_tax: self.pre_tax.value(),
                                roth: self.roth.value(),
                                after_tax: self.after_tax.value() + self.income.after_tax_bought,
                            },
                            b,
                        )
                        .after_tax,
                    );
                } else {
                    let mut raw_guess = -money_left;
                    let mut guess = self.allocate_withdrawals(raw_guess);
                    self.income.taxes = new_inflation
                        * tax(
                            (self.income.rmd + self.income.id + guess.pre_tax) / new_inflation,
                            guess.after_tax * self.after_tax.capital_gains_fraction(),
                        );
                    while raw_guess < self.value()
                        && guess.after_tax + guess.pre_tax + guess.roth + self.income.rmd
                            - self.income.taxes
                            < real_expenses
                    {
                        raw_guess += 1000.0;
                        guess = self.allocate_withdrawals(raw_guess);
                        self.income.taxes = new_inflation
                            * tax(
                                (self.income.rmd + self.income.id + guess.pre_tax) / new_inflation,
                                guess.after_tax * self.after_tax.capital_gains_fraction()
                                    / new_inflation,
                            );
                    }
                    let foo = self.withdraw(raw_guess, b);
                    self.income.ira_sold = foo.0;
                    self.income.cg = foo.1;
                    self.income.after_tax_sold = guess.after_tax;
                    self.income.roth_sold = guess.roth;
                    self.income.taxes = new_inflation
                        * tax(
                            (self.income.rmd + self.income.id + self.income.ira_sold)
                                / new_inflation,
                            self.income.cg / new_inflation,
                        );
                }

                // Rebalance tax-advantaged accounts.
                let target_allocations = self.goal_allocations(b);
                self.pre_tax.rebalance(target_allocations.pre_tax);
                self.roth.rebalance(target_allocations.roth);

                self.inflation *= 1.0 + r.inflation;
            }
        }
    }
}

// Calculates what our asset allocation ought to be in each account, so that our overall
// bond fraction is b.
fn goal_allocations(v: &ValueByAccount, b: f64) -> ValueByAccount {
    assert!(b >= 0.0);
    assert!(b <= 1.0);
    assert!(v.pre_tax >= 0.0);
    assert!(v.roth >= 0.0);
    assert!(v.after_tax >= 0.0);
    let total = v.after_tax + v.pre_tax + v.roth;
    let bond_goal = total * b;
    if bond_goal <= v.pre_tax {
        ValueByAccount {
            pre_tax: if v.pre_tax > 0.0 {
                bond_goal / v.pre_tax
            } else {
                0.0
            },
            roth: 0.0,
            after_tax: 0.0,
        }
    } else if bond_goal <= v.pre_tax + v.after_tax {
        ValueByAccount {
            pre_tax: 1.0,
            roth: 0.0,
            after_tax: if v.after_tax > 0.0 {
                (bond_goal - v.pre_tax) / v.after_tax
            } else {
                0.0
            },
        }
    } else {
        ValueByAccount {
            pre_tax: 1.0,
            roth: if v.roth > 0.0 {
                (bond_goal - v.pre_tax - v.after_tax) / v.roth
            } else {
                0.0
            },
            after_tax: 1.0,
        }
    }
}

// Calculates how to withdraw a total of $a from our accounts.
fn allocate_withdrawals(v: &ValueByAccount, a: f64) -> ValueByAccount {
    assert!(a >= 0.0);
    assert!(v.pre_tax >= 0.0);
    assert!(v.roth >= 0.0);
    assert!(v.after_tax >= 0.0);
    if a <= v.after_tax {
        ValueByAccount {
            pre_tax: 0.0,
            roth: 0.0,
            after_tax: a,
        }
    } else if a <= v.after_tax + v.pre_tax {
        ValueByAccount {
            pre_tax: a - v.after_tax,
            roth: 0.0,
            after_tax: v.after_tax,
        }
    } else {
        ValueByAccount {
            pre_tax: v.pre_tax,
            roth: (a - v.after_tax - v.pre_tax).min(v.roth),
            after_tax: v.after_tax,
        }
    }
}

#[cfg(test)]
mod instance_tests {
    use crate::asset::AssetReturn;
    use crate::instance::*;

    #[test]
    fn goal_allocations() {
        let instance = Instance::new(
            0,
            Account::new(50.0, 50.0),
            Account::new(50.0, 50.0),
            Account::new(50.0, 50.0),
            0.0,
        );
        let alloc = instance.goal_allocations(0.0);
        assert_eq!(alloc.pre_tax, 0.0);
        assert_eq!(alloc.roth, 0.0);
        assert_eq!(alloc.after_tax, 0.0);

        let alloc = instance.goal_allocations(1.0);
        assert_eq!(alloc.pre_tax, 1.0);
        assert_eq!(alloc.roth, 1.0);
        assert_eq!(alloc.after_tax, 1.0);

        let alloc = instance.goal_allocations(0.5);
        assert_eq!(alloc.pre_tax, 1.0);
        assert_eq!(alloc.roth, 0.0);
        assert_eq!(alloc.after_tax, 0.5);
    }

    #[test]
    fn grow_and_reinvest() {
        let mut instance = Instance::new(
            0,
            Account::new(50.0, 50.0),
            Account::new(100.0, 0.0),
            Account::new(90.0, 10.0),
            0.0,
        );
        let id = instance.grow_and_reinvest(
            &HistoricalYear {
                year: 0,
                stocks: AssetReturn { cg: 0.08, id: 0.02 },
                tbonds: AssetReturn { cg: 0.0, id: 0.04 },
                aaabonds: AssetReturn { cg: 0.0, id: 0.0 },
                inflation: 0.0,
            },
            0.0,
        );
        assert_eq!(id, 2.2);
        assert_eq!(
            instance.value_by_account(),
            ValueByAccount {
                pre_tax: 107.0,
                roth: 110.0,
                after_tax: 109.4
            }
        );
    }

    #[test]
    fn contribute() {
        let mut instance = Instance::new(
            0,
            Account::new(0.0, 0.0),
            Account::new(0.0, 0.0),
            Account::new(0.0, 0.0),
            0.0,
        );
        instance.contribute(&YearlyContribution {
            pre_tax: 10000.0,
            roth: 5000.0,
            after_tax: 20000.0,
            target_bond_percent: 20.0,
        });
        assert_eq!(
            instance.value_by_account(),
            ValueByAccount {
                pre_tax: 10000.0,
                roth: 5000.0,
                after_tax: 20000.0
            }
        );
        assert_eq!(instance.bond_fraction(), 0.2);
        assert_eq!(
            instance.bond_fraction_by_account(),
            ValueByAccount {
                pre_tax: 0.7,
                roth: 0.0,
                after_tax: 0.0
            }
        )
    }
}
