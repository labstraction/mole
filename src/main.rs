use std::env;
use std::io;
use std::io::Write;
use std::fs;
mod scanner;



fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        key if key > 2 => println!("too many params"),
        2                     => run_file(&args[1]),
        _                     => run_prompt()
    }

}

fn run_prompt(){

    loop {

        print!(":> ");
        io::stdout().flush().unwrap();
    
        let mut line = String::new();
        
        io::stdin()
            .read_line(&mut line)
            .expect("failed to read line");

        line.truncate(line.len() - 1);

        match line{
            key if key.to_lowercase() == "exit" => break,
            key if key != ""                    => run(key),
            _                                           => continue
        }
        
    }


}

fn run_file(path: &String){

    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    println!("{}", contents);

}

fn run(script: String){
    scanner::scan(script);
}