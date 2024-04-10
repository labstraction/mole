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

pub struct Token {
  pub t_type: TokenType,
  pub lexeme: String,
  pub line: i32,
}

impl Token {
    pub fn to_string(&self) -> String{
      format!("line: {}, lexeme: {}", self.line, self.lexeme)
    }
}

pub fn scan(script: String) -> Vec<TokenType>{
    let mut tokens = Vec::new();
    for c in script.chars()  {
        match c {
            '(' => tokens.push(TokenType::LeftParen),
            _   => tokens.push(TokenType::Bang)
        }
    }

    tokens.push(TokenType::Eof);

    tokens
}


