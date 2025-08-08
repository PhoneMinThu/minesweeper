#![deny(clippy::all, clippy::pedantic)]

mod app;
mod board;
mod difficulty;
mod error;
mod input;
mod ui;

use crate::app::App;
use crate::difficulty::Difficulty;

fn main() {
    let mut app = App::new(Difficulty::Easy);
    if let Err(e) = app.run() {
        eprintln!("Application error: {e}");
    }
}
