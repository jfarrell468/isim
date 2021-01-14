use std::fmt::Debug;

#[derive(Debug)]
pub struct Inflation {
    year: i32,
    pub rate: f64,
}

#[macro_export]
macro_rules! inflation {
    ($e:expr) => {
        $crate::histret::inflation::DATA[$e - 1928].rate / 100.0
    };
}

// From http://pages.stern.nyu.edu/~adamodar/New_Home_Page/datafile/histretSPX.html
#[rustfmt::skip]
pub const DATA: [Inflation; 93] = [
    Inflation { year: 1928, rate: -1.152 },
    Inflation { year: 1929, rate: 0.000 },
    Inflation { year: 1930, rate: -2.671 },
    Inflation { year: 1931, rate: -8.932 },
    Inflation { year: 1932, rate: -10.301 },
    Inflation { year: 1933, rate: -5.192 },
    Inflation { year: 1934, rate: 3.479 },
    Inflation { year: 1935, rate: 2.553 },
    Inflation { year: 1936, rate: 1.032 },
    Inflation { year: 1937, rate: 3.726 },
    Inflation { year: 1938, rate: -2.028 },
    Inflation { year: 1939, rate: -1.301 },
    Inflation { year: 1940, rate: 0.719 },
    Inflation { year: 1941, rate: 5.116 },
    Inflation { year: 1942, rate: 10.922 },
    Inflation { year: 1943, rate: 5.969 },
    Inflation { year: 1944, rate: 1.637 },
    Inflation { year: 1945, rate: 2.274 },
    Inflation { year: 1946, rate: 8.476 },
    Inflation { year: 1947, rate: 14.389 },
    Inflation { year: 1948, rate: 7.689 },
    Inflation { year: 1949, rate: -0.971 },
    Inflation { year: 1950, rate: 1.085 },
    Inflation { year: 1951, rate: 7.860 },
    Inflation { year: 1952, rate: 2.279 },
    Inflation { year: 1953, rate: 0.816 },
    Inflation { year: 1954, rate: 0.311 },
    Inflation { year: 1955, rate: -0.279 },
    Inflation { year: 1956, rate: 1.525 },
    Inflation { year: 1957, rate: 3.342 },
    Inflation { year: 1958, rate: 2.729 },
    Inflation { year: 1959, rate: 1.011 },
    Inflation { year: 1960, rate: 1.458 },
    Inflation { year: 1961, rate: 1.071 },
    Inflation { year: 1962, rate: 1.199 },
    Inflation { year: 1963, rate: 1.240 },
    Inflation { year: 1964, rate: 1.279 },
    Inflation { year: 1965, rate: 1.585 },
    Inflation { year: 1966, rate: 3.015 },
    Inflation { year: 1967, rate: 2.773 },
    Inflation { year: 1968, rate: 4.272 },
    Inflation { year: 1969, rate: 5.462 },
    Inflation { year: 1970, rate: 5.838 },
    Inflation { year: 1971, rate: 4.293 },
    Inflation { year: 1972, rate: 3.272 },
    Inflation { year: 1973, rate: 6.178 },
    Inflation { year: 1974, rate: 11.055 },
    Inflation { year: 1975, rate: 9.143 },
    Inflation { year: 1976, rate: 5.745 },
    Inflation { year: 1977, rate: 6.502 },
    Inflation { year: 1978, rate: 7.631 },
    Inflation { year: 1979, rate: 11.254 },
    Inflation { year: 1980, rate: 13.549 },
    Inflation { year: 1981, rate: 10.335 },
    Inflation { year: 1982, rate: 6.131 },
    Inflation { year: 1983, rate: 3.212 },
    Inflation { year: 1984, rate: 4.301 },
    Inflation { year: 1985, rate: 3.546 },
    Inflation { year: 1986, rate: 1.898 },
    Inflation { year: 1987, rate: 3.665 },
    Inflation { year: 1988, rate: 4.078 },
    Inflation { year: 1989, rate: 4.827 },
    Inflation { year: 1990, rate: 5.398 },
    Inflation { year: 1991, rate: 4.235 },
    Inflation { year: 1992, rate: 3.029 },
    Inflation { year: 1993, rate: 2.952 },
    Inflation { year: 1994, rate: 2.607 },
    Inflation { year: 1995, rate: 2.805 },
    Inflation { year: 1996, rate: 2.931 },
    Inflation { year: 1997, rate: 2.338 },
    Inflation { year: 1998, rate: 1.552 },
    Inflation { year: 1999, rate: 2.188 },
    Inflation { year: 2000, rate: 3.377 },
    Inflation { year: 2001, rate: 2.826 },
    Inflation { year: 2002, rate: 1.586 },
    Inflation { year: 2003, rate: 2.270 },
    Inflation { year: 2004, rate: 2.677 },
    Inflation { year: 2005, rate: 3.393 },
    Inflation { year: 2006, rate: 3.226 },
    Inflation { year: 2007, rate: 2.853 },
    Inflation { year: 2008, rate: 3.839 },
    Inflation { year: 2009, rate: -0.356 },
    Inflation { year: 2010, rate: 1.640 },
    Inflation { year: 2011, rate: 3.157 },
    Inflation { year: 2012, rate: 2.069 },
    Inflation { year: 2013, rate: 1.465 },
    Inflation { year: 2014, rate: 1.622 },
    Inflation { year: 2015, rate: 0.119 },
    Inflation { year: 2016, rate: 1.262 },
    Inflation { year: 2017, rate: 2.130 },
    Inflation { year: 2018, rate: 2.443 },
    Inflation { year: 2019, rate: 2.290 },
    Inflation { year: 2020, rate: 1.20 },
];
