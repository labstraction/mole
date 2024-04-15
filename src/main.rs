use std::env;
use mole::{config::Config, io::{display_error, prompt_repl, read_file}};

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
            display_error(message, 0);
            return;
        }
    };
    println!("{}", content);
    
}

fn run_repl(){

    loop {

        match prompt_repl() {
            Ok(content) => println!("{}", content),
            Err(message) => {
                display_error(message, 0);
                break;
            }
        }
        
    }

}

