# Ticker

Simple real time stock ticker in Rust. Currently uses the IEX api, so you will need an api key.

## Basic Usage

A json config file is used for configuring the refresh rate and symbols to display. 
An example config is included, but the format is
```
[
  refresh_rate: (int),
  symbols: [(string)]
]
```

You will need an IEX api token for now. The app reads it in through the `API_TOKEN` environment variable.

To run the app, use `API_TOKEN=$yourtoken cargo run`.
To quit, use q or ^C.

## Current Issues

Sometimes the api call fails, and the price is replaced with NULL. Resending the request is a possible solution, but it could end up wasting a bunch of api credits.
