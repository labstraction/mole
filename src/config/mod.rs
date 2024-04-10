pub struct Config{
    pub args: Vec<String>
}

impl Config{
    
    pub fn create(args: Vec<String>)-> Config{
        Config{args}
    }

    pub fn is_prompt(&self) -> bool{
        self.args.len() < 2
    }

    pub fn file_path(&self) -> Option<&String>{
        self.args.last()
    }

    pub fn additional_args(&self) -> Option<&[String]> {
        self.args.get(1..self.args.len() - 1)      
    }

}