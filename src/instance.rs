use crate::account::Account;
use crate::config::{AccountType, PhaseType, YearlyContribution};
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
        self.pre_tax.grow_and_reinvest(r, e);
        self.roth.grow_and_reinvest(r, e);
        self.income.id = self.after_tax.grow(r, e);
    }
    pub fn contribute(&mut self, c: &YearlyContribution) {
        self.pre_tax.invest_allocation(&c.pre_tax);
        self.roth.invest_allocation(&c.roth);
        self.after_tax.invest_allocation(&c.after_tax);
    }
    pub fn withdraw_after_tax(&mut self, living_expenses: f64, id: f64) {
        let cg_ratio = self.after_tax.capital_gains() / self.after_tax.value();
        assert!(!cg_ratio.is_nan(), "cg_ratio = {}, cg = {}, value = {}", cg_ratio, self.after_tax.capital_gains(), self.after_tax.value());
        let sell = how_much_to_sell(living_expenses, id / self.inflation, cg_ratio);
        assert!(sell >= 0.0, "sell = {}, cg_ratio = {}", sell, cg_ratio);
        self.income.stocks_sold = sell * self.inflation;
        self.income.cg = self.income.stocks_sold * cg_ratio;
        self.after_tax
            .sell_bonds_first(self.income.stocks_sold);
    }
    pub fn value(&self, a: &AccountType) -> f64 {
        match a {
            AccountType::Total => self.pre_tax.value() + self.roth.value() + self.after_tax.value(),
            AccountType::PreTax => self.pre_tax.value(),
            AccountType::Roth => self.roth.value(),
            AccountType::AfterTax => self.after_tax.value(),
        }
    }
    pub fn bond_fraction(&self, a: &AccountType) -> f64 {
        match a {
            AccountType::Total => self.bond_value() / self.value(a),
            AccountType::PreTax => self.pre_tax.bond_fraction(),
            AccountType::Roth => self.roth.bond_fraction(),
            AccountType::AfterTax => self.after_tax.bond_fraction(),
        }
    }
    pub fn bond_value(&self) -> f64 {
        self.pre_tax.bonds.value + self.roth.bonds.value + self.after_tax.bonds.value
    }
    pub fn capital_gains(&self) -> f64 {
        self.after_tax.capital_gains()
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
                self.grow_and_reinvest(r, self.expense_ratio);
                self.after_tax.stocks.invest(self.income.id);
                self.contribute(contributions);
                self.inflation *= 1.0 + r.inflation;
            }
            PhaseType::Growth => {
                self.grow_and_reinvest(r, self.expense_ratio);
                self.inflation *= 1.0 + r.inflation;
            }
            PhaseType::SimpleWithdrawAndRebalance(w) => {
                let total_sales = w.amount * self.inflation;
                assert!(total_sales >= 0.0);
                let bond_sales = total_sales * (w.bond_percent / 100.0);
                assert!(bond_sales >= 0.0);
                assert!(total_sales >= bond_sales, "total sales = {}, bond sales = {}", total_sales, bond_sales);
                if total_sales >= self.roth.value() {
                    self.roth.stocks.value = 0.0;
                    self.roth.bonds.value = 0.0;
                } else {
                    self.roth.bonds.sell_preserving_cg_ratio(bond_sales);
                    self.roth
                        .stocks
                        .sell_preserving_cg_ratio(total_sales - bond_sales);
                    self.grow_and_reinvest(r, self.expense_ratio);
                    let bond_target = self.roth.value() * w.bond_percent / 100.0;
                    let bond_sales = self.roth.bonds.value - bond_target;
                    if bond_sales > 0.0 {
                        self.roth.bonds.sell_preserving_cg_ratio(bond_sales.min(self.roth.bonds.value));
                        self.roth.stocks.invest(bond_sales);
                    } else {
                        self.roth.stocks.sell_preserving_cg_ratio((-bond_sales).min(self.roth.stocks.value));
                        self.roth.bonds.invest(-bond_sales);
                    }
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
                if self.income.pre_tax_withdrawal > 0.0 {
                    self.pre_tax
                        .sell_bonds_first(self.income.pre_tax_withdrawal);
                }

                // Sell assets in after-tax account, if needed to make up expenses.
                // TODO: Also sell from IRA and Roth if needed.
                self.withdraw_after_tax(
                    w.living_expenses,
                    self.income.id + self.income.pre_tax_withdrawal,
                );

                // TODO: Roth conversion.

                // Calculate taxes.
                let real_id =(self.income.id + self.income.pre_tax_withdrawal) / self.inflation;
                let real_cg = self.income.cg / self.inflation;
                self.income.taxes = self.inflation
                    * tax(
                        real_id,
                        real_cg,
                    );
                assert!(self.income.taxes > 0.0, "taxes = {}, regular income = {}, capital gains = {}", self.income.taxes, real_id, self.income.cg);

                // Invest the leftovers.
                self.income.stocks_bought =
                    self.income.id + self.income.pre_tax_withdrawal + self.income.stocks_sold
                        - self.income.taxes
                        - w.living_expenses * self.inflation;
                assert!(self.income.stocks_bought >= 0.0, "interest = {}, rmd = {}, pre-tax sales = {}, taxes = {}, living expenses = {}, stocks to buy = {}", self.income.id, self.income.pre_tax_withdrawal, self.income.stocks_sold, self.income.taxes, w.living_expenses * self.inflation, self.income.stocks_bought);
                self.after_tax.invest(self.income.stocks_bought, 0.0);
            }
        }
    }
}
