use std::env;
use mole::{config::Config, io, scanner2};

fn main() {

    let config = Config::create(env::args().collect());
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

    match io::read_file(config.file_path().unwrap()) {
        Ok(content) => match scanner2::scan(&content) {
            Ok(tokens) => println!("{:#?}", tokens),
            Err((line, message)) => io::display_error(message, line)
        },
        Err(message) => {
            io::display_error(message, 0);
            return;
        }
    };

    
}

fn run_repl(){

    loop {

        match io::prompt_repl() {
            Ok(content) => {
                match content.as_str() {
                    "exit" => break,
                    other => {
                        match scanner2::scan(other) {
                            Ok(tokens) => println!("{:#?}", tokens),
                            Err((line, message)) => io::display_error(message, line)
                        }
                    }
                }
            },
            Err(message) => {
                io::display_error(message, 0);
                break;
            }
        }
        
    }

}

