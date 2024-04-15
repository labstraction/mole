use self::model::{Literal, ScanResult, Token, TokenType};

pub mod model;

pub fn scan(source: &str) -> Result<Vec<Token>, (usize, String)>{

    let mut start;
    let mut current: usize = 0;
    let mut line: usize = 1;
    let mut tokens: Vec<Token> = Vec::new();

    println!("debug {:#?}", tokens);

    while !is_at_end(current, source.len()) {

        start = current;
        match scan_token(source, start, current, line){
            ScanResult::Ok((token, new_current, new_line)) => {
                current = new_current;
                line = new_line;
                tokens.push(token);
                println!("debug1 {:#?}", tokens);
            },
            ScanResult::Skip((new_current, new_line)) => {
                current = new_current;
                line = new_line;
                println!("debug2 {:#?}", tokens);
            },
            ScanResult::Err(message) => return Err(message)
        };
        
    }

    tokens.push(Token::new(
        TokenType::Eof,
        String::from(""),
        Literal::Empty,
        line,
    ));

    Ok(tokens)
}

fn is_at_end(current: usize, len: usize) -> bool {
    current >= len
}

fn scan_token(source: &str, start:usize, current: usize, line:usize) -> ScanResult<(Token, usize, usize), (usize, usize), (usize, String)>{
    let mut  new_current = current;
    let mut new_line= line;
    let selected_char = get_char(source, current);
    new_current += 1;
    println!("debug3 {:#?}", selected_char);
    match selected_char {
        '(' => ScanResult::Ok((create_token(source, start, new_current, new_line, TokenType::LeftParen, Literal::Empty), new_current, new_line)),
        ')' => ScanResult::Ok((create_token(source, start, new_current, new_line, TokenType::RightParen, Literal::Empty), new_current, new_line)),
        '{' => ScanResult::Ok((create_token(source, start, new_current, new_line, TokenType::LeftBrace, Literal::Empty), new_current, new_line)),
        '}' => ScanResult::Ok((create_token(source, start, new_current, new_line, TokenType::RightBrace, Literal::Empty), new_current, new_line)),
        ',' => ScanResult::Ok((create_token(source, start, new_current, new_line, TokenType::Comma, Literal::Empty), new_current, new_line)),
        '.' => ScanResult::Ok((create_token(source, start, new_current, new_line, TokenType::Dot, Literal::Empty), new_current, new_line)),
        '-' => ScanResult::Ok((create_token(source, start, new_current, new_line, TokenType::Minus, Literal::Empty), new_current, new_line)),
        '+' => ScanResult::Ok((create_token(source, start, new_current, new_line, TokenType::Plus, Literal::Empty), new_current, new_line)),
        ';' => ScanResult::Ok((create_token(source, start, new_current, new_line, TokenType::Semicolon, Literal::Empty), new_current, new_line)),
        '*' => ScanResult::Ok((create_token(source, start, new_current, new_line, TokenType::Star, Literal::Empty), new_current, new_line)),
        '!' => {
            if check('=', source, new_current) {
                new_current+=1;
                ScanResult::Ok((create_token(source, start, new_current, new_line, TokenType::BangEqual, Literal::Empty), new_current, new_line))
            } else {
                ScanResult::Ok((create_token(source, start, new_current, new_line, TokenType::Bang, Literal::Empty), new_current, new_line))
            }
        }
        '=' => {
            if check('=', source, new_current) {
                new_current+=1;
                ScanResult::Ok((create_token(source, start, new_current, new_line, TokenType::EqualEqual, Literal::Empty), new_current, new_line))
            } else {
                ScanResult::Ok((create_token(source, start, new_current, new_line, TokenType::Equal, Literal::Empty), new_current, new_line))
            }
        }
        '<' => {
            if check('=', source, new_current) {
                new_current+=1;
                ScanResult::Ok((create_token(source, start, new_current, new_line, TokenType::LessEqual, Literal::Empty), new_current, new_line))
            } else {
                ScanResult::Ok((create_token(source, start, new_current, new_line, TokenType::Less, Literal::Empty), new_current, new_line))
            }
        }
        '>' => {
            if check('=', source, new_current) {
                new_current+=1;
                ScanResult::Ok((create_token(source, start, new_current, new_line, TokenType::GreaterEqual, Literal::Empty), new_current, new_line))
            } else {
                ScanResult::Ok((create_token(source, start, new_current, new_line, TokenType::Greater, Literal::Empty), new_current, new_line))
            }
        }
        '/' => {
            if check('/', source, new_current) {
                new_current+=1;
                while peek(source, new_current) != '\n' && !is_at_end(new_current, source.len()) {
                    new_current+=1;
                }
                ScanResult::Skip((new_current, new_line))
            } else {
                ScanResult::Ok((create_token(source, start, new_current, new_line, TokenType::Slash, Literal::Empty), new_current, new_line))
            }
        }
        ' ' | '\r' | '\t' => ScanResult::Skip((new_current, new_line)),
        '\n' => {
            new_line += 1;
            ScanResult::Skip((new_current, new_line))
        },
        '"' => match parse_string(source, start, new_current, new_line,) {
            Ok((token, current, line)) => {
                new_current = current;
                new_line = line;
                ScanResult::Ok((token, new_current, new_line))
            },
            Err((message, _, line)) => {
                ScanResult::Err((line, message))
            }
        },
        other => {
            if is_digit(other) {
                let res = parse_number(source, start, new_current, new_line,);
                new_current = res.1;
                ScanResult::Ok((res.0, new_current, new_line))
            } else if is_alphabetic(other) {
                println!("debug4 {:#?}", other);
                let res = parse_identifier(source, start, new_current, new_line,);
                new_current = res.1;
                println!("debug4 {:#?}", res.0);
                ScanResult::Ok((res.0, new_current, new_line))
            } else {
                return ScanResult::Err((new_line, String::from("invalid char")));
            }
        }
    }
}



fn get_char(source: &str, index: usize) -> char{
    source.chars().nth(index).unwrap_or(' ')
}

fn create_token(source:&str, start:usize, current: usize, line:usize, t_type: TokenType, literal: Literal)->Token{
    let text = &source[start..current];
    Token::new(t_type, String::from(text), literal, line)
}

fn check(expected: char, source: &str, current: usize) -> bool {
    if is_at_end(current, source.len()) {
        return false;
    }
    if source.chars().nth(current).unwrap_or(' ') != expected {
        return false;
    }
    true
}

fn peek(source: &str, current: usize) -> char {
    if is_at_end(current, source.len()) {
        return '\0';
    };
    get_char(source, current)
}

fn parse_string(source: &str, start: usize, current: usize, line: usize) -> Result<(Token, usize, usize), (String, usize, usize)>{
    let mut new_line = line;
    let mut new_current = current;

    while peek(source, new_current) != '"' && !is_at_end(new_current, source.len()) {
        if peek(source, new_current) == '\n' {
            new_line += 1;
        }
        new_current+=1;
    }

    if is_at_end(new_current, source.len()) {
        return Err((String::from("Unterminated string"), new_current, new_line))
    }

    // The closing ".
    new_current+=1;

    // Trim the surrounding quotes.
    let value = 
        source
        .get(start + 1..new_current - 1)
        .unwrap_or("");

    Ok((
        create_token(source, start, new_current, new_line,TokenType::String, Literal::String(String::from(value))),
        new_current,
        new_line
    ))
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn peek_next(source: &str, current: usize) -> char {
    if current + 1 >= source.len() {
        return '\0';
    }
    let next = current + 1;
    get_char(source, next)
}

fn parse_number(source: &str, start: usize, current: usize, line: usize) -> (Token, usize) {
    let mut new_current = current;
    while is_digit(peek(source, new_current)) {
        new_current += 1;
    }

    if peek(source, new_current) == '.' && is_digit(peek_next(source, new_current)) {
        // Consume the "."
        new_current += 1;

        while is_digit(peek(source, new_current)) {
            new_current += 1;
        }
    }

    let value = source.get(start..new_current).unwrap_or("");

    let token = create_token(
        source, start, new_current, line,
        TokenType::Number,
        Literal::Number(value.parse::<f64>().unwrap_or(0.0)),
    );

    (token, new_current)
}

fn is_alphabetic(c: char) -> bool{
    c.is_ascii_alphabetic() || c == '_'
}

fn is_alphanumeric(c: char) -> bool{
    is_alphabetic(c) || is_digit(c)
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
        "print" => Some(TokenType::Print),
        "return" => Some(TokenType::Return),
        "super" => Some(TokenType::Super),
        "this" => Some(TokenType::This),
        "true" => Some(TokenType::True),
        "var" => Some(TokenType::Var),
        "while" => Some(TokenType::While),
        _ => None
    }
}

fn parse_identifier(source: &str, start: usize, current: usize, line: usize) -> (Token, usize){
    let mut new_current = current;
    while is_alphanumeric(peek(source,new_current)) {
        new_current += 1;
    }
    let value = source.get(start..new_current).unwrap_or("");
    let option = find_keyword(value);
    let t_type = match option {
        Some(t_type) => t_type,
        None => TokenType::Identifier
    };
    let token = create_token(source, start, new_current, line, t_type, Literal::Empty);

    (token, new_current)
}



