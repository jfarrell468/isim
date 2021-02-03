use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Asset {
    pub value: f64,
    cost_basis: f64,
}

#[derive(Debug)]
pub struct AssetReturn {
    pub cg: f64, // Capital gains (long-term).
    pub id: f64, // Interest and dividends.
}

impl Asset {
    pub fn new(value: f64) -> Asset {
        Asset::new_with_basis(value, value)
    }
    pub fn new_with_basis(value: f64, cost_basis: f64) -> Asset {
        assert!(value >= 0.0);
        assert!(cost_basis >= 0.0);
        assert!(cost_basis <= value);
        Asset { value, cost_basis }
    }
    pub fn grow(&mut self, r: &AssetReturn, e: f64) -> f64 {
        assert!(r.id >= 0.0);
        let id = self.value * r.id;
        self.value += self.value * r.cg;
        // TODO: How does expense ratio affect capital gains?
        self.value *= 1.0 - e;
        id
    }
    pub fn invest(&mut self, amt: f64) {
        assert!(amt >= 0.0);
        self.value += amt;
        self.cost_basis += amt;
    }
    pub fn capital_gains(&self) -> f64 {
        self.value - self.cost_basis
    }
    pub fn capital_gains_ratio(&self) -> f64 {
        if self.value > 0.0 {
            self.capital_gains() / self.value
        } else {
            0.0
        }
    }
    pub fn sell_preserving_cg_ratio(&mut self, a: f64) -> f64 {
        assert!(a >= 0.0);
        assert!(a <= self.value);
        let cg_ratio = self.capital_gains_ratio();
        self.value -= a;
        self.cost_basis = self.value * (1.0-cg_ratio);
        a * cg_ratio
    }
}

#[cfg(test)]
mod asset_tests {
    use crate::asset::*;

    #[test]
    fn new() {
        let asset = Asset::new(100.0);
        assert_eq!(asset.value, 100.0);
        assert_eq!(asset.cost_basis, 100.0);
        assert_eq!(asset.capital_gains(), 0.0);
    }

    #[test]
    fn new_with_basis() {
        let asset = Asset::new_with_basis(100.0, 50.0);
        assert_eq!(asset.value, 100.0);
        assert_eq!(asset.cost_basis, 50.0);
        assert_eq!(asset.capital_gains(), 50.0);
    }

    #[test]
    fn capital_gains_ratio() {
        let asset = Asset::new_with_basis(5.0, 1.0);
        assert_eq!(asset.capital_gains_ratio(), 0.8);

        let asset = Asset::new_with_basis(0.0, 0.0);
        assert_eq!(asset.capital_gains_ratio(), 0.0);
    }

    #[test]
    fn grow() {
        let mut asset = Asset::new(100.0);
        assert_eq!(asset.grow(&AssetReturn { cg: 0.05, id: 0.01 }, 0.0), 1.0);
        assert_eq!(asset.value, 105.0);
        assert_eq!(asset.cost_basis, 100.0);
        assert_eq!(asset.capital_gains(), 5.0);
    }

    #[test]
    fn grow_with_expense_ratio() {
        let mut asset = Asset::new(100.0);
        assert_eq!(asset.grow(&AssetReturn { cg: 0.1, id: 0.01 }, 0.01), 1.0);
        assert_eq!(asset.value, 108.9);
        assert_eq!(asset.cost_basis, 100.0);
        assert_eq!((asset.capital_gains() * 100.0).round() / 100.0, 8.9);
    }

    #[test]
    fn invest() {
        let mut asset = Asset::new(100.0);
        asset.invest(50.0);
        assert_eq!(asset.value, 150.0);
        assert_eq!(asset.cost_basis, 150.0);
        assert_eq!(asset.capital_gains(), 0.0);
    }

    #[test]
    fn sell_preserving_cg_ratio() {
        let mut asset = Asset::new_with_basis(100.0, 50.0);
        assert_eq!(asset.capital_gains_ratio(), 0.5);
        assert_eq!(asset.sell_preserving_cg_ratio(10.0), 5.0);
        assert_eq!(asset.value, 90.0);
        assert_eq!(asset.capital_gains_ratio(), 0.5);

        let mut asset = Asset::new_with_basis(100.0, 100.0);
        assert_eq!(asset.capital_gains_ratio(), 0.0);
        assert_eq!(asset.sell_preserving_cg_ratio(10.0), 0.0);
        assert_eq!(asset.value, 90.0);
        assert_eq!(asset.capital_gains_ratio(), 0.0);
    }
}
