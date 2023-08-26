use time::macros::datetime;

mod data;
mod hurst_exponent;
mod math;

fn main() {
    hurst_example();
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
