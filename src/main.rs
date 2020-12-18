mod historical_returns;
mod asset;

fn main() {
    let mut normal = asset::Asset::new(100.0);
    let mut roth = asset::Asset::new(100.0);
    let marginal_rate = 0.42;
    for r in historical_returns::RETURNS.iter() {
        let x = roth.grow(&r.stocks);
        roth.invest(x);
        let x = normal.grow(&r.stocks);
        normal.invest(x * (1.0-marginal_rate));
        println!("{:>20.2}{:>20.2}{:>20.2}", normal.value, roth.value, 100.0*normal.value/roth.value);
    }
}
