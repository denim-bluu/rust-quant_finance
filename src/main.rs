use ndarray::Array;
use time::{macros::datetime, OffsetDateTime};
use tokio_test;
use yahoo_finance_api as yahoo;

mod utils;
use crate::utils::*;

mod hurst_exponent;
use crate::hurst_exponent::*;

fn get_history(ticker: &str, start: OffsetDateTime, end: OffsetDateTime) -> Vec<yahoo::Quote> {
    let provider = yahoo::YahooConnector::new();
    let response = tokio_test::block_on(provider.get_quote_history(ticker, start, end)).unwrap();
    return response.quotes().unwrap();
}

fn main() {
    let start = datetime!(2020-1-1 0:00:00.00 UTC);
    let end = datetime!(2020-1-31 23:59:59.99 UTC);
    let quote_history = get_history("^GSPC", start, end);
    let quote_vec: Vec<f64> = quote_history.iter().map(|q| q.adjclose).collect();
    let quote_arr = Array::try_from(quote_vec).unwrap();

    let mean = quote_arr.mean().unwrap();
    let n = Array::range(0, 10, 1).map(|exponent| quote_arr.len() as f64 / 2.0.powi(exponent));
    print!("{:#?}", n.);
    
}
