mod app;
mod config;
mod table;
mod iex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = match config::Config::from_file("config.txt") {
        Ok(config) => config,
        Err(e) => {
            println!("Error loading config: {}", e);
            std::default::Default::default()
        }
    };

    let mut app = app::App::from_config(&config);
    app.start();

    Ok(())
}

