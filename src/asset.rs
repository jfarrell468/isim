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
        Asset {
            value,
            cost_basis: value,
        }
    }
    pub fn new_with_basis(value: f64, cost_basis: f64) -> Asset {
        Asset { value, cost_basis }
    }
    pub fn grow(&mut self, returns: &AssetReturn) -> f64 {
        let id = self.value * returns.id / 100.0;
        self.value += self.value * returns.cg / 100.0;
        id
    }
    pub fn invest(&mut self, amt: f64) {
        self.value += amt;
        self.cost_basis += amt;
    }
    pub fn capital_gains(&self) -> f64 {
        self.value - self.cost_basis
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
    fn grow() {
        let mut asset = Asset::new(100.0);
        assert_eq!(asset.grow(&AssetReturn { cg: 5.0, id: 1.0 }), 1.0);
        assert_eq!(asset.value, 105.0);
        assert_eq!(asset.cost_basis, 100.0);
        assert_eq!(asset.capital_gains(), 5.0);
    }

    #[test]
    fn invest() {
        let mut asset = Asset::new(100.0);
        asset.invest(50.0);
        assert_eq!(asset.value, 150.0);
        assert_eq!(asset.cost_basis, 150.0);
        assert_eq!(asset.capital_gains(), 0.0);
    }
}
