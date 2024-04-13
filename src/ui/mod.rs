use std::{fs, io::{self, Write}};

use crate::{config::Config, scanner, scanner2};

pub fn start(config: Config) -> Result<(), String>{

    if config.is_prompt() {
        start_prompt()
    } else {
        start_file(config.file_path().unwrap())
    }

}

fn start_prompt() -> Result<(), String>{

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
            key if !key.is_empty()              => scan(key).unwrap(),
            _                                           => continue
        }
        
    }

    Ok(())

}

fn start_file(path: &str) -> Result<(), String>{

    match fs::read_to_string(path){
        Ok(content) => scan(content),
        Err(error) => Err(error.to_string())
    }

}

fn scan(script: String) -> Result<(), String>{
    let mut scanner = scanner2::Scanner::new(script);
    scanner.scan_tokens();
    // let tokens = match scanner::scan(script){
    //     Ok(tokens) => tokens,
    //     Err(message) => return Err(message)
    // };
    println!("{:#?}", scanner);
    Ok(())
}

pub fn display_error(message: String, line: i32) {
    let report = create_report(message, line);
    eprintln!("{}", report);
}

fn create_report(message: String, line: i32) -> String{

    if line > 0 { 
        format!("[line {}] Error: {}", line, message)} 
    else {
        format!("Error: {}", message)
    }
    
}
