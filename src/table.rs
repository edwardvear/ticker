use reqwest::blocking::Client;

use tui::backend::Backend;
use tui::terminal::Frame;
use tui::widgets::Row;
use tui::layout::{Constraint, Rect};

use crate::iex;

pub struct Table {
    header: Vec<String>,
    rows: Vec<Vec<String>>,
    symbols: Vec<String>,
    client: Client,
}

impl Table {
    pub fn new(symbols: &Vec<String>) -> Table {
        let client = Client::builder().build().expect("Failed to build reqwest client");
        let header = vec![String::from("Symbol"), String::from("Latest Price")];
        let mut rows = Vec::new();
        for symbol in symbols {
            let quote = iex::get_quote(&symbol, &client);
            
            if let Ok(quote)= quote {
                if let Some(price) = quote.latest_price {
                    rows.push(vec![String::from(symbol), price.to_string()]);
                } else {
                    rows.push(vec![String::from(symbol), String::from("NULL")]);
                }
            } else {
                rows.push(vec![String::from(symbol), String::from("NULL")]);
            }
        }

        Table {
            header: header,
            rows: rows,
            symbols: symbols.to_vec(),
            client: client,
        }
    }

    pub fn add_symbol(&mut self, new_symbol: String) {
        if !self.symbols.contains(&new_symbol) {
            self.symbols.push(new_symbol);
        }
        self.update();
    }

    pub fn update(&mut self) {
        let mut rows = Vec::new();
        for symbol in &self.symbols {
            let quote = iex::get_quote(&symbol, &self.client);
            if let Ok(quote)= quote {
                if let Some(price) = quote.latest_price {
                    rows.push(vec![String::from(symbol), price.to_string()]);
                } else {
                    rows.push(vec![String::from(symbol), String::from("NULL")]);
                }
            } else {
                rows.push(vec![String::from(symbol), String::from("NULL")]);
            }
        }

        self.rows = rows;
    }

    pub fn render(&self, frame: &mut Frame<impl Backend>, rect: Rect) {
        let table = tui::widgets::Table::new(
            self.header.clone().into_iter(), 
            self.rows.clone().into_iter().map(|row| {
                Row::Data(row.into_iter())
            }))
            .widths(&[Constraint::Length(10), Constraint::Length(20)]);

        frame.render_widget(table, rect);
    }
}

