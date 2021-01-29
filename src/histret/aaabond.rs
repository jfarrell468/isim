// Historic returns for Aaa corporate bonds.

use crate::histret::BondRate;

// From http://pages.stern.nyu.edu/~adamodar/New_Home_Page/datafile/histretSPX.html
#[rustfmt::skip]
pub const DATA: [BondRate; 94] = [
    BondRate { year: 1927, rate: 0.0446 },
    BondRate { year: 1928, rate: 0.0461 },
    BondRate { year: 1929, rate: 0.0467 },
    BondRate { year: 1930, rate: 0.0452 },
    BondRate { year: 1931, rate: 0.0532 },
    BondRate { year: 1932, rate: 0.0459 },
    BondRate { year: 1933, rate: 0.045 },
    BondRate { year: 1934, rate: 0.0381 },
    BondRate { year: 1935, rate: 0.0344 },
    BondRate { year: 1936, rate: 0.031 },
    BondRate { year: 1937, rate: 0.0321 },
    BondRate { year: 1938, rate: 0.0308 },
    BondRate { year: 1939, rate: 0.0294 },
    BondRate { year: 1940, rate: 0.0271 },
    BondRate { year: 1941, rate: 0.028 },
    BondRate { year: 1942, rate: 0.0281 },
    BondRate { year: 1943, rate: 0.0274 },
    BondRate { year: 1944, rate: 0.027 },
    BondRate { year: 1945, rate: 0.0261 },
    BondRate { year: 1946, rate: 0.0261 },
    BondRate { year: 1947, rate: 0.0286 },
    BondRate { year: 1948, rate: 0.0279 },
    BondRate { year: 1949, rate: 0.0258 },
    BondRate { year: 1950, rate: 0.0267 },
    BondRate { year: 1951, rate: 0.0301 },
    BondRate { year: 1952, rate: 0.0297 },
    BondRate { year: 1953, rate: 0.0313 },
    BondRate { year: 1954, rate: 0.029 },
    BondRate { year: 1955, rate: 0.0315 },
    BondRate { year: 1956, rate: 0.0375 },
    BondRate { year: 1957, rate: 0.0381 },
    BondRate { year: 1958, rate: 0.0408 },
    BondRate { year: 1959, rate: 0.0458 },
    BondRate { year: 1960, rate: 0.0435 },
    BondRate { year: 1961, rate: 0.0442 },
    BondRate { year: 1962, rate: 0.0424 },
    BondRate { year: 1963, rate: 0.0435 },
    BondRate { year: 1964, rate: 0.0444 },
    BondRate { year: 1965, rate: 0.0468 },
    BondRate { year: 1966, rate: 0.0539 },
    BondRate { year: 1967, rate: 0.0619 },
    BondRate { year: 1968, rate: 0.0645 },
    BondRate { year: 1969, rate: 0.0772 },
    BondRate { year: 1970, rate: 0.0764 },
    BondRate { year: 1971, rate: 0.0725 },
    BondRate { year: 1972, rate: 0.0708 },
    BondRate { year: 1973, rate: 0.0768 },
    BondRate { year: 1974, rate: 0.0889 },
    BondRate { year: 1975, rate: 0.0879 },
    BondRate { year: 1976, rate: 0.0798 },
    BondRate { year: 1977, rate: 0.0819 },
    BondRate { year: 1978, rate: 0.0916 },
    BondRate { year: 1979, rate: 0.1074 },
    BondRate { year: 1980, rate: 0.1321 },
    BondRate { year: 1981, rate: 0.1423 },
    BondRate { year: 1982, rate: 0.1183 },
    BondRate { year: 1983, rate: 0.1257 },
    BondRate { year: 1984, rate: 0.1213 },
    BondRate { year: 1985, rate: 0.1016 },
    BondRate { year: 1986, rate: 0.0849 },
    BondRate { year: 1987, rate: 0.1011 },
    BondRate { year: 1988, rate: 0.0957 },
    BondRate { year: 1989, rate: 0.0886 },
    BondRate { year: 1990, rate: 0.0905 },
    BondRate { year: 1991, rate: 0.0831 },
    BondRate { year: 1992, rate: 0.0798 },
    BondRate { year: 1993, rate: 0.0693 },
    BondRate { year: 1994, rate: 0.0846 },
    BondRate { year: 1995, rate: 0.0682 },
    BondRate { year: 1996, rate: 0.072 },
    BondRate { year: 1997, rate: 0.0676 },
    BondRate { year: 1998, rate: 0.0622 },
    BondRate { year: 1999, rate: 0.0755 },
    BondRate { year: 2000, rate: 0.0721 },
    BondRate { year: 2001, rate: 0.0677 },
    BondRate { year: 2002, rate: 0.0621 },
    BondRate { year: 2003, rate: 0.0562 },
    BondRate { year: 2004, rate: 0.0547 },
    BondRate { year: 2005, rate: 0.0537 },
    BondRate { year: 2006, rate: 0.0532 },
    BondRate { year: 2007, rate: 0.0549 },
    BondRate { year: 2008, rate: 0.0505 },
    BondRate { year: 2009, rate: 0.0526 },
    BondRate { year: 2010, rate: 0.0502 },
    BondRate { year: 2011, rate: 0.0393 },
    BondRate { year: 2012, rate: 0.0365 },
    BondRate { year: 2013, rate: 0.0462 },
    BondRate { year: 2014, rate: 0.0379 },
    BondRate { year: 2015, rate: 0.0397 },
    BondRate { year: 2016, rate: 0.0406 },
    BondRate { year: 2017, rate: 0.0351 },
    BondRate { year: 2018, rate: 0.0402 },
    BondRate { year: 2019, rate: 0.0301 },
    BondRate { year: 2020, rate: 0.0223 },
];

mod aaabond_tests {
    #[cfg(test)]
    use crate::asset::*;
    #[cfg(test)]
    use crate::bond_return_10y;
    #[cfg(test)]
    use crate::histret::aaabond::DATA;

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
        assert_eq!((100.0 * bonds.value).round() / 100.0, 18_064.22);
    }
}
