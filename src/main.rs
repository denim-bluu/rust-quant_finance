use ndarray::*;
use time::{macros::datetime, OffsetDateTime};
use tokio_test;
use yahoo_finance_api as yahoo;

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
    println!("Quote history of VTI:\n{:#?}", quote_vec);
    let quote_arr = ndarray::Array::try_from(quote_vec).unwrap();
    println!("Quote history of VTI:\n{:#?}", quote_arr);


}

fn shift_arr() {
    
}