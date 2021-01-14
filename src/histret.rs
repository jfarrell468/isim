#[macro_use]
mod sp500;
#[macro_use]
mod tbond;
#[macro_use]
mod inflation;

use crate::asset::AssetReturn;
use std::fmt::Debug;

#[derive(Debug)]
pub struct HistoricalYear {
    pub year: i32,
    pub stocks: AssetReturn,
    pub bonds: AssetReturn,
    pub inflation: f64,
}

macro_rules! hy {
    ($e:expr) => {
        HistoricalYear {
            year: $e,
            stocks: sp500!($e),
            bonds: tbond!($e),
            inflation: inflation!($e),
        }
    };
}

#[rustfmt::skip]
pub const RETURNS: [HistoricalYear; 93] = [
    hy!(1928), hy!(1929),
    hy!(1930), hy!(1931), hy!(1932), hy!(1933), hy!(1934),
    hy!(1935), hy!(1936), hy!(1937), hy!(1938), hy!(1939),
    hy!(1940), hy!(1941), hy!(1942), hy!(1943), hy!(1944),
    hy!(1945), hy!(1946), hy!(1947), hy!(1948), hy!(1949),
    hy!(1950), hy!(1951), hy!(1952), hy!(1953), hy!(1954),
    hy!(1955), hy!(1956), hy!(1957), hy!(1958), hy!(1959),
    hy!(1960), hy!(1961), hy!(1962), hy!(1963), hy!(1964),
    hy!(1965), hy!(1966), hy!(1967), hy!(1968), hy!(1969),
    hy!(1970), hy!(1971), hy!(1972), hy!(1973), hy!(1974),
    hy!(1975), hy!(1976), hy!(1977), hy!(1978), hy!(1979),
    hy!(1980), hy!(1981), hy!(1982), hy!(1983), hy!(1984),
    hy!(1985), hy!(1986), hy!(1987), hy!(1988), hy!(1989),
    hy!(1990), hy!(1991), hy!(1992), hy!(1993), hy!(1994),
    hy!(1995), hy!(1996), hy!(1997), hy!(1998), hy!(1999),
    hy!(2000), hy!(2001), hy!(2002), hy!(2003), hy!(2004),
    hy!(2005), hy!(2006), hy!(2007), hy!(2008), hy!(2009),
    hy!(2010), hy!(2011), hy!(2012), hy!(2013), hy!(2014),
    hy!(2015), hy!(2016), hy!(2017), hy!(2018), hy!(2019),
    hy!(2020),
];

mod historical_returns_tests {
    #[cfg(test)]
    use crate::asset::Asset;
    #[cfg(test)]
    use crate::histret::*;

    #[test]
    pub fn real_returns_1928_to_2020() {
        let mut stonks = Asset::new(100.0);
        let mut bonds = Asset::new(100.0);
        for i in 0..93 {
            let is = stonks.grow(&RETURNS[i].stocks, 0.0);
            stonks.invest(is);
            let ib = bonds.grow(&RETURNS[i].bonds, 0.0);
            bonds.invest(ib);
        }
        assert_eq!((100.0 * stonks.value).round() / 100.0, 592_868.15);
        assert_eq!((100.0 * bonds.value).round() / 100.0, 8_920.90);
    }
}
