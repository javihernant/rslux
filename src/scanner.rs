use crate::token::{Token, TokenType};
use std::str::FromStr;

pub struct Scanner {
    source: StringIter,
    tokens: Vec<Token>,
    had_error: bool,
}

struct StringIter {
    string: String,
    start: usize,
    current: usize,
    line: usize,
}

impl StringIter {
    pub fn new(string: String) -> StringIter {
        StringIter { 
            string, 
            start: 0, 
            current: 0,
            line: 1 
        }
    }

    pub fn next(&mut self) -> Option<char> {
        let elem = self.string[self.current..].chars().next();
        if let Some(elem) = elem {
            self.current += elem.len_utf8();
        }
        elem
    }

    pub fn peek(&self) -> Option<char> {
        self.string[self.current..].chars().next()
    }

    pub fn peek_next(&self) -> Option<char> {
        let mut it = self.string[self.current..].chars();
        let _ = it.next();
        it.next()
    }

    pub fn update_start(&mut self) {
        self.start = self.current;
    }

    pub fn count_line(&mut self) {
        self.line += 1;
    }

    pub fn slice(&self) -> Option<&str> {
        self.string.get(self.start..self.current)
    }
}

impl Scanner{
    pub fn new(source: String) -> Scanner {
        let mut scanner = Scanner {
            source: StringIter::new(source),
            tokens: Vec::new(),
            had_error: false,
        };
        Self::populate_tokens(&mut scanner);
        scanner
    }

    fn populate_tokens(&mut self) {
        while self.source.peek().is_some() {
            //we are at the beginning of the next lexeme
            self.source.update_start();
            self.scan_token();
        }
        // let end_token = Token::new(TokenType::EOF, None, None, line);
        // self.tokens.push(end_token);
    }

    pub fn tokens(&mut self) -> &[Token] {
        self.tokens.as_slice()
    }

    fn scan_token(&mut self) {
        if let Some(ch) = self.source.next() {
            let token_type = match ch {
                '(' => Some(TokenType::LeftParen),
                ')' => Some(TokenType::RightParent),
                '{' => Some(TokenType::LeftBrace),
                '}' => Some(TokenType::RightBrace),
                ',' => Some(TokenType::Comma),
                '.' => Some(TokenType::Dot),
                '-' => Some(TokenType::Minus),
                '+' => Some(TokenType::Plus),
                ';' => Some(TokenType::Semicolon),
                '*' => Some(TokenType::Star),
                '!' => {
                    match self.source.peek() {
                        Some('=') => {
                            let _ = self.source.next();
                            Some(TokenType::BangEqual)
                        },
                        _ => Some(TokenType::Bang),
                    }
                },
                '=' => {
                    match self.source.peek() {
                        Some('=') => {
                            let _ = self.source.next();
                            Some(TokenType::EqualEqual)
                        },
                        _ => Some(TokenType::Equal),
                    }
                },
                '<' => {
                    match self.source.peek() {
                        Some('=') => {
                            let _ = self.source.next();
                            Some(TokenType::LessEqual)
                        },
                        _ => Some(TokenType::Less),
                    }
                },
                '>' => {
                    match self.source.peek() {
                        Some('=') => {
                            let _ = self.source.next();
                            Some(TokenType::GreaterEqual)
                        },
                        _ => Some(TokenType::Greater),
                    }
                },
                '/' => {
                    match self.source.peek() {
                        Some('/') => {
                            while let Some(ch) = self.source.next() {
                                if ch == '\n' {
                                    break
                                }
                            }
                            None
                        },
                        _ => Some(TokenType::Slash)
                    }
                },
                ' ' | '\r' | '\t' => {
                    None
                },
                '\n' => {
                    self.source.count_line();
                    None
                },
                '\"' => {
                    self.scan_string()
                },
                c if c.is_ascii_digit() => {
                    self.scan_number()
                },
                c if c.is_alphabetic() || c == '_' => {
                    self.scan_ident()
                },
                _ => {
                    self.error("Unexpected character.");
                    None
                }
            };

            if let Some(token_type) = token_type {
                self.add_token(token_type);
            }
        }

    }

    fn scan_string(&mut self) -> Option<TokenType> {

        loop {
            match self.source.next() {
                Some('\n') => {
                    self.source.count_line();
                    self.error("Expected a terminating \"");
                    return None
                },
                Some('"') => {
                    let slice = self.source.slice().expect("Slice returned isn't valid");
                    let slice = &slice[1..slice.len() - 1];
                    return Some(TokenType::String(slice.to_string()))
                }
                Some(_) => continue,
                None => {
                    self.error("Expected a terminating \"");
                    return None
                }
            }
        }
    }

    fn scan_number(&mut self) -> Option<TokenType>{
        while let Some(c) = self.source.peek() {
            if c.is_ascii_digit() {
                let _ = self.source.next();
                continue;
            } else {
                break;
            }
        }
        
        if let (Some('.'), Some('0'..='9')) = (self.source.peek(), self.source.peek_next()) {
            let _ = self.source.next();
            while let Some(c) = self.source.peek() {
                match c {
                    '0'..='9' => {
                        let _ = self.source.next();
                    },
                    _ => break,
                }   
            }
        }
        let slice = self.source.slice().expect("Got invalid slice at scanning number");
        let num = f64::from_str(slice).expect("Couldnt parse invalid number");
        Some(TokenType::Number(num))
    }

    fn scan_ident(&mut self) -> Option<TokenType> {
        while let Some(c) = self.source.peek() {
            if c.is_alphanumeric() {
                let _ = self.source.next();
            } else {
                break ;
            }
        }
        let slice = self.source.slice().expect("Couldn't get identifier slice");
        let token = TokenType::from_ident(slice.to_string());
        Some(token)
    }

    fn error(&mut self, messg: &str) {
        self.had_error = true;
        eprintln!("Error: {}",messg);
    }

    fn add_token(&mut self, token_type: TokenType) {
        let token = Token::new(token_type);
        self.tokens.push(token);
    }
}

// #[derive(Debug)]
// pub struct ScanError {
//     line: i32,
//     location: Option<String>,
//     messg: Option<String>,
// }

// Impl's for ScanError

// impl std::error::Error for ScanError { }

// impl fmt::Display for ScanError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "[line {}]", self.line)?;
//         if let Some(location) = self.location {
//             write!(f, "At {location}")?;
//         }
//         if let Some(messg) = self.messg {
//             write!(f, ": {messg}")?;
//         }
//         Ok(())
//     }
// }