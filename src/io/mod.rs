
use std::{fs, io::{self, Write}};

pub fn read_file(path: &String) -> Result<String, String>{
    match fs::read_to_string(path){
        Ok(content) => Ok(content),
        Err(error) => Err(error.to_string())
    }
}

pub fn prompt_repl() -> Result<String, String>{

        print!(":> ");
        io::stdout().flush().unwrap();
    
        let mut line = String::new();
        
        match io::stdin().read_line(&mut line){
            Ok(_) => {
                line.truncate(line.len() - 1);
                Ok(line)
            }
            Err(error) => Err(error.to_string())
        }

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