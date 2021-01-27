fn distributon_period(age: i32) -> Option<f64> {
    #[rustfmt::skip]
    const DISTRIBUTION_PERIOD: [f64; 46] = [
        27.4,  // Age 70
        26.5,
        25.6,
        24.7,
        23.8,
        22.9,
        22.0,
        21.2,
        20.3,
        19.5,
        18.7,
        17.9,
        17.1,
        16.3,
        15.5,
        14.8,
        14.1,
        13.4,
        12.7,
        12.0,
        11.4,
        10.8,
        10.2,
        9.6,
        9.1,
        8.6,
        8.1,
        7.6,
        7.1,
        6.7,
        6.3,
        5.9,
        5.5,
        5.2,
        4.9,
        4.5,
        4.2,
        3.9,
        3.7,
        3.4,
        3.1,
        2.9,
        2.6,
        2.4,
        2.1,
        1.9,  // Age 115 and over
    ];
    if age < 70 {
        None
    } else {
        Some(DISTRIBUTION_PERIOD[(age - 70).min(45) as usize])
    }
}

pub fn rmd_fraction(age: i32) -> f64 {
    match distributon_period(age) {
        None => 0.0,
        Some(dp) => 1.0 / dp,
    }
}

#[cfg(test)]
mod rmd_tests {
    use crate::rmd::*;

    #[test]
    fn distributon_period_test() {
        assert_eq!(distributon_period(40), Option::None);
        assert_eq!(distributon_period(70), Option::Some(27.4));
        assert_eq!(distributon_period(80), Option::Some(18.7));
        assert_eq!(distributon_period(115), Option::Some(1.9));
        assert_eq!(distributon_period(120), Option::Some(1.9));
    }

    #[test]
    fn rmd_fraction_test() {
        assert_eq!(rmd_fraction(40), 0.0);
        assert_eq!(rmd_fraction(80), 1.0 / 18.7);
        assert_eq!(rmd_fraction(120), 1.0 / 1.9);
    }
}
