// Calculates taxes on retirement income for a married couple in Illinois, filing
// jointly. Assumes that neither has wages, so there is no FICA tax.
pub fn tax(i: f64, cg: f64) -> f64 {
    let ill = illinois(i, cg);
    let fed = federal(i, cg);
    assert!(
        ill >= 0.0,
        "illinois tax = {}, income = {}, cg = {}",
        ill,
        i,
        cg
    );
    assert!(
        fed >= 0.0,
        "federal tax = {}, income = {}, cg = {}",
        fed,
        i,
        cg
    );
    ill + fed
}

fn illinois(i: f64, cg: f64) -> f64 {
    let income = i + cg;
    if income <= 500_000.0 {
        0.0495 * (income - 2.0 * 2325.0).max(0.0)
    } else {
        0.0495 * income
    }
}

fn federal(i: f64, cg: f64) -> f64 {
    let fi = fed_income(i);
    let fcg = fed_cg(i, cg);
    let niit = fed_niit(i, cg);
    assert!(
        fi >= 0.0,
        "federal income tax = {}, income = {}, cg = {}",
        fi,
        i,
        cg
    );
    assert!(
        fcg >= 0.0,
        "federal cap gains tax = {}, income = {}, cg = {}",
        fcg,
        i,
        cg
    );
    assert!(
        niit >= 0.0,
        "fed niit = {}, income = {}, cg = {}",
        niit,
        i,
        cg
    );
    fi + fcg + niit
}

#[rustfmt::skip]
const FED_INCOME_BRACKETS: [(f64, f64); 7] = [
    (0.37, 622_050.0),
    (0.35, 414_700.0),
    (0.32, 326_600.0),
    (0.24, 171_050.0),
    (0.22, 80_250.0),
    (0.12, 19_750.0),
    (0.10, 0.0),
];
fn fed_income(i: f64) -> f64 {
    let mut tax = 0.0;
    let income = (i - 24_800.0).max(0.0);
    if income > FED_INCOME_BRACKETS[0].1 {
        tax += FED_INCOME_BRACKETS[0].0 * (income - FED_INCOME_BRACKETS[0].1);
    }
    for j in 1..FED_INCOME_BRACKETS.len() {
        if income > FED_INCOME_BRACKETS[j].1 {
            tax += FED_INCOME_BRACKETS[j].0
                * (income.min(FED_INCOME_BRACKETS[j - 1].1) - FED_INCOME_BRACKETS[j].1)
        }
    }
    tax
}

#[rustfmt::skip]
const FED_CG_BRACKETS: [(f64, f64); 3] = [
    (0.2, 496_600.0),
    (0.15, 80_000.0),
    (0.0, 0.0)
];
fn fed_cg(i: f64, cg: f64) -> f64 {
    #[cfg(test)]
    println!("=============== income = {}, cg = {}", i, cg);
    let mut tax = 0.0;
    let income = i + cg;
    #[cfg(test)]
    println!(
        "bracket {}, {}%",
        FED_CG_BRACKETS[0].1,
        100.0 * FED_CG_BRACKETS[0].0
    );
    if income > FED_CG_BRACKETS[0].1 {
        #[cfg(test)]
        println!(
            "eligible cg = {}, taxed at {}%",
            (income - FED_CG_BRACKETS[0].1).min(cg),
            100.0 * FED_CG_BRACKETS[0].0
        );
        tax += FED_CG_BRACKETS[0].0 * (income - FED_CG_BRACKETS[0].1).min(cg);
    }
    for j in 1..FED_CG_BRACKETS.len() {
        #[cfg(test)]
        println!(
            "bracket {}, {}%",
            FED_CG_BRACKETS[j].1,
            100.0 * FED_CG_BRACKETS[j].0
        );
        #[cfg(test)]
        println!(
            "income in prior brackets: {}",
            (income - FED_CG_BRACKETS[j - 1].1).max(0.0)
        );
        let prev_cg = (income - FED_CG_BRACKETS[j - 1].1).max(0.0).min(cg);
        #[cfg(test)]
        println!("cg in prior brackets: {}", prev_cg);
        #[cfg(test)]
        println!(
            "income in this bracket: {}",
            (income.min(FED_CG_BRACKETS[j - 1].1) - FED_CG_BRACKETS[j].1).max(0.0)
        );
        let eligible_cg = (income.min(FED_CG_BRACKETS[j - 1].1) - FED_CG_BRACKETS[j].1)
            .max(0.0)
            .min(cg)
            - prev_cg;
        #[cfg(test)]
        println!("cg in this bracket: {}", eligible_cg);
        #[cfg(test)]
        println!(
            "eligible cg = {}, taxed at {}%",
            eligible_cg,
            100.0 * FED_CG_BRACKETS[j].0
        );
        tax += eligible_cg * FED_CG_BRACKETS[j].0;
    }
    tax
}

// 3.8% of total income over $250k. Assumes we are retired, and income is coming
// from interest and dividends, not wages.
fn fed_niit(i: f64, cg: f64) -> f64 {
    0.038 * (i + cg - 250_000.0).max(0.0)
}

// Calculates the value of investments to sell in order to have $l in living expenses
// after taxes, if those investments generate $i in interest in dividends, and have
// appreciated such that cg_ratio (between 0 and 1) of their value is unrealized
// capital gains.
pub fn how_much_to_sell(l: f64, i: f64, cg_ratio: f64) -> f64 {
    let mut guess = (l - i).max(0.0);
    assert!(
        guess >= 0.0,
        "guess = {}, l = {}, i = {}, cg_ratio = {}",
        guess,
        l,
        i,
        cg_ratio
    );
    while i + guess - tax(i, guess * cg_ratio) < l {
        // Linear search in $1k increments. We can do better, but this is OK for now.
        guess += 1000.0
    }
    guess
}

mod tax_tests {
    #[cfg(test)]
    use super::*;

    #[test]
    pub fn tax_test() {
        assert_eq!(
            (100.0 * illinois(50_000.0, 75_000.0)).round() / 100.0,
            5_957.33
        );
        assert_eq!(fed_income(50_000.0), 2_629.0);
        assert_eq!(fed_cg(50_000.0, 75_000.0), 6_750.0);
        assert_eq!(fed_niit(50_000.0, 75_000.0), 0.0);
        assert_eq!(
            (100.0 * tax(50_000.0, 75_000.0)).round() / 100.0,
            5_957.33 + 2_629.0 + 6_750.0
        );
    }

    #[test]
    pub fn illinois_test() {
        assert_eq!(illinois(0.0, 0.0), 0.0);
        assert_eq!(illinois(100.0, 0.0), 0.0);
        assert_eq!(illinois(0.0, 100.0), 0.0);
        assert_eq!(
            (100.0 * illinois(25_000.0, 25_000.0)).round() / 100.0,
            2244.83
        );
        assert_eq!(illinois(501_000.0, 0.0), 24_799.5);
    }

    #[test]
    pub fn fed_income_test() {
        assert_eq!(fed_income(20_000.0), 0.0);
        assert_eq!(fed_income(24_800.0 + 1000.0), 100.0);
        assert_eq!(fed_income(50_000.0), 2_629.0);
        assert_eq!(fed_income(125_000.0), 13_624.0);
        assert_eq!(fed_income(250_000.0), 42_207.0);
        assert_eq!(fed_income(350_000.0), 66_207.0);
        assert_eq!(fed_income(450_000.0), 98_410.0);
        assert_eq!(fed_income(750_000.0), 205_473.0);
    }

    #[test]
    pub fn fed_cg_test() {
        assert_eq!(fed_cg(0.0, 20_000.0), 0.0);
        assert_eq!(fed_cg(80_000.0, 20_000.0), 3_000.0);
        assert_eq!(fed_cg(400_000.0, 100.0), 15.00);
        assert_eq!(fed_cg(600_000.0, 100.0), 20.00);
        assert_eq!(fed_cg(490_000.0, 10_000.0), 0.15 * 6_600.0 + 0.20 * 3_400.0);
        assert_eq!(fed_cg(900_000.0, 0.0), 0.0);
    }

    #[test]
    pub fn fed_niit_test() {
        assert_eq!(fed_niit(100_000.0, 100_000.0), 0.0);
        assert_eq!(fed_niit(130_000.0, 130_000.0), 380.0);
    }

    #[test]
    pub fn how_much_to_sell_test() {
        assert_eq!(how_much_to_sell(180_000.0, 50_000.0, 0.5), 145_000.0);
    }
}
