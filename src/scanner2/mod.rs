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
    // End
    Eof,
}

pub enum Literal{

}

#[derive(Debug)]
pub struct Token {
  pub t_type: TokenType,
  pub lexeme: String,
  pub literal: Literal,
  pub line: i32,
}

impl Token {

    pub fn new(t_type: TokenType, lexeme: String, literal: Literal, line: i32) -> Token{
        Token{t_type, lexeme, literal, line}
    }

    pub fn to_string(&self) -> String{
      format!("type: {}, lexeme: {}, literal:{}", self.t_type, self.lexeme, self.literal)
    }
}