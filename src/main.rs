use std::{env, process};
use mole::{config::Config, ui};

fn main() {
    let config = Config::create(env::args().collect());
    if let Err(message) = ui::start(config) {
        ui::display_error(message, 0);
        process::exit(1);
    }
}
