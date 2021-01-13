#[macro_use]
mod sp500;
#[macro_use]
mod tbond;

use crate::asset::AssetReturn;
use std::fmt::Debug;

#[derive(Debug)]
pub struct HistoricalYear {
    pub year: i32,
    pub stocks: AssetReturn,
    pub bonds: AssetReturn,
    pub inflation: f64,
}

// From http://pages.stern.nyu.edu/~adamodar/New_Home_Page/datafile/histretSPX.html
#[rustfmt::skip]
pub const RETURNS: [HistoricalYear; 93] = [
    HistoricalYear { year: 1928, stocks: sp500!(1928), bonds: tbond!(1928), inflation: -1.152  },
    HistoricalYear { year: 1929, stocks: sp500!(1929), bonds: tbond!(1929), inflation: 0.000   },
    HistoricalYear { year: 1930, stocks: sp500!(1930), bonds: tbond!(1930), inflation: -2.671  },
    HistoricalYear { year: 1931, stocks: sp500!(1931), bonds: tbond!(1931), inflation: -8.932  },
    HistoricalYear { year: 1932, stocks: sp500!(1932), bonds: tbond!(1932), inflation: -10.301 },
    HistoricalYear { year: 1933, stocks: sp500!(1933), bonds: tbond!(1933), inflation: -5.192  },
    HistoricalYear { year: 1934, stocks: sp500!(1934), bonds: tbond!(1934), inflation: 3.479   },
    HistoricalYear { year: 1935, stocks: sp500!(1935), bonds: tbond!(1935), inflation: 2.553   },
    HistoricalYear { year: 1936, stocks: sp500!(1936), bonds: tbond!(1936), inflation: 1.032   },
    HistoricalYear { year: 1937, stocks: sp500!(1937), bonds: tbond!(1937), inflation: 3.726   },
    HistoricalYear { year: 1938, stocks: sp500!(1938), bonds: tbond!(1938), inflation: -2.028  },
    HistoricalYear { year: 1939, stocks: sp500!(1939), bonds: tbond!(1939), inflation: -1.301  },
    HistoricalYear { year: 1940, stocks: sp500!(1940), bonds: tbond!(1940), inflation: 0.719   },
    HistoricalYear { year: 1941, stocks: sp500!(1941), bonds: tbond!(1941), inflation: 5.116   },
    HistoricalYear { year: 1942, stocks: sp500!(1942), bonds: tbond!(1942), inflation: 10.922  },
    HistoricalYear { year: 1943, stocks: sp500!(1943), bonds: tbond!(1943), inflation: 5.969   },
    HistoricalYear { year: 1944, stocks: sp500!(1944), bonds: tbond!(1944), inflation: 1.637   },
    HistoricalYear { year: 1945, stocks: sp500!(1945), bonds: tbond!(1945), inflation: 2.274   },
    HistoricalYear { year: 1946, stocks: sp500!(1946), bonds: tbond!(1946), inflation: 8.476   },
    HistoricalYear { year: 1947, stocks: sp500!(1947), bonds: tbond!(1947), inflation: 14.389  },
    HistoricalYear { year: 1948, stocks: sp500!(1948), bonds: tbond!(1948), inflation: 7.689   },
    HistoricalYear { year: 1949, stocks: sp500!(1949), bonds: tbond!(1949), inflation: -0.971  },
    HistoricalYear { year: 1950, stocks: sp500!(1950), bonds: tbond!(1950), inflation: 1.085   },
    HistoricalYear { year: 1951, stocks: sp500!(1951), bonds: tbond!(1951), inflation: 7.860   },
    HistoricalYear { year: 1952, stocks: sp500!(1952), bonds: tbond!(1952), inflation: 2.279   },
    HistoricalYear { year: 1953, stocks: sp500!(1953), bonds: tbond!(1953), inflation: 0.816   },
    HistoricalYear { year: 1954, stocks: sp500!(1954), bonds: tbond!(1954), inflation: 0.311   },
    HistoricalYear { year: 1955, stocks: sp500!(1955), bonds: tbond!(1955), inflation: -0.279  },
    HistoricalYear { year: 1956, stocks: sp500!(1956), bonds: tbond!(1956), inflation: 1.525   },
    HistoricalYear { year: 1957, stocks: sp500!(1957), bonds: tbond!(1957), inflation: 3.342   },
    HistoricalYear { year: 1958, stocks: sp500!(1958), bonds: tbond!(1958), inflation: 2.729   },
    HistoricalYear { year: 1959, stocks: sp500!(1959), bonds: tbond!(1959), inflation: 1.011   },
    HistoricalYear { year: 1960, stocks: sp500!(1960), bonds: tbond!(1960), inflation: 1.458   },
    HistoricalYear { year: 1961, stocks: sp500!(1961), bonds: tbond!(1961), inflation: 1.071   },
    HistoricalYear { year: 1962, stocks: sp500!(1962), bonds: tbond!(1962), inflation: 1.199   },
    HistoricalYear { year: 1963, stocks: sp500!(1963), bonds: tbond!(1963), inflation: 1.240   },
    HistoricalYear { year: 1964, stocks: sp500!(1964), bonds: tbond!(1964), inflation: 1.279   },
    HistoricalYear { year: 1965, stocks: sp500!(1965), bonds: tbond!(1965), inflation: 1.585   },
    HistoricalYear { year: 1966, stocks: sp500!(1966), bonds: tbond!(1966), inflation: 3.015   },
    HistoricalYear { year: 1967, stocks: sp500!(1967), bonds: tbond!(1967), inflation: 2.773   },
    HistoricalYear { year: 1968, stocks: sp500!(1968), bonds: tbond!(1968), inflation: 4.272   },
    HistoricalYear { year: 1969, stocks: sp500!(1969), bonds: tbond!(1969), inflation: 5.462   },
    HistoricalYear { year: 1970, stocks: sp500!(1970), bonds: tbond!(1970), inflation: 5.838   },
    HistoricalYear { year: 1971, stocks: sp500!(1971), bonds: tbond!(1971), inflation: 4.293   },
    HistoricalYear { year: 1972, stocks: sp500!(1972), bonds: tbond!(1972), inflation: 3.272   },
    HistoricalYear { year: 1973, stocks: sp500!(1973), bonds: tbond!(1973), inflation: 6.178   },
    HistoricalYear { year: 1974, stocks: sp500!(1974), bonds: tbond!(1974), inflation: 11.055  },
    HistoricalYear { year: 1975, stocks: sp500!(1975), bonds: tbond!(1975), inflation: 9.143   },
    HistoricalYear { year: 1976, stocks: sp500!(1976), bonds: tbond!(1976), inflation: 5.745   },
    HistoricalYear { year: 1977, stocks: sp500!(1977), bonds: tbond!(1977), inflation: 6.502   },
    HistoricalYear { year: 1978, stocks: sp500!(1978), bonds: tbond!(1978), inflation: 7.631   },
    HistoricalYear { year: 1979, stocks: sp500!(1979), bonds: tbond!(1979), inflation: 11.254  },
    HistoricalYear { year: 1980, stocks: sp500!(1980), bonds: tbond!(1980), inflation: 13.549  },
    HistoricalYear { year: 1981, stocks: sp500!(1981), bonds: tbond!(1981), inflation: 10.335  },
    HistoricalYear { year: 1982, stocks: sp500!(1982), bonds: tbond!(1982), inflation: 6.131   },
    HistoricalYear { year: 1983, stocks: sp500!(1983), bonds: tbond!(1983), inflation: 3.212   },
    HistoricalYear { year: 1984, stocks: sp500!(1984), bonds: tbond!(1984), inflation: 4.301   },
    HistoricalYear { year: 1985, stocks: sp500!(1985), bonds: tbond!(1985), inflation: 3.546   },
    HistoricalYear { year: 1986, stocks: sp500!(1986), bonds: tbond!(1986), inflation: 1.898   },
    HistoricalYear { year: 1987, stocks: sp500!(1987), bonds: tbond!(1987), inflation: 3.665   },
    HistoricalYear { year: 1988, stocks: sp500!(1988), bonds: tbond!(1988), inflation: 4.078   },
    HistoricalYear { year: 1989, stocks: sp500!(1989), bonds: tbond!(1989), inflation: 4.827   },
    HistoricalYear { year: 1990, stocks: sp500!(1990), bonds: tbond!(1990), inflation: 5.398   },
    HistoricalYear { year: 1991, stocks: sp500!(1991), bonds: tbond!(1991), inflation: 4.235   },
    HistoricalYear { year: 1992, stocks: sp500!(1992), bonds: tbond!(1992), inflation: 3.029   },
    HistoricalYear { year: 1993, stocks: sp500!(1993), bonds: tbond!(1993), inflation: 2.952   },
    HistoricalYear { year: 1994, stocks: sp500!(1994), bonds: tbond!(1994), inflation: 2.607   },
    HistoricalYear { year: 1995, stocks: sp500!(1995), bonds: tbond!(1995), inflation: 2.805   },
    HistoricalYear { year: 1996, stocks: sp500!(1996), bonds: tbond!(1996), inflation: 2.931   },
    HistoricalYear { year: 1997, stocks: sp500!(1997), bonds: tbond!(1997), inflation: 2.338   },
    HistoricalYear { year: 1998, stocks: sp500!(1998), bonds: tbond!(1998), inflation: 1.552   },
    HistoricalYear { year: 1999, stocks: sp500!(1999), bonds: tbond!(1999), inflation: 2.188   },
    HistoricalYear { year: 2000, stocks: sp500!(2000), bonds: tbond!(2000), inflation: 3.377   },
    HistoricalYear { year: 2001, stocks: sp500!(2001), bonds: tbond!(2001), inflation: 2.826   },
    HistoricalYear { year: 2002, stocks: sp500!(2002), bonds: tbond!(2002), inflation: 1.586   },
    HistoricalYear { year: 2003, stocks: sp500!(2003), bonds: tbond!(2003), inflation: 2.270   },
    HistoricalYear { year: 2004, stocks: sp500!(2004), bonds: tbond!(2004), inflation: 2.677   },
    HistoricalYear { year: 2005, stocks: sp500!(2005), bonds: tbond!(2005), inflation: 3.393   },
    HistoricalYear { year: 2006, stocks: sp500!(2006), bonds: tbond!(2006), inflation: 3.226   },
    HistoricalYear { year: 2007, stocks: sp500!(2007), bonds: tbond!(2007), inflation: 2.853   },
    HistoricalYear { year: 2008, stocks: sp500!(2008), bonds: tbond!(2008), inflation: 3.839   },
    HistoricalYear { year: 2009, stocks: sp500!(2009), bonds: tbond!(2009), inflation: -0.356  },
    HistoricalYear { year: 2010, stocks: sp500!(2010), bonds: tbond!(2010), inflation: 1.640   },
    HistoricalYear { year: 2011, stocks: sp500!(2011), bonds: tbond!(2011), inflation: 3.157   },
    HistoricalYear { year: 2012, stocks: sp500!(2012), bonds: tbond!(2012), inflation: 2.069   },
    HistoricalYear { year: 2013, stocks: sp500!(2013), bonds: tbond!(2013), inflation: 1.465   },
    HistoricalYear { year: 2014, stocks: sp500!(2014), bonds: tbond!(2014), inflation: 1.622   },
    HistoricalYear { year: 2015, stocks: sp500!(2015), bonds: tbond!(2015), inflation: 0.119   },
    HistoricalYear { year: 2016, stocks: sp500!(2016), bonds: tbond!(2016), inflation: 1.262   },
    HistoricalYear { year: 2017, stocks: sp500!(2017), bonds: tbond!(2017), inflation: 2.130   },
    HistoricalYear { year: 2018, stocks: sp500!(2018), bonds: tbond!(2018), inflation: 2.443   },
    HistoricalYear { year: 2019, stocks: sp500!(2019), bonds: tbond!(2019), inflation: 2.290   },
    HistoricalYear { year: 2020, stocks: sp500!(2020), bonds: tbond!(2020), inflation: 1.20   },
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
            let is = stonks.grow(&RETURNS[i].stocks);
            stonks.invest(is);
            let ib = bonds.grow(&RETURNS[i].bonds);
            bonds.invest(ib);
        }
        assert_eq!((100.0 * stonks.value).round() / 100.0, 592_868.15);
        assert_eq!((100.0 * bonds.value).round() / 100.0, 8_920.90);
    }
}
