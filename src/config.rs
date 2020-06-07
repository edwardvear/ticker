use std::fs::File;
use std::io::BufReader;

#[derive(serde::Deserialize, serde::Serialize, std::default::Default)]
pub struct Config {
    pub refresh_rate: u16,
    pub symbols: Vec<String>,
}

impl Config {
    pub fn from_file(file: &str) -> Result<Config, std::io::Error> {
        let file = File::open(file)?;
        let reader = BufReader::new(file);

        let config = serde_json::from_reader(reader)?;
        Ok(config)
    }
}
