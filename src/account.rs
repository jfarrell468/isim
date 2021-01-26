use crate::asset::Asset;
use crate::config::Allocation;
use crate::histret::HistoricalYear;

use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Account {
    pub stocks: Asset,
    pub bonds: Asset,
}

// TODO: pub enum AssetClass { Stocks(Asset), Bonds(Asset), Cash(Asset)

impl Account {
    pub fn new(stocks: f64, bonds: f64) -> Account {
        Account {
            stocks: Asset::new(stocks),
            bonds: Asset::new(bonds),
        }
    }
    pub fn new_with_basis(stocks: f64, stocks_basis: f64, bonds: f64, bonds_basis: f64) -> Account {
        Account {
            stocks: Asset::new_with_basis(stocks, stocks_basis),
            bonds: Asset::new_with_basis(bonds, bonds_basis),
        }
    }
    pub fn from_allocation(a: &Allocation) -> Account {
        assert!(a.value >= 0.0);
        assert!(a.bond_percent >= 0.0);
        assert!(a.bond_percent <= 100.0);
        let bonds = a.value * a.bond_percent / 100.0;
        let stocks = a.value - bonds;
        Account::new(stocks, bonds)
    }
    pub fn from_allocation_and_basis(a: &Allocation, basis: f64) -> Account {
        assert!(a.value >= 0.0);
        assert!(a.bond_percent >= 0.0);
        assert!(a.bond_percent <= 100.0);
        assert!(basis >= 0.0);
        let bonds = a.value * a.bond_percent / 100.0;
        let stocks = a.value - bonds;
        // Allocate all capital gains to stocks.
        assert!(stocks >= basis);
        Account::new_with_basis(stocks, basis, bonds, bonds)
    }
    pub fn grow(&mut self, r: &HistoricalYear, e: f64) -> f64 {
        let mut id: f64 = 0.0;
        id += self.stocks.grow(&r.stocks, e);
        id += self.bonds.grow(&r.bonds, e);
        return id;
    }
    pub fn grow_and_reinvest(&mut self, r: &HistoricalYear, e: f64) {
        let s = self.stocks.grow(&r.stocks, e);
        self.stocks.invest(s);
        let b = self.bonds.grow(&r.bonds, e);
        self.bonds.invest(b);
    }
    pub fn invest(&mut self, s: f64, b: f64) {
        self.stocks.invest(s);
        self.bonds.invest(b);
    }
    pub fn invest_allocation(&mut self, a: &Allocation) {
        let bonds = a.value * a.bond_percent / 100.0;
        self.invest(a.value - bonds, bonds);
    }
    pub fn capital_gains(&self) -> f64 {
        self.stocks.capital_gains() + self.bonds.capital_gains()
    }
    pub fn value(&self) -> f64 {
        self.stocks.value + self.bonds.value
    }
    pub fn bond_fraction(&self) -> f64 {
        self.bonds.value / self.value()
    }
    // Sell $a in stocks, preserving the same fraction of capital gains.
    pub fn sell_stocks(&mut self, a: f64) {
        self.stocks.sell_preserving_cg_ratio(a);
    }
}
