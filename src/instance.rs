use crate::account::Account;
use crate::config::{PhaseType, YearlyContribution};
use crate::histret::HistoricalYear;
use crate::rmd::rmd_fraction;
use crate::tax::{how_much_to_sell, tax};

use chrono::{Datelike, Utc};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Instance {
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
    pub pre_tax_withdrawal: f64,
    pub stocks_sold: f64,
    pub cg: f64,
    pub taxes: f64,
    pub stocks_bought: f64,
}

#[derive(Debug)]
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
                pre_tax_withdrawal: 0.0,
                stocks_sold: 0.0,
                cg: 0.0,
                taxes: 0.0,
                stocks_bought: 0.0,
            },
        }
    }
    pub fn start(&self) -> usize {
        self.start
    }
    pub fn inflation_adjusted(&self, v: f64) -> f64 {
        v / self.inflation
    }
    pub fn grow_and_reinvest(&mut self, r: &HistoricalYear, e: f64) {
        self.pre_tax.grow_and_reinvest(&r.stocks, &r.tbonds, e);
        self.roth.grow_and_reinvest(&r.stocks, &r.tbonds, e);
        self.income.id = self.after_tax.grow(&r.stocks, &r.tbonds, e);
    }
    pub fn contribute(&mut self, c: &YearlyContribution) {
        self.pre_tax
            .invest_allocation(c.pre_tax.value, c.pre_tax.bond_percent / 100.0);
        self.roth
            .invest_allocation(c.roth.value, c.roth.bond_percent / 100.0);
        self.after_tax
            .invest_allocation(c.after_tax.value, c.after_tax.bond_percent / 100.0);
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
        self.bond_value() / self.value()
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
    pub fn next(&mut self, y: usize, c: &PhaseType, r: &HistoricalYear) {
        self.income = Income {
            id: 0.0,
            pre_tax_withdrawal: 0.0,
            stocks_sold: 0.0,
            cg: 0.0,
            taxes: 0.0,
            stocks_bought: 0.0,
        };
        match c {
            PhaseType::Accumulation(contributions) => {
                // TODO: Configure an overall target bond percent, and use that to determine
                // allocations for each account.
                self.grow_and_reinvest(r, self.expense_ratio);
                self.after_tax.stocks.invest(self.income.id);
                self.contribute(contributions);
                self.inflation *= 1.0 + r.inflation;
            }
            PhaseType::Growth => {
                // TODO: This is wrong. We aren't investing the dividends.
                self.grow_and_reinvest(r, self.expense_ratio);
                self.inflation *= 1.0 + r.inflation;
            }
            PhaseType::SimpleWithdrawAndRebalance(w) => {
                let total_sales = w.amount * self.inflation;
                if total_sales >= self.roth.value() {
                    self.roth.stocks.value = 0.0;
                    self.roth.bonds.value = 0.0;
                } else {
                    self.roth.sell_with_goal_allocation(total_sales, w.bond_percent / 100.0);
                    self.grow_and_reinvest(r, self.expense_ratio);
                    self.roth.rebalance(w.bond_percent / 100.0);
                }
                self.inflation *= 1.0 + r.inflation;
            }
            PhaseType::WithdrawTaxAware(w) => {
                // Market growth. After-tax interest and dividends.
                self.grow_and_reinvest(r, self.expense_ratio);

                self.inflation *= 1.0 + r.inflation;

                // RMDs.
                self.income.pre_tax_withdrawal = self.pre_tax.value()
                    * rmd_fraction(Utc::now().year() - w.birth_year + y as i32);

                // Calculate how much to sell from the after-tax account, if needed to make up
                // expenses.
                // TODO: Also sell from IRA and Roth if needed.
                let cg_ratio = self.after_tax.capital_gains() / self.after_tax.value();
                assert!(
                    !cg_ratio.is_nan(),
                    "cg_ratio = {}, cg = {}, value = {}",
                    cg_ratio,
                    self.after_tax.capital_gains(),
                    self.after_tax.value()
                );
                let sell = how_much_to_sell(w.living_expenses, (self.income.id + self.income.pre_tax_withdrawal) / self.inflation, cg_ratio);
                assert!(sell >= 0.0, "sell = {}, cg_ratio = {}", sell, cg_ratio);
                self.income.stocks_sold = sell * self.inflation;
                self.income.cg = self.income.stocks_sold * cg_ratio;

                // TODO: Roth conversion.

                // Calculate taxes.
                let real_id = (self.income.id + self.income.pre_tax_withdrawal) / self.inflation;
                let real_cg = self.income.cg / self.inflation;
                self.income.taxes = self.inflation * tax(real_id, real_cg);
                assert!(
                    self.income.taxes > 0.0,
                    "taxes = {}, regular income = {}, capital gains = {}",
                    self.income.taxes,
                    real_id,
                    self.income.cg
                );

                // Calculate how much we have left over to invest.
                self.income.stocks_bought =
                    self.income.id + self.income.pre_tax_withdrawal + self.income.stocks_sold
                        - self.income.taxes
                        - w.living_expenses * self.inflation;
                assert!(self.income.stocks_bought >= 0.0, "interest = {}, rmd = {}, pre-tax sales = {}, taxes = {}, living expenses = {}, stocks to buy = {}", self.income.id, self.income.pre_tax_withdrawal, self.income.stocks_sold, self.income.taxes, w.living_expenses * self.inflation, self.income.stocks_bought);

                // Calculate our target asset allocations, and buy/sell accordingly.
                let mut target_values = self.value_by_account();
                target_values.pre_tax -= self.income.pre_tax_withdrawal;
                target_values.after_tax += self.income.stocks_bought - self.income.stocks_sold;
                let target_allocations = goal_allocations(
                    &target_values, w.bond_percent / 100.0
                );
                self.pre_tax.sell_with_goal_allocation(self.income.pre_tax_withdrawal, target_allocations.pre_tax);
                // TODO: The capital gains may not match what we estimated above.
                self.after_tax.sell_with_goal_allocation(self.income.stocks_sold, target_allocations.after_tax);
                self.after_tax.invest_with_goal_allocation(self.income.stocks_bought, target_allocations.after_tax);

                // Rebalance tax-advantaged accounts.
                self.pre_tax.rebalance(target_allocations.pre_tax);
                self.roth.rebalance(target_allocations.roth);
            }
        }
    }
}

fn goal_allocations(v: &ValueByAccount, b: f64) -> ValueByAccount {
    assert!(b >= 0.0);
    assert!(b <= 1.0);
    let total = v.after_tax + v.pre_tax + v.roth;
    let bond_goal = total * b;
    if bond_goal <= v.pre_tax {
        ValueByAccount {
            pre_tax: bond_goal / v.pre_tax,
            roth: 0.0,
            after_tax: 0.0,
        }
    } else if bond_goal <= v.pre_tax + v.after_tax {
        ValueByAccount {
            pre_tax: 1.0,
            roth: 0.0,
            after_tax: (bond_goal - v.pre_tax) / v.after_tax,
        }
    } else {
        ValueByAccount {
            pre_tax: 1.0,
            roth: (bond_goal - v.pre_tax - v.after_tax)
                / v.roth,
            after_tax: 1.0,
        }
    }
}

#[cfg(test)]
mod instance_tests {
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
}
