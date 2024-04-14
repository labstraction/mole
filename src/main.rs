use std::{env, process, task::Context};
use mole::{config::Config, io::read_file, ui};

fn main() {
    let config = Config::create(env::args().collect());
    // if let Err(message) = ui::start(config) {
    //     ui::display_error(message, 0);
    //     process::exit(1);
    // }
    run(config);
}


fn run(config: Config){

    if config.is_prompt() {
        run_repl()
    } else {
        run_file(config)
    }

}

fn run_file(config: Config){
    let content = match read_file(config.file_path().unwrap()) {
        Ok(content) => content,
        Err(message) => {
            println!("{}",message);
            return;
        }
    };
    println!("{}", content);
}

fn run_repl(){

}

