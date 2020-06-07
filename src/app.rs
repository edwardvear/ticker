use crate::config::Config;

use std::{
    io,
    sync::mpsc::channel,
    thread,
    time::Duration,
};

use tui::{
    Terminal,
    backend::TermionBackend,
    layout::{Layout, Constraint, Direction},
};

use termion::{
    raw::{
        IntoRawMode,
        RawTerminal,
    },
    input::TermRead,
    event::Key,
};

pub struct App {
    terminal: Terminal<TermionBackend<RawTerminal<io::Stdout>>>,
    table: crate::table::Table,
}

impl App {
    pub fn from_config(config: &Config) -> App {
        let stdout = io::stdout().into_raw_mode().expect("Couldn't instantiate stdout");
        let backend = TermionBackend::new(stdout);
        let terminal = Terminal::new(backend).expect("Couldn't create terminal");

        let symbols = config.symbols.clone();
        let table = crate::table::Table::new(&symbols);

        App {
            terminal: terminal,
            table: table,
        }
    }

    pub fn start(&mut self) {
        let stdin = io::stdin();

        let (tx, rx) = channel();
        let input_tx = tx.clone();
        let timer_tx = tx.clone();

        // Input Loop
        thread::spawn(move || {
            let key_stream = stdin.keys();
            for c in key_stream {
                input_tx.send(Event::Input(c.unwrap())).expect("Failed to send Input");
            }
        });

        // Timer Loop
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::new(10, 0));
                timer_tx.send(Event::Tick).expect("Failed to send Tick");
            }
        });

        // Drawing Loop
        self.terminal.clear().expect("Failed to clear terminal");
        loop {
            let table = &mut self.table;
            self.terminal.draw(|mut f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Percentage(100),
                        ].as_ref()
                    )
                    .split(f.size());

                table.render(&mut f, chunks[0]);
            }).expect("Failed to draw terminal");
            let event = rx.recv().unwrap();
            match event {
                Event::Input(Key::Char('q')) => {
                    self.terminal.clear().expect("Failed to clear terminal");
                    break;
                },
                Event::Input(Key::Ctrl('c')) => {
                    self.terminal.clear().expect("Failed to clear terminal");
                    break;
                },
                Event::Tick => {
                    self.table.update();
                },
                _ => {},
            };
        }
    }
}

enum Event {
    Input(Key),
    Tick,
}
