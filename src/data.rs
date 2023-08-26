use time::OffsetDateTime;
use tokio_test;
use yahoo_finance_api as yahoo;

/// The function `get_history` retrieves historical stock quotes for a given ticker symbol
/// within a specified time range.
/// 
/// Arguments:
/// 
/// * `ticker`: The `ticker` parameter is a string that represents the stock symbol or
/// ticker symbol of the company whose historical quotes you want to retrieve. For example,
/// "AAPL" represents Apple Inc.
/// * `start`: The `start` parameter is the starting date and time for the historical data
/// you want to retrieve. It is of type `OffsetDateTime`, which represents a date and time
/// with an offset from UTC.
/// * `end`: The `end` parameter is the end date and time for the historical data you want
/// to retrieve. It is of type `OffsetDateTime`, which represents a date and time with an
/// offset from UTC.
/// 
/// Returns:
/// 
/// a vector of `yahoo::Quote` objects.
pub fn get_history(ticker: &str, start: OffsetDateTime, end: OffsetDateTime) -> Vec<yahoo::Quote> {
    let provider = yahoo::YahooConnector::new();
    let response = tokio_test::block_on(provider.get_quote_history(ticker, start, end)).unwrap();
    return response.quotes().unwrap();
}
