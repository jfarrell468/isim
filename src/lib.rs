mod account;
mod asset;
pub mod config;
mod histret;
pub mod instance;
mod report;
mod rmd;
pub mod scenario;
mod tax;

#[macro_export]
macro_rules! assert_eq_decimal_places {
    ($left:expr, $right:expr, $precision:expr $(,)?) => {
        let x = 10.0_f64.powi($precision);
        assert_eq!(($left * x).round() / x, ($right * x).round() / x)
    };
}

#[macro_export]
macro_rules! assert_eq_cents {
    ($left:expr, $right:expr $(,)?) => {
        $crate::assert_eq_decimal_places!($left, $right, 2)
    };
}
