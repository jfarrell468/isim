use crate::asset::*;

use std::fmt::Debug;

#[derive(Debug)]
pub struct HistoricalYear {
    pub year: i32,
    pub stocks: AssetReturn,
    pub bonds: AssetReturn,
    pub inflation: f64
}

// From http://pages.stern.nyu.edu/~adamodar/New_Home_Page/datafile/histretSPX.html
pub const RETURNS: [HistoricalYear; 92] = [
    HistoricalYear { year: 1928, stocks: AssetReturn { cg: 37.882,  id: 5.929 }, bonds: AssetReturn { cg: -2.335,  id: 3.170  }, inflation: -1.152  },
    HistoricalYear { year: 1929, stocks: AssetReturn { cg: -11.910, id: 3.612 }, bonds: AssetReturn { cg: 0.754,   id: 3.450  }, inflation: 0.000   },
    HistoricalYear { year: 1930, stocks: AssetReturn { cg: -28.485, id: 3.361 }, bonds: AssetReturn { cg: 1.181,   id: 3.360  }, inflation: -2.671  },
    HistoricalYear { year: 1931, stocks: AssetReturn { cg: -47.066, id: 3.229 }, bonds: AssetReturn { cg: -5.779,  id: 3.220  }, inflation: -8.932  },
    HistoricalYear { year: 1932, stocks: AssetReturn { cg: -14.778, id: 6.136 }, bonds: AssetReturn { cg: 4.860,   id: 3.930  }, inflation: -10.301 },
    HistoricalYear { year: 1933, stocks: AssetReturn { cg: 44.075,  id: 5.907 }, bonds: AssetReturn { cg: -1.495,  id: 3.350  }, inflation: -5.192  },
    HistoricalYear { year: 1934, stocks: AssetReturn { cg: -4.714,  id: 3.526 }, bonds: AssetReturn { cg: 4.433,   id: 3.530  }, inflation: 3.479   },
    HistoricalYear { year: 1935, stocks: AssetReturn { cg: 41.368,  id: 5.372 }, bonds: AssetReturn { cg: 1.462,   id: 3.010  }, inflation: 2.553   },
    HistoricalYear { year: 1936, stocks: AssetReturn { cg: 27.923,  id: 4.021 }, bonds: AssetReturn { cg: 2.178,   id: 2.840  }, inflation: 1.032   },
    HistoricalYear { year: 1937, stocks: AssetReturn { cg: -38.591, id: 3.255 }, bonds: AssetReturn { cg: -1.211,  id: 2.590  }, inflation: 3.726   },
    HistoricalYear { year: 1938, stocks: AssetReturn { cg: 24.550,  id: 4.733 }, bonds: AssetReturn { cg: 1.483,   id: 2.730  }, inflation: -2.028  },
    HistoricalYear { year: 1939, stocks: AssetReturn { cg: -5.175,  id: 4.077 }, bonds: AssetReturn { cg: 1.852,   id: 2.560  }, inflation: -1.301  },
    HistoricalYear { year: 1940, stocks: AssetReturn { cg: -15.088, id: 4.415 }, bonds: AssetReturn { cg: 3.052,   id: 2.350  }, inflation: 0.719   },
    HistoricalYear { year: 1941, stocks: AssetReturn { cg: -17.864, id: 5.092 }, bonds: AssetReturn { cg: -4.032,  id: 2.010  }, inflation: 5.116   },
    HistoricalYear { year: 1942, stocks: AssetReturn { cg: 12.428,  id: 6.746 }, bonds: AssetReturn { cg: -0.175,  id: 2.470  }, inflation: 10.922  },
    HistoricalYear { year: 1943, stocks: AssetReturn { cg: 19.447,  id: 5.614 }, bonds: AssetReturn { cg: 0.000,   id: 2.490  }, inflation: 5.969   },
    HistoricalYear { year: 1944, stocks: AssetReturn { cg: 13.796,  id: 5.235 }, bonds: AssetReturn { cg: 0.088,   id: 2.490  }, inflation: 1.637   },
    HistoricalYear { year: 1945, stocks: AssetReturn { cg: 30.723,  id: 5.098 }, bonds: AssetReturn { cg: 1.324,   id: 2.480  }, inflation: 2.274   },
    HistoricalYear { year: 1946, stocks: AssetReturn { cg: -11.866, id: 3.437 }, bonds: AssetReturn { cg: 0.798,   id: 2.330  }, inflation: 8.476   },
    HistoricalYear { year: 1947, stocks: AssetReturn { cg: 0.000,   id: 5.200 }, bonds: AssetReturn { cg: -1.320,  id: 2.240  }, inflation: 14.389  },
    HistoricalYear { year: 1948, stocks: AssetReturn { cg: -0.654,  id: 6.358 }, bonds: AssetReturn { cg: -0.439,  id: 2.390  }, inflation: 7.689   },
    HistoricalYear { year: 1949, stocks: AssetReturn { cg: 10.461,  id: 7.843 }, bonds: AssetReturn { cg: 2.223,   id: 2.440  }, inflation: -0.971  },
    HistoricalYear { year: 1950, stocks: AssetReturn { cg: 21.680,  id: 9.126 }, bonds: AssetReturn { cg: -1.760,  id: 2.190  }, inflation: 1.085   },
    HistoricalYear { year: 1951, stocks: AssetReturn { cg: 16.349,  id: 7.330 }, bonds: AssetReturn { cg: -2.685,  id: 2.390  }, inflation: 7.860   },
    HistoricalYear { year: 1952, stocks: AssetReturn { cg: 11.780,  id: 6.371 }, bonds: AssetReturn { cg: -0.432,  id: 2.700  }, inflation: 2.279   },
    HistoricalYear { year: 1953, stocks: AssetReturn { cg: -6.624,  id: 5.416 }, bonds: AssetReturn { cg: 1.394,   id: 2.750  }, inflation: 0.816   },
    HistoricalYear { year: 1954, stocks: AssetReturn { cg: 45.022,  id: 7.541 }, bonds: AssetReturn { cg: 0.700,   id: 2.590  }, inflation: 0.311   },
    HistoricalYear { year: 1955, stocks: AssetReturn { cg: 26.404,  id: 6.194 }, bonds: AssetReturn { cg: -3.846,  id: 2.510  }, inflation: -0.279  },
    HistoricalYear { year: 1956, stocks: AssetReturn { cg: 2.617,   id: 4.823 }, bonds: AssetReturn { cg: -5.216,  id: 2.960  }, inflation: 1.525   },
    HistoricalYear { year: 1957, stocks: AssetReturn { cg: -14.313, id: 3.856 }, bonds: AssetReturn { cg: 3.207,   id: 3.590  }, inflation: 3.342   },
    HistoricalYear { year: 1958, stocks: AssetReturn { cg: 38.060,  id: 5.660 }, bonds: AssetReturn { cg: -5.309,  id: 3.210  }, inflation: 2.729   },
    HistoricalYear { year: 1959, stocks: AssetReturn { cg: 8.477,   id: 3.580 }, bonds: AssetReturn { cg: -6.507,  id: 3.860  }, inflation: 1.011   },
    HistoricalYear { year: 1960, stocks: AssetReturn { cg: -2.972,  id: 3.309 }, bonds: AssetReturn { cg: 6.950,   id: 4.690  }, inflation: 1.458   },
    HistoricalYear { year: 1961, stocks: AssetReturn { cg: 23.129,  id: 3.509 }, bonds: AssetReturn { cg: -1.779,  id: 3.840  }, inflation: 1.071   },
    HistoricalYear { year: 1962, stocks: AssetReturn { cg: -11.810, id: 2.998 }, bonds: AssetReturn { cg: 1.634,   id: 4.060  }, inflation: 1.199   },
    HistoricalYear { year: 1963, stocks: AssetReturn { cg: 18.891,  id: 3.721 }, bonds: AssetReturn { cg: -2.176,  id: 3.860  }, inflation: 1.240   },
    HistoricalYear { year: 1964, stocks: AssetReturn { cg: 12.970,  id: 3.446 }, bonds: AssetReturn { cg: -0.402,  id: 4.130  }, inflation: 1.279   },
    HistoricalYear { year: 1965, stocks: AssetReturn { cg: 9.062,   id: 3.337 }, bonds: AssetReturn { cg: -3.461,  id: 4.180  }, inflation: 1.585   },
    HistoricalYear { year: 1966, stocks: AssetReturn { cg: -13.091, id: 3.120 }, bonds: AssetReturn { cg: -1.712,  id: 4.620  }, inflation: 3.015   },
    HistoricalYear { year: 1967, stocks: AssetReturn { cg: 20.092,  id: 3.711 }, bonds: AssetReturn { cg: -6.421,  id: 4.840  }, inflation: 2.773   },
    HistoricalYear { year: 1968, stocks: AssetReturn { cg: 7.660,   id: 3.154 }, bonds: AssetReturn { cg: -2.425,  id: 5.700  }, inflation: 4.272   },
    HistoricalYear { year: 1969, stocks: AssetReturn { cg: -11.361, id: 3.120 }, bonds: AssetReturn { cg: -11.044, id: 6.030  }, inflation: 5.462   },
    HistoricalYear { year: 1970, stocks: AssetReturn { cg: 0.098,   id: 3.463 }, bonds: AssetReturn { cg: 9.105,   id: 7.650  }, inflation: 5.838   },
    HistoricalYear { year: 1971, stocks: AssetReturn { cg: 10.787,  id: 3.434 }, bonds: AssetReturn { cg: 3.397,   id: 6.390  }, inflation: 4.293   },
    HistoricalYear { year: 1972, stocks: AssetReturn { cg: 15.633,  id: 3.122 }, bonds: AssetReturn { cg: -3.112,  id: 5.930  }, inflation: 3.272   },
    HistoricalYear { year: 1973, stocks: AssetReturn { cg: -17.366, id: 3.057 }, bonds: AssetReturn { cg: -2.701,  id: 6.360  }, inflation: 6.178   },
    HistoricalYear { year: 1974, stocks: AssetReturn { cg: -29.718, id: 3.816 }, bonds: AssetReturn { cg: -4.751,  id: 6.740  }, inflation: 11.055  },
    HistoricalYear { year: 1975, stocks: AssetReturn { cg: 31.549,  id: 5.446 }, bonds: AssetReturn { cg: -3.825,  id: 7.430  }, inflation: 9.143   },
    HistoricalYear { year: 1976, stocks: AssetReturn { cg: 19.148,  id: 4.683 }, bonds: AssetReturn { cg: 7.985,   id: 8.000  }, inflation: 5.745   },
    HistoricalYear { year: 1977, stocks: AssetReturn { cg: -11.502, id: 4.522 }, bonds: AssetReturn { cg: -5.580,  id: 6.870  }, inflation: 6.502   },
    HistoricalYear { year: 1978, stocks: AssetReturn { cg: 1.062,   id: 5.447 }, bonds: AssetReturn { cg: -8.468,  id: 7.690  }, inflation: 7.631   },
    HistoricalYear { year: 1979, stocks: AssetReturn { cg: 12.309,  id: 6.211 }, bonds: AssetReturn { cg: -8.339,  id: 9.010  }, inflation: 11.254  },
    HistoricalYear { year: 1980, stocks: AssetReturn { cg: 25.774,  id: 5.962 }, bonds: AssetReturn { cg: -13.380, id: 10.390 }, inflation: 13.549  },
    HistoricalYear { year: 1981, stocks: AssetReturn { cg: -9.730,  id: 5.028 }, bonds: AssetReturn { cg: -4.641,  id: 12.840 }, inflation: 10.335  },
    HistoricalYear { year: 1982, stocks: AssetReturn { cg: 14.761,  id: 5.658 }, bonds: AssetReturn { cg: 19.095,  id: 13.720 }, inflation: 6.131   },
    HistoricalYear { year: 1983, stocks: AssetReturn { cg: 17.271,  id: 5.066 }, bonds: AssetReturn { cg: -7.340,  id: 10.540 }, inflation: 3.212   },
    HistoricalYear { year: 1984, stocks: AssetReturn { cg: 1.401,   id: 4.746 }, bonds: AssetReturn { cg: 1.903,   id: 11.830 }, inflation: 4.301   },
    HistoricalYear { year: 1985, stocks: AssetReturn { cg: 26.333,  id: 4.902 }, bonds: AssetReturn { cg: 14.212,  id: 11.500 }, inflation: 3.546   },
    HistoricalYear { year: 1986, stocks: AssetReturn { cg: 14.620,  id: 3.874 }, bonds: AssetReturn { cg: 15.024,  id: 9.260  }, inflation: 1.898   },
    HistoricalYear { year: 1987, stocks: AssetReturn { cg: 2.028,   id: 3.785 }, bonds: AssetReturn { cg: -12.071, id: 7.110  }, inflation: 3.665   },
    HistoricalYear { year: 1988, stocks: AssetReturn { cg: 12.401,  id: 4.136 }, bonds: AssetReturn { cg: -0.766,  id: 8.990  }, inflation: 4.078   },
    HistoricalYear { year: 1989, stocks: AssetReturn { cg: 27.250,  id: 4.225 }, bonds: AssetReturn { cg: 8.584,   id: 9.110  }, inflation: 4.827   },
    HistoricalYear { year: 1990, stocks: AssetReturn { cg: -6.559,  id: 3.495 }, bonds: AssetReturn { cg: -1.605,  id: 7.840  }, inflation: 5.398   },
    HistoricalYear { year: 1991, stocks: AssetReturn { cg: 26.307,  id: 3.928 }, bonds: AssetReturn { cg: 6.925,   id: 8.080  }, inflation: 4.235   },
    HistoricalYear { year: 1992, stocks: AssetReturn { cg: 4.464,   id: 3.029 }, bonds: AssetReturn { cg: 2.272,   id: 7.090  }, inflation: 3.029   },
    HistoricalYear { year: 1993, stocks: AssetReturn { cg: 7.055,   id: 2.912 }, bonds: AssetReturn { cg: 7.441,   id: 6.770  }, inflation: 2.952   },
    HistoricalYear { year: 1994, stocks: AssetReturn { cg: -1.539,  id: 2.865 }, bonds: AssetReturn { cg: -13.807, id: 5.770  }, inflation: 2.607   },
    HistoricalYear { year: 1995, stocks: AssetReturn { cg: 34.111,  id: 3.085 }, bonds: AssetReturn { cg: 15.671,  id: 7.810  }, inflation: 2.805   },
    HistoricalYear { year: 1996, stocks: AssetReturn { cg: 20.264,  id: 2.417 }, bonds: AssetReturn { cg: -4.281,  id: 5.710  }, inflation: 2.931   },
    HistoricalYear { year: 1997, stocks: AssetReturn { cg: 31.008,  id: 2.095 }, bonds: AssetReturn { cg: 3.639,   id: 6.300  }, inflation: 2.338   },
    HistoricalYear { year: 1998, stocks: AssetReturn { cg: 26.669,  id: 1.669 }, bonds: AssetReturn { cg: 9.111,   id: 5.810  }, inflation: 1.552   },
    HistoricalYear { year: 1999, stocks: AssetReturn { cg: 19.526,  id: 1.359 }, bonds: AssetReturn { cg: -12.904, id: 4.650  }, inflation: 2.188   },
    HistoricalYear { year: 2000, stocks: AssetReturn { cg: -10.139, id: 1.107 }, bonds: AssetReturn { cg: 10.215,  id: 6.440  }, inflation: 3.377   },
    HistoricalYear { year: 2001, stocks: AssetReturn { cg: -13.042, id: 1.192 }, bonds: AssetReturn { cg: 0.462,   id: 5.110  }, inflation: 2.826   },
    HistoricalYear { year: 2002, stocks: AssetReturn { cg: -23.367, id: 1.401 }, bonds: AssetReturn { cg: 10.066,  id: 5.050  }, inflation: 1.586   },
    HistoricalYear { year: 2003, stocks: AssetReturn { cg: 26.379,  id: 1.977 }, bonds: AssetReturn { cg: -3.445,  id: 3.820  }, inflation: 2.270   },
    HistoricalYear { year: 2004, stocks: AssetReturn { cg: 8.994,   id: 1.748 }, bonds: AssetReturn { cg: 0.241,   id: 4.250  }, inflation: 2.677   },
    HistoricalYear { year: 2005, stocks: AssetReturn { cg: 3.001,   id: 1.833 }, bonds: AssetReturn { cg: -1.352,  id: 4.220  }, inflation: 3.393   },
    HistoricalYear { year: 2006, stocks: AssetReturn { cg: 13.619,  id: 1.993 }, bonds: AssetReturn { cg: -2.429,  id: 4.390  }, inflation: 3.226   },
    HistoricalYear { year: 2007, stocks: AssetReturn { cg: 3.530,   id: 1.955 }, bonds: AssetReturn { cg: 5.510,   id: 4.700  }, inflation: 2.853   },
    HistoricalYear { year: 2008, stocks: AssetReturn { cg: -38.486, id: 1.933 }, bonds: AssetReturn { cg: 16.081,  id: 4.020  }, inflation: 3.839   },
    HistoricalYear { year: 2009, stocks: AssetReturn { cg: 23.454,  id: 2.481 }, bonds: AssetReturn { cg: -13.327, id: 2.210  }, inflation: -0.356  },
    HistoricalYear { year: 2010, stocks: AssetReturn { cg: 12.783,  id: 2.038 }, bonds: AssetReturn { cg: 4.623,   id: 3.840  }, inflation: 1.640   },
    HistoricalYear { year: 2011, stocks: AssetReturn { cg: -0.003,  id: 2.102 }, bonds: AssetReturn { cg: 12.745,  id: 3.290  }, inflation: 3.157   },
    HistoricalYear { year: 2012, stocks: AssetReturn { cg: 13.406,  id: 2.485 }, bonds: AssetReturn { cg: 1.092,   id: 1.880  }, inflation: 2.069   },
    HistoricalYear { year: 2013, stocks: AssetReturn { cg: 29.601,  id: 2.544 }, bonds: AssetReturn { cg: -10.865, id: 1.760  }, inflation: 1.465   },
    HistoricalYear { year: 2014, stocks: AssetReturn { cg: 11.391,  id: 2.134 }, bonds: AssetReturn { cg: 7.710,   id: 3.036  }, inflation: 1.622   },
    HistoricalYear { year: 2015, stocks: AssetReturn { cg: -0.729,  id: 2.107 }, bonds: AssetReturn { cg: -0.886,  id: 2.170  }, inflation: 0.119   },
    HistoricalYear { year: 2016, stocks: AssetReturn { cg: 9.537,   id: 2.236 }, bonds: AssetReturn { cg: -1.579,  id: 2.270  }, inflation: 1.262   },
    HistoricalYear { year: 2017, stocks: AssetReturn { cg: 19.420,  id: 2.186 }, bonds: AssetReturn { cg: 0.352,   id: 2.450  }, inflation: 2.130   },
    HistoricalYear { year: 2018, stocks: AssetReturn { cg: -6.237,  id: 2.010 }, bonds: AssetReturn { cg: -2.427,  id: 2.410  }, inflation: 2.443   },
    HistoricalYear { year: 2019, stocks: AssetReturn { cg: 28.878,  id: 2.346 }, bonds: AssetReturn { cg: 6.946,   id: 2.690  }, inflation: 2.290   }
];

mod historical_returns_tests {
    use crate::asset::*;
    use crate::historical_returns::*;

    #[test]
    pub fn returns_1928_to_2019() {
        let mut stonks = Asset::new(100.0);
        let mut bonds = Asset::new(100.0);
        for i in 0..92 {
            let is = stonks.grow(&RETURNS[i].stocks);
            stonks.invest(is);
            let ib = bonds.grow(&RETURNS[i].bonds);
            bonds.invest(ib);
        }
        // This *should* be 502_417.21
        assert_eq!((100.0 * stonks.value).round()/100.0, 502_423.42);
        // This *should* be 8_012.89
        assert_eq!((100.0 * bonds.value).round()/100.0,  8_012.76);
    }
}
