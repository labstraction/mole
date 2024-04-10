use std::env;
use mole::{config::Config, ui};

fn main() {
    let config = Config::create(env::args().collect());
    ui::start(config);
}
