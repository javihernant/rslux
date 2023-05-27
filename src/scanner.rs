use crate::{token::{Token, TokenKind}, value::Value};
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
        self.tokens.push(self.produce_token(TokenKind::Eof));
    }

    pub fn tokens(&mut self) -> &[Token] {
        self.tokens.as_slice()
    }

    fn scan_token(&mut self) {
        if let Some(ch) = self.source.next() {
            let token = match ch {
                '(' => Some(self.produce_token(TokenKind::LeftParen)),
                ')' => Some(self.produce_token(TokenKind::RightParent)),
                '{' => Some(self.produce_token(TokenKind::LeftBrace)),
                '}' => Some(self.produce_token(TokenKind::RightBrace)),
                ',' => Some(self.produce_token(TokenKind::Comma)),
                '.' => Some(self.produce_token(TokenKind::Dot)),
                '-' => Some(self.produce_token(TokenKind::Minus)),
                '+' => Some(self.produce_token(TokenKind::Plus)),
                ';' => Some(self.produce_token(TokenKind::Semicolon)),
                '*' => Some(self.produce_token(TokenKind::Star)),
                '!' => {
                    match self.source.peek() {
                        Some('=') => {
                            let _ = self.source.next();
                            Some(self.produce_token(TokenKind::BangEqual))
                        },
                        _ => Some(self.produce_token(TokenKind::Bang)),
                    }
                },
                '=' => {
                    match self.source.peek() {
                        Some('=') => {
                            let _ = self.source.next();
                            Some(self.produce_token(TokenKind::EqualEqual))
                        },
                        _ => Some(self.produce_token(TokenKind::Equal)),
                    }
                },
                '<' => {
                    match self.source.peek() {
                        Some('=') => {
                            let _ = self.source.next();
                            Some(self.produce_token(TokenKind::LessEqual))
                        },
                        _ => Some(self.produce_token(TokenKind::Less)),
                    }
                },
                '>' => {
                    match self.source.peek() {
                        Some('=') => {
                            let _ = self.source.next();
                            Some(self.produce_token(TokenKind::GreaterEqual))
                        },
                        _ => Some(self.produce_token(TokenKind::Greater)),
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
                        _ => Some(self.produce_token(TokenKind::Slash))
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

            if let Some(mut token) = token {
                self.tokens.push(token);
            }
        }

    }

    fn produce_token(&self, tkind: TokenKind) -> Token {
        let literal = match tkind {
            TokenKind::True => Some(Value::Bool(true)),
            TokenKind::False => Some(Value::Bool(false)),
            _ => None,
        };
        let lexeme = self.source.slice().expect("Couldn't get lexeme").to_string();
        Token::new(tkind, literal, lexeme, self.source.line)
    }

    fn scan_string(&mut self) -> Option<Token> {

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
                    let literal = Some(Value::String(slice.to_string()));
                    let token = Token::new(TokenKind::String, literal, slice.to_string(), self.source.line);
                    return Some(token)
                }
                Some(_) => continue,
                None => {
                    self.error("Expected a terminating \"");
                    return None
                }
            }
        }
    }

    fn scan_number(&mut self) -> Option<Token>{
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
        let token = Token::new(TokenKind::Number, Some(Value::Number(num)),slice.to_string(),self.source.line);
        Some(token)
    }

    fn scan_ident(&mut self) -> Option<Token> {
        while let Some(c) = self.source.peek() {
            if c.is_alphanumeric() {
                let _ = self.source.next();
            } else {
                break ;
            }
        }
        let slice = self.source.slice().expect("Couldn't get identifier slice");
        let tkind = TokenKind::from_ident(slice);
        Some(self.produce_token(tkind))
    }

    fn error(&mut self, messg: &str) {
        self.had_error = true;
        eprintln!("Error: {}",messg);
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