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

    pub fn to_string(&self) -> String {
        format!(
            "type: {:#?}, lexeme: {}, literal:{:#?}",
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
            line: 1
        }
    }

    pub fn scan_tokens(&self) {

        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            scanToken();
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            String::from(""),
            Literal::Empty,
            self.line,
        ));
    }

    fn is_at_end(&self) -> bool{
        return self.current >= self.source.len();
    }

    fn advance(&self) -> char{
        let c = self.source.chars().nth(self.current).unwrap_or(' ');
        self.current+=1;
        c
    }
}
