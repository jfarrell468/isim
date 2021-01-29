// Historic returns for 10-year treasury bonds.

use crate::histret::BondRate;

// From http://pages.stern.nyu.edu/~adamodar/New_Home_Page/datafile/histretSPX.html
#[rustfmt::skip]
pub const DATA: [BondRate; 94] = [
    BondRate { year: 1927, rate: 0.0317 },
    BondRate { year: 1928, rate: 0.0345 },
    BondRate { year: 1929, rate: 0.0336 },
    BondRate { year: 1930, rate: 0.0322 },
    BondRate { year: 1931, rate: 0.0393 },
    BondRate { year: 1932, rate: 0.0335 },
    BondRate { year: 1933, rate: 0.0353 },
    BondRate { year: 1934, rate: 0.0301 },
    BondRate { year: 1935, rate: 0.0284 },
    BondRate { year: 1936, rate: 0.0259 },
    BondRate { year: 1937, rate: 0.0273 },
    BondRate { year: 1938, rate: 0.0256 },
    BondRate { year: 1939, rate: 0.0235 },
    BondRate { year: 1940, rate: 0.0201 },
    BondRate { year: 1941, rate: 0.0247 },
    BondRate { year: 1942, rate: 0.0249 },
    BondRate { year: 1943, rate: 0.0249 },
    BondRate { year: 1944, rate: 0.0248 },
    BondRate { year: 1945, rate: 0.0233 },
    BondRate { year: 1946, rate: 0.0224 },
    BondRate { year: 1947, rate: 0.0239 },
    BondRate { year: 1948, rate: 0.0244 },
    BondRate { year: 1949, rate: 0.0219 },
    BondRate { year: 1950, rate: 0.0239 },
    BondRate { year: 1951, rate: 0.027 },
    BondRate { year: 1952, rate: 0.0275 },
    BondRate { year: 1953, rate: 0.0259 },
    BondRate { year: 1954, rate: 0.0251 },
    BondRate { year: 1955, rate: 0.0296 },
    BondRate { year: 1956, rate: 0.0359 },
    BondRate { year: 1957, rate: 0.0321 },
    BondRate { year: 1958, rate: 0.0386 },
    BondRate { year: 1959, rate: 0.0469 },
    BondRate { year: 1960, rate: 0.0384 },
    BondRate { year: 1961, rate: 0.0406 },
    BondRate { year: 1962, rate: 0.0386 },
    BondRate { year: 1963, rate: 0.0413 },
    BondRate { year: 1964, rate: 0.0418 },
    BondRate { year: 1965, rate: 0.0462 },
    BondRate { year: 1966, rate: 0.0484 },
    BondRate { year: 1967, rate: 0.057 },
    BondRate { year: 1968, rate: 0.0603 },
    BondRate { year: 1969, rate: 0.0765 },
    BondRate { year: 1970, rate: 0.0639 },
    BondRate { year: 1971, rate: 0.0593 },
    BondRate { year: 1972, rate: 0.0636 },
    BondRate { year: 1973, rate: 0.0674 },
    BondRate { year: 1974, rate: 0.0743 },
    BondRate { year: 1975, rate: 0.08 },
    BondRate { year: 1976, rate: 0.0687 },
    BondRate { year: 1977, rate: 0.0769 },
    BondRate { year: 1978, rate: 0.0901 },
    BondRate { year: 1979, rate: 0.1039 },
    BondRate { year: 1980, rate: 0.1284 },
    BondRate { year: 1981, rate: 0.1372 },
    BondRate { year: 1982, rate: 0.1054 },
    BondRate { year: 1983, rate: 0.1183 },
    BondRate { year: 1984, rate: 0.115 },
    BondRate { year: 1985, rate: 0.0926 },
    BondRate { year: 1986, rate: 0.0711 },
    BondRate { year: 1987, rate: 0.0899 },
    BondRate { year: 1988, rate: 0.0911 },
    BondRate { year: 1989, rate: 0.0784 },
    BondRate { year: 1990, rate: 0.0808 },
    BondRate { year: 1991, rate: 0.0709 },
    BondRate { year: 1992, rate: 0.0677 },
    BondRate { year: 1993, rate: 0.0577 },
    BondRate { year: 1994, rate: 0.0781 },
    BondRate { year: 1995, rate: 0.0571 },
    BondRate { year: 1996, rate: 0.063 },
    BondRate { year: 1997, rate: 0.0581 },
    BondRate { year: 1998, rate: 0.0465 },
    BondRate { year: 1999, rate: 0.0644 },
    BondRate { year: 2000, rate: 0.0511 },
    BondRate { year: 2001, rate: 0.0505 },
    BondRate { year: 2002, rate: 0.0382 },
    BondRate { year: 2003, rate: 0.0425 },
    BondRate { year: 2004, rate: 0.0422 },
    BondRate { year: 2005, rate: 0.0439 },
    BondRate { year: 2006, rate: 0.047 },
    BondRate { year: 2007, rate: 0.0402 },
    BondRate { year: 2008, rate: 0.0221 },
    BondRate { year: 2009, rate: 0.0384 },
    BondRate { year: 2010, rate: 0.0329 },
    BondRate { year: 2011, rate: 0.0188 },
    BondRate { year: 2012, rate: 0.0176 },
    BondRate { year: 2013, rate: 0.03036 },
    BondRate { year: 2014, rate: 0.0217 },
    BondRate { year: 2015, rate: 0.0227 },
    BondRate { year: 2016, rate: 0.0245 },
    BondRate { year: 2017, rate: 0.0241 },
    BondRate { year: 2018, rate: 0.0269 },
    BondRate { year: 2019, rate: 0.0192 },
    BondRate { year: 2020, rate: 0.0093 },
];

mod tbond_tests {
    #[cfg(test)]
    use crate::asset::*;
    #[cfg(test)]
    use crate::bond_return_10y;
    #[cfg(test)]
    use crate::histret::tbond::DATA;

    #[test]
    pub fn returns_1928_to_2019() {
        let mut bonds = Asset::new(100.0);
        for i in 1928..=2019 {
            let ret: AssetReturn = bond_return_10y!(i, DATA);
            let is = bonds.grow(&ret, 0.0);
            bonds.invest(is);
            println!("{:?}", ret);
            println!(
                "{} {}",
                (100.0 * (ret.cg + ret.id)).round() / 100.0,
                (100.0 * bonds.value).round() / 100.0
            );
        }
        assert_eq!((100.0 * bonds.value).round() / 100.0, 8_012.89);
    }
}
