use std::env;
use reqwest::blocking::Client;
use serde::{Serialize, Deserialize};

/// Should definitely not be blocking, but not sure if importing a whole async runtime just
/// for this is the correct approach. System threads for each request don't seem like the right approach either.
pub fn get_quote(symbol: &str, client: &Client) -> Result<Quote, reqwest::Error> {
    Ok(client.get(format!("https://cloud.iexapis.com/stable/stock/{}/quote", symbol).as_str())
        .query(&[("token", env::var("API_TOKEN")
            .expect("Please provide an api token with an API_TOKEN environment variable")
        )])
        .send()?
        .json::<Quote>()?
    )
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    symbol: Option<String>,
    company_name: Option<String>,
    primary_exchange: Option<String>,
    calculation_price: Option<String>,
    open: Option<f64>,
    open_time: Option<u64>,
    open_source: Option<String>,
    close: Option<f64>,
    close_time: Option<u64>,
    close_source: Option<String>,
    high: Option<f64>,
    high_time: Option<u64>,
    high_source: Option<String>,
    low: Option<f64>,
    low_time: Option<u64>,
    low_source: Option<String>,
    pub latest_price: Option<f64>,
    latest_source: Option<String>,
    latest_time: Option<String>,
    latest_update: Option<u64>,
    latest_volume: Option<i64>,
    iex_realtime_price: Option<f64>,
    iex_realtime_size: Option<u64>,
    iex_last_updated: Option<i64>,
    delayed_price: Option<f64>,
    delayed_price_time: Option<u64>,
    odd_lot_delayed_price: Option<f64>,
    odd_lot_delayed_price_time: Option<u64>,
    extended_price: Option<f64>,
    extended_price_time: Option<u64>,
    extended_change: Option<f64>,
    extended_change_percent: Option<f64>,
    previous_close: Option<f64>,
    previous_volume: Option<u64>,
    change: Option<f64>,
    change_percent: Option<f64>,
    volume: Option<u64>,
    iex_market_percent: Option<f64>,
    iex_volume: Option<u64>,
    avg_total_volume: Option<u64>,
    iex_bid_price: Option<f64>,
    iex_bid_size: Option<u64>,
    iex_ask_price: Option<f64>,
    iex_ask_size: Option<u64>,
    iex_open: Option<f64>,
    iex_open_time: Option<u64>,
    iex_close: Option<f64>,
    iex_close_time: Option<u64>,
    market_cap: Option<u64>,
    pe_ratio: Option<f64>,
    week_52_high: Option<f64>,
    week_52_low: Option<f64>,
    ytd_change: Option<f64>,
    last_trade_time: Option<u64>,
    is_us_market_open: Option<bool>,
}
