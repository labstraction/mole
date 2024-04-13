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
    String(String),
    Number(f64),
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

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        };
        return self.source.chars().nth(self.current).unwrap_or(' ');
    }

    fn parse_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            print!("Unterminated string.");
            return;
        }

        // The closing ".
        self.advance();

        // Trim the surrounding quotes.
        let value = self
            .source
            .get(self.start + 1..self.current - 1)
            .unwrap_or("");

        self.add_token(TokenType::String, Literal::String(String::from(value)))
    }

    fn is_digit(c: char) -> bool {
        c.is_ascii_digit()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap_or(' ')
    }

    fn parse_number(&mut self) {
        while Scanner::is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && Scanner::is_digit(self.peek_next()) {
            // Consume the "."
            self.advance();

            while Scanner::is_digit(self.peek()) {
                self.advance();
            }
        }

        let value = self.source.get(self.start..self.current).unwrap_or("");

        self.add_token(
            TokenType::Number,
            Literal::Number(value.parse::<f64>().unwrap_or(0.0)),
        );
    }

    fn is_alphabetic(c: char) -> bool{
        c.is_ascii_alphabetic() || c == '_'
    }

    fn is_alphanumeric(c: char) -> bool{
        Scanner::is_alphabetic(c) || Scanner::is_digit(c)
    }

    fn find_keyword(text: &str) -> Option<TokenType>{
        match text {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "for" => Some(TokenType::For),
            "fun" => Some(TokenType::Fun),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::For),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "var" => Some(TokenType::Var),
            "while" => Some(TokenType::While),
            _ => None
        }
    }

    fn parse_identifier(&mut self){
        while Scanner::is_alphanumeric(self.peek()) {self.advance();}
        let value = self.source.get(self.start..self.current).unwrap_or("");
        println!("{}",value);
        let option = Scanner::find_keyword(value);
        let t_type = match option {
            Some(t_type) => t_type,
            None => TokenType::Identifier
        };

        self.add_token(t_type, Literal::Empty)
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
                if self.check('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, Literal::Empty)
                }
            }
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '"' => self.parse_string(),
            other => {
                if Scanner::is_digit(other) {
                    self.parse_number()
                } else if Scanner::is_alphabetic(other) {
                    self.parse_identifier()
                } else {
                    self.add_token(TokenType::Eof, Literal::Empty)
                }
            }
        };
    }
}
