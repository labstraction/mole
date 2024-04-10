use std::{fs, io::{self, Write}, process};

use crate::{config::Config, scanner};

pub fn start(config: Config){

    if config.is_prompt() {
        start_prompt()
    } else {
        start_file(config.file_path().unwrap())
    }

}

fn start_prompt(){

    loop {

        print!(":> ");
        io::stdout().flush().unwrap();
    
        let mut line = String::new();
        
        io::stdin()
            .read_line(&mut line)
            .unwrap_or_else(|err| {display_error(err.to_string(), 0); 0});

        line.truncate(line.len() - 1);

        match line{
            key if key.to_lowercase() == "exit" => break,
            key if !key.is_empty()              => scan(key),
            _                                           => continue
        }
        
    }

    process::exit(1);

}

fn start_file(path: &str){

    match fs::read_to_string(path){
        Ok(content) => scan(content),
        Err(error) => {display_error(error.to_string(), 0)}
    }

}

fn scan(script: String){
    scanner::scan(script);
}

fn display_error(message: String, line: i32) {
    let report = create_report(message, line);
    eprintln!("{}", report);
    process::exit(1);
}

fn create_report(message: String, line: i32) -> String{

    return if line > 0 { 
        format!("[line {}] Error: {}", line, message)} 
    else {
        format!("Error: {}", message)
    };
    
}
