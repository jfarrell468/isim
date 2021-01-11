use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct Tbond10 {
    year: i32,
    pub rate: f64,
}

#[macro_export]
macro_rules! tbond {
    ($e:expr) => {
        AssetReturn {
            cg: 100.0*(($crate::histret::tbond::DATA[$e-1928].rate + ($crate::histret::tbond::DATA[$e-1927].rate-$crate::histret::tbond::DATA[$e-1928].rate)/((1.0 + $crate::histret::tbond::DATA[$e-1927].rate)*(1.0 + $crate::histret::tbond::DATA[$e-1927].rate)*(1.0 + $crate::histret::tbond::DATA[$e-1927].rate)*(1.0 + $crate::histret::tbond::DATA[$e-1927].rate)*(1.0 + $crate::histret::tbond::DATA[$e-1927].rate)*(1.0 + $crate::histret::tbond::DATA[$e-1927].rate)*(1.0 + $crate::histret::tbond::DATA[$e-1927].rate)*(1.0 + $crate::histret::tbond::DATA[$e-1927].rate)*(1.0 + $crate::histret::tbond::DATA[$e-1927].rate)*(1.0 + $crate::histret::tbond::DATA[$e-1927].rate)))/$crate::histret::tbond::DATA[$e-1927].rate-1.0),
            id: 100.0*$crate::histret::tbond::DATA[$e-1928].rate
        }
    };
}

pub const DATA: [Tbond10; 93] = [
    Tbond10 { year: 1927, rate: 0.0317 },
    Tbond10 { year: 1928, rate: 0.0345 },
    Tbond10 { year: 1929, rate: 0.0336 },
    Tbond10 { year: 1930, rate: 0.0322 },
    Tbond10 { year: 1931, rate: 0.0393 },
    Tbond10 { year: 1932, rate: 0.0335 },
    Tbond10 { year: 1933, rate: 0.0353 },
    Tbond10 { year: 1934, rate: 0.0301 },
    Tbond10 { year: 1935, rate: 0.0284 },
    Tbond10 { year: 1936, rate: 0.0259 },
    Tbond10 { year: 1937, rate: 0.0273 },
    Tbond10 { year: 1938, rate: 0.0256 },
    Tbond10 { year: 1939, rate: 0.0235 },
    Tbond10 { year: 1940, rate: 0.0201 },
    Tbond10 { year: 1941, rate: 0.0247 },
    Tbond10 { year: 1942, rate: 0.0249 },
    Tbond10 { year: 1943, rate: 0.0249 },
    Tbond10 { year: 1944, rate: 0.0248 },
    Tbond10 { year: 1945, rate: 0.0233 },
    Tbond10 { year: 1946, rate: 0.0224 },
    Tbond10 { year: 1947, rate: 0.0239 },
    Tbond10 { year: 1948, rate: 0.0244 },
    Tbond10 { year: 1949, rate: 0.0219 },
    Tbond10 { year: 1950, rate: 0.0239 },
    Tbond10 { year: 1951, rate: 0.027 },
    Tbond10 { year: 1952, rate: 0.0275 },
    Tbond10 { year: 1953, rate: 0.0259 },
    Tbond10 { year: 1954, rate: 0.0251 },
    Tbond10 { year: 1955, rate: 0.0296 },
    Tbond10 { year: 1956, rate: 0.0359 },
    Tbond10 { year: 1957, rate: 0.0321 },
    Tbond10 { year: 1958, rate: 0.0386 },
    Tbond10 { year: 1959, rate: 0.0469 },
    Tbond10 { year: 1960, rate: 0.0384 },
    Tbond10 { year: 1961, rate: 0.0406 },
    Tbond10 { year: 1962, rate: 0.0386 },
    Tbond10 { year: 1963, rate: 0.0413 },
    Tbond10 { year: 1964, rate: 0.0418 },
    Tbond10 { year: 1965, rate: 0.0462 },
    Tbond10 { year: 1966, rate: 0.0484 },
    Tbond10 { year: 1967, rate: 0.057 },
    Tbond10 { year: 1968, rate: 0.0603 },
    Tbond10 { year: 1969, rate: 0.0765 },
    Tbond10 { year: 1970, rate: 0.0639 },
    Tbond10 { year: 1971, rate: 0.0593 },
    Tbond10 { year: 1972, rate: 0.0636 },
    Tbond10 { year: 1973, rate: 0.0674 },
    Tbond10 { year: 1974, rate: 0.0743 },
    Tbond10 { year: 1975, rate: 0.08 },
    Tbond10 { year: 1976, rate: 0.0687 },
    Tbond10 { year: 1977, rate: 0.0769 },
    Tbond10 { year: 1978, rate: 0.0901 },
    Tbond10 { year: 1979, rate: 0.1039 },
    Tbond10 { year: 1980, rate: 0.1284 },
    Tbond10 { year: 1981, rate: 0.1372 },
    Tbond10 { year: 1982, rate: 0.1054 },
    Tbond10 { year: 1983, rate: 0.1183 },
    Tbond10 { year: 1984, rate: 0.115 },
    Tbond10 { year: 1985, rate: 0.0926 },
    Tbond10 { year: 1986, rate: 0.0711 },
    Tbond10 { year: 1987, rate: 0.0899 },
    Tbond10 { year: 1988, rate: 0.0911 },
    Tbond10 { year: 1989, rate: 0.0784 },
    Tbond10 { year: 1990, rate: 0.0808 },
    Tbond10 { year: 1991, rate: 0.0709 },
    Tbond10 { year: 1992, rate: 0.0677 },
    Tbond10 { year: 1993, rate: 0.0577 },
    Tbond10 { year: 1994, rate: 0.0781 },
    Tbond10 { year: 1995, rate: 0.0571 },
    Tbond10 { year: 1996, rate: 0.063 },
    Tbond10 { year: 1997, rate: 0.0581 },
    Tbond10 { year: 1998, rate: 0.0465 },
    Tbond10 { year: 1999, rate: 0.0644 },
    Tbond10 { year: 2000, rate: 0.0511 },
    Tbond10 { year: 2001, rate: 0.0505 },
    Tbond10 { year: 2002, rate: 0.0382 },
    Tbond10 { year: 2003, rate: 0.0425 },
    Tbond10 { year: 2004, rate: 0.0422 },
    Tbond10 { year: 2005, rate: 0.0439 },
    Tbond10 { year: 2006, rate: 0.047 },
    Tbond10 { year: 2007, rate: 0.0402 },
    Tbond10 { year: 2008, rate: 0.0221 },
    Tbond10 { year: 2009, rate: 0.0384 },
    Tbond10 { year: 2010, rate: 0.0329 },
    Tbond10 { year: 2011, rate: 0.0188 },
    Tbond10 { year: 2012, rate: 0.0176 },
    Tbond10 { year: 2013, rate: 0.03036 },
    Tbond10 { year: 2014, rate: 0.0217 },
    Tbond10 { year: 2015, rate: 0.0227 },
    Tbond10 { year: 2016, rate: 0.0245 },
    Tbond10 { year: 2017, rate: 0.0241 },
    Tbond10 { year: 2018, rate: 0.0269 },
    Tbond10 { year: 2019, rate: 0.0192 }
];

mod tbond_tests {
    use crate::asset::*;
    use crate::histret::tbond::*;

    #[test]
    pub fn returns_1928_to_2019() {
        let mut bonds = Asset::new(100.0);
        for i in 1928..=2019 {
            let ret: AssetReturn = tbond!(i);
            let is = bonds.grow(&ret);
            bonds.invest(is);
            println!("{:?}", ret);
            println!("{:?}", AssetReturn {
                id: 100.0*DATA[i-1928].rate,
                cg: 100.0*((DATA[i-1928].rate + (DATA[i-1927].rate-DATA[i-1928].rate)/(1.0 + DATA[i-1927].rate).powi(10))/DATA[i-1927].rate-1.0) });
            println!("{:?}", AssetReturn {
                id: 100.0*DATA[i-1928].rate,
                cg: 100.0*((DATA[i-1928].rate + (DATA[i-1927].rate-DATA[i-1928].rate)/((1.0 + DATA[i-1927].rate)*(1.0 + DATA[i-1927].rate)*(1.0 + DATA[i-1927].rate)*(1.0 + DATA[i-1927].rate)*(1.0 + DATA[i-1927].rate)*(1.0 + DATA[i-1927].rate)*(1.0 + DATA[i-1927].rate)*(1.0 + DATA[i-1927].rate)*(1.0 + DATA[i-1927].rate)*(1.0 + DATA[i-1927].rate)))/DATA[i-1927].rate-1.0) });
            println!("{} {}", (100.0 * (ret.cg+ret.id)).round()/100.0, (100.0 * bonds.value).round()/100.0);
        }
        assert_eq!((100.0 * bonds.value).round()/100.0, 8_012.89);
    }
}