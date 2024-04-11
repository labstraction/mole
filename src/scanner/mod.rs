#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Debug)]
pub struct Token {
  pub t_type: TokenType,
  pub lexeme: String,
  pub line: i32,
}

impl Token {

    pub fn new(t_type: TokenType, lexeme: String) -> Token{
        Token{t_type, lexeme, line: 0}
    }

    pub fn to_string(&self) -> String{
      format!("line: {}, lexeme: {}", self.line, self.lexeme)
    }
}

pub fn scan(script: String) -> Result<Vec<Token>, String>{
    let mut tokens = Vec::new();
    for c in script.chars()  {
        let t_type = match c {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '-' => TokenType::Minus,
            '+' => TokenType::Plus,
            ';' => TokenType::Semicolon,
            '*' => TokenType::Star,
             _  => return Err(format!("Unexpected character: {}", c)),
            
        };
        tokens.push(Token::new(t_type, c.to_string()))
    }

    tokens.push(Token::new(TokenType::Eof, String::from("")));

    Ok(tokens)
}


