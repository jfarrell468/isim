use std::fmt::Debug;

#[derive(Debug)]
pub struct Sp500 {
    year: i32,
    pub price: f64,
    pub dividend: f64,
}

#[macro_export]
macro_rules! sp500 {
    ($e:expr) => {
        AssetReturn {
            cg: 100.0
                * ($crate::histret::sp500::DATA[$e - 1927].price
                    / crate::histret::sp500::DATA[$e - 1928].price
                    - 1.0),
            id: 100.0
                * (crate::histret::sp500::DATA[$e - 1927].dividend
                    / crate::histret::sp500::DATA[$e - 1928].price),
        }
    };
}

#[rustfmt::skip]
pub const DATA: [Sp500; 93] = [
    Sp500 { year: 1927, price: 17.66, dividend: 17.66 * 0.035 },
    Sp500 { year: 1928, price: 24.35, dividend: 24.35 * 0.043 },
    Sp500 { year: 1929, price: 21.45, dividend: 21.45 * 0.041 },
    Sp500 { year: 1930, price: 15.34, dividend: 15.34 * 0.047 },
    Sp500 { year: 1931, price: 8.12, dividend: 8.12 * 0.061 },
    Sp500 { year: 1932, price: 6.92, dividend: 6.92 * 0.072 },
    Sp500 { year: 1933, price: 9.97, dividend: 9.97 * 0.041 },
    Sp500 { year: 1934, price: 9.5, dividend: 9.5 * 0.037 },
    Sp500 { year: 1935, price: 13.43, dividend: 13.43 * 0.038 },
    Sp500 { year: 1936, price: 17.18, dividend: 0.54 },
    Sp500 { year: 1937, price: 10.55, dividend: 10.55 * 0.053 },
    Sp500 { year: 1938, price: 13.14, dividend: 13.14 * 0.038 },
    Sp500 { year: 1939, price: 12.46, dividend: 12.46 * 0.043 },
    Sp500 { year: 1940, price: 10.58, dividend: 10.58 * 0.052 },
    Sp500 { year: 1941, price: 8.69, dividend: 8.69 * 0.062 },
    Sp500 { year: 1942, price: 9.77, dividend: 9.77 * 0.06 },
    Sp500 { year: 1943, price: 11.67, dividend: 11.67 * 0.047 },
    Sp500 { year: 1944, price: 13.28, dividend: 13.28 * 0.046 },
    Sp500 { year: 1945, price: 17.36, dividend: 17.36 * 0.039 },
    Sp500 { year: 1946, price: 15.3, dividend: 15.3 * 0.039 },
    Sp500 { year: 1947, price: 15.3, dividend: 15.3 * 0.052 },
    Sp500 { year: 1948, price: 15.2, dividend: 15.2 * 0.064 },
    Sp500 { year: 1949, price: 16.79, dividend: 16.79 * 0.071 },
    Sp500 { year: 1950, price: 20.43, dividend: 20.43 * 0.075 },
    Sp500 { year: 1951, price: 23.77, dividend: 23.77 * 0.063 },
    Sp500 { year: 1952, price: 26.57, dividend: 26.57 * 0.057 },
    Sp500 { year: 1953, price: 24.81, dividend: 24.81 * 0.058 },
    Sp500 { year: 1954, price: 35.98, dividend: 35.98 * 0.052 },
    Sp500 { year: 1955, price: 45.48, dividend: 45.48 * 0.049 },
    Sp500 { year: 1956, price: 46.67, dividend: 46.67 * 0.047 },
    Sp500 { year: 1957, price: 39.99, dividend: 39.99 * 0.045 },
    Sp500 { year: 1958, price: 55.21, dividend: 55.21 * 0.041 },
    Sp500 { year: 1959, price: 59.89, dividend: 59.89 * 0.033 },
    Sp500 { year: 1960, price: 58.11, dividend: 1.981551 },
    Sp500 { year: 1961, price: 71.55, dividend: 2.039175 },
    Sp500 { year: 1962, price: 63.1, dividend: 2.1454 },
    Sp500 { year: 1963, price: 75.02, dividend: 2.348126 },
    Sp500 { year: 1964, price: 84.75, dividend: 2.584875 },
    Sp500 { year: 1965, price: 92.43, dividend: 2.828358 },
    Sp500 { year: 1966, price: 80.33, dividend: 2.883847 },
    Sp500 { year: 1967, price: 96.47, dividend: 2.980923 },
    Sp500 { year: 1968, price: 103.86, dividend: 3.043098 },
    Sp500 { year: 1969, price: 92.06, dividend: 3.240512 },
    Sp500 { year: 1970, price: 92.15, dividend: 3.18839 },
    Sp500 { year: 1971, price: 102.09, dividend: 3.16479 },
    Sp500 { year: 1972, price: 118.05, dividend: 3.18735 },
    Sp500 { year: 1973, price: 97.55, dividend: 3.60935 },
    Sp500 { year: 1974, price: 68.56, dividend: 3.722808 },
    Sp500 { year: 1975, price: 90.19, dividend: 3.733866 },
    Sp500 { year: 1976, price: 107.46, dividend: 4.223178 },
    Sp500 { year: 1977, price: 95.1, dividend: 4.85961 },
    Sp500 { year: 1978, price: 96.11, dividend: 5.180329 },
    Sp500 { year: 1979, price: 107.94, dividend: 5.969082 },
    Sp500 { year: 1980, price: 135.76, dividend: 6.435024 },
    Sp500 { year: 1981, price: 122.55, dividend: 6.826035 },
    Sp500 { year: 1982, price: 140.64, dividend: 6.933552 },
    Sp500 { year: 1983, price: 164.93, dividend: 7.124976 },
    Sp500 { year: 1984, price: 167.24, dividend: 7.826832 },
    Sp500 { year: 1985, price: 211.28, dividend: 8.197664 },
    Sp500 { year: 1986, price: 242.17, dividend: 8.185346 },
    Sp500 { year: 1987, price: 247.08, dividend: 9.166668 },
    Sp500 { year: 1988, price: 277.72, dividend: 10.220096 },
    Sp500 { year: 1989, price: 353.4, dividend: 11.73288 },
    Sp500 { year: 1990, price: 330.22, dividend: 12.350228 },
    Sp500 { year: 1991, price: 417.09, dividend: 12.971499 },
    Sp500 { year: 1992, price: 435.71, dividend: 12.63559 },
    Sp500 { year: 1993, price: 466.45, dividend: 12.68744 },
    Sp500 { year: 1994, price: 459.27, dividend: 13.364757 },
    Sp500 { year: 1995, price: 615.93, dividend: 14.16639 },
    Sp500 { year: 1996, price: 740.74, dividend: 14.888874 },
    Sp500 { year: 1997, price: 970.43, dividend: 15.522 },
    Sp500 { year: 1998, price: 1229.23, dividend: 16.2 },
    Sp500 { year: 1999, price: 1469.25, dividend: 16.709 },
    Sp500 { year: 2000, price: 1320.28, dividend: 16.27 },
    Sp500 { year: 2001, price: 1148.09, dividend: 15.74 },
    Sp500 { year: 2002, price: 879.82, dividend: 16.08 },
    Sp500 { year: 2003, price: 1111.91, dividend: 17.39 },
    Sp500 { year: 2004, price: 1211.92, dividend: 19.44 },
    Sp500 { year: 2005, price: 1248.29, dividend: 22.22 },
    Sp500 { year: 2006, price: 1418.3, dividend: 24.88 },
    Sp500 { year: 2007, price: 1468.36, dividend: 27.73 },
    Sp500 { year: 2008, price: 903.25, dividend: 28.39 },
    Sp500 { year: 2009, price: 1115.1, dividend: 22.41 },
    Sp500 { year: 2010, price: 1257.64, dividend: 22.73 },
    Sp500 { year: 2011, price: 1257.6, dividend: 26.43 },
    Sp500 { year: 2012, price: 1426.19, dividend: 31.25 },
    Sp500 { year: 2013, price: 1848.36, dividend: 36.28 },
    Sp500 { year: 2014, price: 2058.9, dividend: 39.44 },
    Sp500 { year: 2015, price: 2043.9, dividend: 43.39 },
    Sp500 { year: 2016, price: 2238.83, dividend: 45.7 },
    Sp500 { year: 2017, price: 2673.61, dividend: 48.93 },
    Sp500 { year: 2018, price: 2506.85, dividend: 53.75 },
    Sp500 { year: 2019, price: 3230.78, dividend: 58.8 },
];

mod sp500_tests {
    #[cfg(test)]
    use crate::asset::*;

    #[test]
    pub fn returns_1928_to_2019() {
        let mut stonks = Asset::new(100.0);
        for i in 1928..=2019 {
            let ret: AssetReturn = sp500!(i);
            let is = stonks.grow(&ret);
            stonks.invest(is);
            println!("{:?}", ret);
            println!(
                "{} {}",
                (100.0 * (ret.cg + ret.id)).round() / 100.0,
                (100.0 * stonks.value).round() / 100.0
            );
        }
        assert_eq!((100.0 * stonks.value).round() / 100.0, 502_417.21);
    }
}
