use nalgebra as na;
use time::macros::datetime;

mod data;
mod hurst_exponent;
mod math;
mod trading;
mod vis;

fn main() {
    let start = datetime!(2006-5-24 0:00:00.00 UTC);
    let end = datetime!(2012-4-9 0:00:00.00 UTC);
    let uso: Vec<f64> = data::get_history("USO", start, end)
        .iter()
        .map(|q| q.adjclose)
        .collect();
    let gld: Vec<f64> = data::get_history("GLD", start, end)
        .iter()
        .map(|q| q.adjclose)
        .collect();
    let pairs = trading::AssetPairs::new(gld, uso).unwrap();
    let hedge_ratios = pairs.calculate_hedge_ratio(20);
    vis::draw_line_chart("Hedge Ratio", &hedge_ratios);
    let portfolio = pairs.get_weighted_portfolio(20);
    vis::draw_line_chart("Constructed portfolio market value", &portfolio);
    let ret = pairs.bollinger_band_mean_reversion_strategy(1.0, 0.0, 20);
}

fn hurst_example() {
    let start = datetime!(2007-7-24 0:00:00.00 UTC);
    let end = datetime!(2012-3-27 0:00:00.00 UTC);
    let quotes = data::get_history("CAD=X", start, end);
    let q_vec: Vec<f64> = quotes.iter().map(|q| q.adjclose).collect();

    let ret = math::calculate_daily_returns(&q_vec);
    let hurst_exp = hurst_exponent::calcualte_hurst_exp(&ret);
    println!("Hurst Exponent: {:#?}", hurst_exp);
}
