use std::fmt;

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

#[derive(Debug)]
pub enum Literal {
    Empty,
}

#[derive(Debug)]
pub struct Token {
    pub t_type: TokenType,
    pub lexeme: String,
    pub literal: Literal,
    pub line: usize,
}

impl Token {
    pub fn new(t_type: TokenType, lexeme: String, literal: Literal, line: usize) -> Token {
        Token {
            t_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "type: {:?}, lexeme: {}, literal:{:?}",
            self.t_type, self.lexeme, self.literal
        )
    }
}

#[derive(Debug)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            String::from(""),
            Literal::Empty,
            self.line,
        ));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap_or(' ');
        self.current += 1;
        c
    }

    fn add_token(&mut self, t_type: TokenType, literal: Literal) {
        let text = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(t_type, String::from(text), literal, self.line));
    }

    fn check(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap_or(' ') != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char{
        if self.is_at_end() { return '\0'};
        return self.source.chars().nth(self.current).unwrap_or(' ');
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, Literal::Empty),
            ')' => self.add_token(TokenType::RightParen, Literal::Empty),
            '{' => self.add_token(TokenType::LeftBrace, Literal::Empty),
            '}' => self.add_token(TokenType::RightBrace, Literal::Empty),
            ',' => self.add_token(TokenType::Comma, Literal::Empty),
            '.' => self.add_token(TokenType::Dot, Literal::Empty),
            '-' => self.add_token(TokenType::Minus, Literal::Empty),
            '+' => self.add_token(TokenType::Plus, Literal::Empty),
            ';' => self.add_token(TokenType::Semicolon, Literal::Empty),
            '*' => self.add_token(TokenType::Star, Literal::Empty),
            '!' => {
                if self.check('=') {
                    self.add_token(TokenType::BangEqual, Literal::Empty)
                } else {
                    self.add_token(TokenType::Bang, Literal::Empty)
                }
            }
            '=' => {
                if self.check('=') {
                    self.add_token(TokenType::EqualEqual, Literal::Empty)
                } else {
                    self.add_token(TokenType::Equal, Literal::Empty)
                }
            }
            '<' => {
                if self.check('=') {
                    self.add_token(TokenType::LessEqual, Literal::Empty)
                } else {
                    self.add_token(TokenType::Equal, Literal::Empty)
                }
            }
            '>' => {
                if self.check('=') {
                    self.add_token(TokenType::GreaterEqual, Literal::Empty)
                } else {
                    self.add_token(TokenType::Equal, Literal::Empty)
                }
            }
            '/' => {
                if self.check('/')  {
                    while self.peek() != '\n' && !self.is_at_end(){ self.advance();};
                  } else {
                    self.add_token(TokenType::Slash, Literal::Empty)
                  }
            }
            _ => self.add_token(TokenType::Eof, Literal::Empty),
        };
    }
}
