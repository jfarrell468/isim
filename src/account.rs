use crate::asset::{Asset, AssetReturn};
use crate::config::Allocation;

use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Account {
    pub stocks: Asset,
    pub bonds: Asset,
}

// TODO: pub enum AssetClass { Stocks(Asset), Bonds(Asset), Cash(Asset)

impl Account {
    pub fn new(stocks: f64, bonds: f64) -> Account {
        Account::new_with_basis(stocks, 0.0, bonds, 0.0)
    }
    pub fn new_with_basis(stocks: f64, stocks_basis: f64, bonds: f64, bonds_basis: f64) -> Account {
        assert!(stocks >= 0.0);
        assert!(stocks_basis >= 0.0);
        assert!(bonds >= 0.0);
        assert!(bonds_basis >= 0.0);
        Account {
            stocks: Asset::new_with_basis(stocks, stocks_basis),
            bonds: Asset::new_with_basis(bonds, bonds_basis),
        }
    }
    pub fn from_allocation(a: &Allocation) -> Account {
        Account::from_allocation_and_basis(a, 0.0)
    }
    pub fn from_allocation_and_basis(a: &Allocation, basis: f64) -> Account {
        assert!(a.value >= 0.0);
        assert!(a.bond_percent >= 0.0);
        assert!(a.bond_percent <= 100.0);
        assert!(basis >= 0.0);
        let bonds = a.value * a.bond_percent / 100.0;
        let stocks = a.value - bonds;
        // Allocate all capital gains to stocks.
        Account::new_with_basis(stocks, basis, bonds, bonds)
    }

    // Accessors.

    pub fn value(&self) -> f64 {
        self.stocks.value + self.bonds.value
    }
    pub fn bond_fraction(&self) -> f64 {
        if self.value() > 0.0 {
            self.bonds.value / self.value()
        } else {
            0.0
        }
    }
    pub fn capital_gains(&self) -> f64 {
        self.stocks.capital_gains() + self.bonds.capital_gains()
    }
    pub fn capital_gains_fraction(&self) -> f64 {
        if self.value() > 0.0 {
            self.capital_gains() / self.value()
        } else {
            0.0
        }
    }

    // Market growth methods. All return interest and dividends.

    pub fn grow(&mut self, s: &AssetReturn, b: &AssetReturn, e: f64) -> f64 {
        let mut id: f64 = 0.0;
        id += self.stocks.grow(s, e);
        id += self.bonds.grow(b, e);
        id
    }
    pub fn grow_and_reinvest(&mut self, s: &AssetReturn, b: &AssetReturn, e: f64) -> f64 {
        let sid = self.stocks.grow(s, e);
        self.stocks.invest(sid);
        let bid = self.bonds.grow(b, e);
        self.bonds.invest(bid);
        sid + bid
    }

    // Methods for investing new funds.

    // Invest $s in stocks and $b in bonds.
    pub fn invest(&mut self, s: f64, b: f64) {
        self.stocks.invest(s);
        self.bonds.invest(b);
    }
    // Invest $a, preserving the current asset allocation between stocks and bonds.
    pub fn invest_preserving_allocation(&mut self, a: f64) {
        self.invest_allocation(a, self.bond_fraction());
    }
    // Invest $a, with a fraction b going to bonds, and the rest to stocks.
    pub fn invest_allocation(&mut self, a: f64, b: f64) {
        assert!(a >= 0.0);
        assert!(b >= 0.0);
        assert!(b <= 1.0);
        let bonds = a * b;
        self.invest(a - bonds, bonds);
    }
    // Invest $a, and try to make the resulting bond fraction b. But, do not rebalance,
    // so the desired bond fraction may not be achievable.
    pub fn invest_with_goal_allocation(&mut self, a: f64, b: f64) {
        assert!(a >= 0.0);
        assert!(b >= 0.0);
        assert!(b <= 1.0);
        let bond_goal = (self.value() + a) * b;
        let stock_goal = self.value() + a - bond_goal;
        self.invest(
            (stock_goal - self.stocks.value).min(a).max(0.0),
            (bond_goal - self.bonds.value).min(a).max(0.0),
        );
    }

    // Methods for selling assets. Returns realized capital gains.

    // Sell stocks and bonds, preserving the same overall asset allocation in the account.
    pub fn sell_preserving_allocation(&mut self, a: f64) -> f64 {
        assert!(a >= 0.0);
        assert!(a <= self.value());
        let bond_sales = a * self.bond_fraction();
        self.bonds.sell_preserving_cg_ratio(bond_sales)
            + self.stocks.sell_preserving_cg_ratio(a - bond_sales)
    }
    // Sell $a in stocks, preserving the same fraction of capital gains. If not enough stocks,
    // sell some bonds as well.
    pub fn sell_stocks_first(&mut self, a: f64) -> f64 {
        assert!(a >= 0.0);
        assert!(a <= self.value());
        let stock_sales = self.stocks.value.min(a);
        self.stocks.sell_preserving_cg_ratio(stock_sales)
            + self.bonds.sell_preserving_cg_ratio(a - stock_sales)
    }
    pub fn sell_bonds_first(&mut self, a: f64) -> f64 {
        assert!(a >= 0.0);
        assert!(a <= self.value());
        let bond_sales = self.bonds.value.min(a);
        self.bonds.sell_preserving_cg_ratio(bond_sales)
            + self.stocks.sell_preserving_cg_ratio(a - bond_sales)
    }
    // Sell $a, and try to achieve an overall bond fraction of b. But, do not rebalance,
    // so the desired bond fraction may not be achievable.
    pub fn sell_with_goal_allocation(&mut self, a: f64, b: f64) -> f64 {
        assert!(a >= 0.0);
        assert!(
            a <= self.value(),
            "Trying to sell {} but only have {}",
            a,
            self.value()
        );
        assert!(b >= 0.0, "bond fraction should be >= 0 but was {}", b);
        assert!(b <= 1.0, "bond fraction should be <= 1 but was {}", b);
        let bond_goal = (self.value() - a) * b;
        let stock_goal = self.value() - a - bond_goal;
        self.stocks
            .sell_preserving_cg_ratio((self.stocks.value - stock_goal).min(a).max(0.0))
            + self
                .bonds
                .sell_preserving_cg_ratio((self.bonds.value - bond_goal).min(a).max(0.0))
    }

    // Rebalance to a bond fraction of b. Returns realized capital gains.
    pub fn rebalance(&mut self, b: f64) -> f64 {
        let bond_target = self.value() * b;
        if bond_target > self.bonds.value {
            let buy_bonds = (bond_target - self.bonds.value).min(self.stocks.value);
            assert!(buy_bonds >= 0.0);
            self.invest(0.0, buy_bonds);
            self.stocks.sell_preserving_cg_ratio(buy_bonds)
        } else {
            let buy_stocks = (self.bonds.value - bond_target).min(self.bonds.value);
            assert!(buy_stocks >= 0.0);
            self.invest(buy_stocks, 0.0);
            self.bonds.sell_preserving_cg_ratio(buy_stocks)
        }
    }
}

#[cfg(test)]
mod account_tests {
    use crate::account::*;
    #[cfg(test)]
    use crate::assert_eq_cents;
    #[cfg(test)]
    use crate::asset::AssetReturn;

    #[test]
    fn new() {
        let account = Account::new(3.0, 4.0);
        assert_eq!(account.stocks.value, 3.0);
        assert_eq!(account.bonds.value, 4.0);
        assert_eq!(account.bond_fraction(), 4.0 / 7.0);
    }

    #[test]
    fn new_with_basis() {
        let account = Account::new_with_basis(3.0, 1.0, 4.0, 1.0);
        assert_eq!(account.stocks.value, 3.0);
        assert_eq!(account.stocks.capital_gains(), 2.0);
        assert_eq!(account.bonds.value, 4.0);
        assert_eq!(account.bonds.capital_gains(), 3.0);
        assert_eq!(account.bond_fraction(), 4.0 / 7.0);
    }

    #[test]
    fn from_allocation() {
        let account = Account::from_allocation(&Allocation {
            value: 10.0,
            bond_percent: 20.0,
        });
        assert_eq!(account.stocks.value, 8.0);
        assert_eq!(account.bonds.value, 2.0);
    }

    #[test]
    fn from_allocation_and_basis() {
        let account = Account::from_allocation_and_basis(
            &Allocation {
                value: 10.0,
                bond_percent: 20.0,
            },
            4.0,
        );
        assert_eq!(account.stocks.value, 8.0);
        assert_eq!(account.stocks.capital_gains(), 4.0);
        assert_eq!(account.bonds.value, 2.0);
        assert_eq!(account.bonds.capital_gains(), 0.0);
    }

    #[test]
    fn grow() {
        let mut account = Account::new_with_basis(100.0, 100.0, 100.0, 100.0);
        let id = account.grow(
            &AssetReturn { cg: 0.01, id: 0.02 },
            &AssetReturn { cg: 0.03, id: 0.04 },
            0.0,
        );
        assert_eq!(id, 6.0);
        assert_eq!(account.value(), 204.0);
        assert_eq!(account.capital_gains(), 4.0);
    }

    #[test]
    fn grow_expense_ratio() {
        let mut account = Account::new_with_basis(100.0, 100.0, 100.0, 100.0);
        let id = account.grow(
            &AssetReturn { cg: 0.01, id: 0.02 },
            &AssetReturn { cg: 0.03, id: 0.04 },
            0.01,
        );
        assert_eq!(id, 6.0);
        assert_eq_cents!(account.value(), 204.0 * 0.99);
        assert_eq_cents!(account.capital_gains(), 204.0 * 0.99 - 200.0);
    }

    #[test]
    fn grow_and_reinvest() {
        let mut account = Account::new_with_basis(100.0, 100.0, 100.0, 100.0);
        let id = account.grow_and_reinvest(
            &AssetReturn { cg: 0.01, id: 0.02 },
            &AssetReturn { cg: 0.03, id: 0.04 },
            0.0,
        );
        assert_eq!(id, 6.0);
        assert_eq!(account.value(), 210.0);
        assert_eq!(account.capital_gains(), 4.0);
    }

    #[test]
    fn grow_and_reinvest_expense_ratio() {
        let mut account = Account::new_with_basis(100.0, 100.0, 100.0, 100.0);
        let id = account.grow_and_reinvest(
            &AssetReturn { cg: 0.01, id: 0.02 },
            &AssetReturn { cg: 0.03, id: 0.04 },
            0.01,
        );
        assert_eq!(id, 6.0);
        assert_eq_cents!(account.value(), 204.0 * 0.99 + 6.0);
        assert_eq_cents!(account.capital_gains(), 204.0 * 0.99 - 200.0);
    }

    #[test]
    fn invest() {
        let mut account = Account::new_with_basis(3.0, 1.0, 4.0, 1.0);
        let cg = account.capital_gains();
        account.invest(1.0, 2.0);
        assert_eq!(account.value(), 10.0);
        assert_eq!(account.capital_gains(), cg);
    }

    #[test]
    fn invest_preserving_allocation() {
        let mut account = Account::new_with_basis(3.0, 1.0, 4.0, 1.0);
        let cg = account.capital_gains();
        let bf = account.bond_fraction();
        account.invest_preserving_allocation(7.0);
        assert_eq!(account.value(), 14.0);
        assert_eq!(account.capital_gains(), cg);
        assert_eq!(account.bond_fraction(), bf);
    }

    #[test]
    fn invest_allocation() {
        let mut account = Account::new_with_basis(100.0, 17.0, 100.0, 23.0);
        let cg = account.capital_gains();
        account.invest_allocation(5.0, 0.2);
        assert_eq!(account.value(), 205.0);
        assert_eq!(account.stocks.value, 104.0);
        assert_eq!(account.bonds.value, 101.0);
        assert_eq!(account.capital_gains(), cg);
    }

    #[test]
    fn sell_preserving_allocation() {
        let mut account = Account::new_with_basis(80.0, 40.0, 20.0, 20.0);
        let bf = account.bond_fraction();
        assert_eq!(bf, 0.20);
        let initial_cg = account.capital_gains();
        assert_eq!(initial_cg, 40.0);
        let realized_cg = account.sell_preserving_allocation(10.0);
        assert_eq!(realized_cg, 4.0);
        assert_eq!(account.bond_fraction(), bf);
        assert_eq!(account.value(), 90.0);
        assert_eq!(account.stocks.value, 72.0);
        assert_eq!(account.bonds.value, 18.0);
        assert_eq!(account.capital_gains(), initial_cg - realized_cg);
    }

    #[test]
    fn sell_stocks_first() {
        let mut account = Account::new(100.0, 100.0);
        account.sell_stocks_first(50.0);
        assert_eq!(account.stocks.value, 50.0);
        assert_eq!(account.bonds.value, 100.0);
        account.sell_stocks_first(100.0);
        assert_eq!(account.stocks.value, 0.0);
        assert_eq!(account.bonds.value, 50.0);
    }

    #[test]
    fn sell_bonds_first() {
        let mut account = Account::new(100.0, 100.0);
        account.sell_bonds_first(50.0);
        assert_eq!(account.stocks.value, 100.0);
        assert_eq!(account.bonds.value, 50.0);
        account.sell_bonds_first(100.0);
        assert_eq!(account.stocks.value, 50.0);
        assert_eq!(account.bonds.value, 0.0);
    }

    #[test]
    fn sell_with_goal_allocation() {
        let mut account = Account::new_with_basis(100.0, 50.0, 100.0, 90.0);
        let cg = account.capital_gains();
        let realized_cg = account.sell_with_goal_allocation(20.0, 0.5);
        assert_eq!(account.value(), 180.0);
        assert_eq!(account.capital_gains(), cg - realized_cg);
        assert_eq!(account.bond_fraction(), 0.5);
        assert_eq!(realized_cg, 6.0);
    }

    #[test]
    fn sell_with_goal_allocation_all_stocks() {
        let mut account = Account::new_with_basis(100.0, 50.0, 100.0, 90.0);
        let cg = account.capital_gains();
        let realized_cg = account.sell_with_goal_allocation(20.0, 1.0);
        assert_eq!(account.value(), 180.0);
        assert_eq!(account.capital_gains(), cg - realized_cg);
        assert_eq!(account.bond_fraction(), 100.0 / 180.0);
        assert_eq!(realized_cg, 10.0);
    }

    #[test]
    fn sell_with_goal_allocation_all_bonds() {
        let mut account = Account::new_with_basis(100.0, 50.0, 100.0, 90.0);
        let cg = account.capital_gains();
        let realized_cg = account.sell_with_goal_allocation(20.0, 0.0);
        assert_eq!(account.value(), 180.0);
        assert_eq!(account.capital_gains(), cg - realized_cg);
        assert_eq!(account.bond_fraction(), 80.0 / 180.0);
        assert_eq!(realized_cg, 2.0);
    }

    #[test]
    fn rebalance_to_stocks() {
        let mut account = Account::new_with_basis(100.0, 50.0, 100.0, 90.0);
        let orig_value = account.value();
        let cg = account.capital_gains();
        let realized_cg = account.rebalance(0.2);
        assert_eq!(account.value(), orig_value);
        assert_eq!(account.capital_gains(), cg - realized_cg);
        assert_eq!(account.bond_fraction(), 0.2);
        assert_eq!(realized_cg, 6.0);
    }

    #[test]
    fn rebalance_to_bonds() {
        let mut account = Account::new_with_basis(100.0, 50.0, 100.0, 90.0);
        let orig_value = account.value();
        let cg = account.capital_gains();
        let realized_cg = account.rebalance(0.8);
        assert_eq!(account.value(), orig_value);
        assert_eq!(account.capital_gains(), cg - realized_cg);
        assert_eq!(account.bond_fraction(), 0.8);
        assert_eq!(realized_cg, 30.0);
    }

    #[test]
    fn invest_with_goal_allocation() {
        let mut account = Account::new(100.0, 100.0);
        account.invest_with_goal_allocation(10.0, 0.5);
        assert_eq!(account.value(), 210.0);
        assert_eq!(account.stocks.value, 105.0);
        assert_eq!(account.bonds.value, 105.0);
    }

    #[test]
    fn invest_with_goal_allocation_all_stocks() {
        let mut account = Account::new(100.0, 100.0);
        account.invest_with_goal_allocation(10.0, 0.2);
        assert_eq!(account.value(), 210.0);
        assert_eq!(account.stocks.value, 110.0);
        assert_eq!(account.bonds.value, 100.0);
    }

    #[test]
    fn invest_with_goal_allocation_all_bonds() {
        let mut account = Account::new(100.0, 100.0);
        account.invest_with_goal_allocation(10.0, 0.8);
        assert_eq!(account.value(), 210.0);
        assert_eq!(account.stocks.value, 100.0);
        assert_eq!(account.bonds.value, 110.0);
    }

    #[test]
    fn capital_gains_ratio() {
        let account = Account::new_with_basis(0.0, 0.0, 0.0, 0.0);
        assert_eq!(account.capital_gains_fraction(), 0.0);

        let account = Account::new_with_basis(10.0, 0.0, 10.0, 10.0);
        assert_eq!(account.capital_gains_fraction(), 0.5);
    }
}
