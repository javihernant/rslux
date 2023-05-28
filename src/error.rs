pub mod compiletime {
    use std::fmt::Display;

    use crate::token::{Token, TokenKind};

    pub struct ParseError {
        token: Token,
        messg: String,
    }
    
    impl ParseError {
        pub fn new(messg: &str, token: &Token) -> ParseError {
            ParseError { token: token.clone(), messg: messg.to_string() }
        }
    }
    impl Display for ParseError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self.token.kind() {
                TokenKind::Eof => {write!(f, "[line {}] at end: {}", self.token.line(), self.messg)},
                _ => {write!(f, "[line {}] at '{}': {}", self.token.line(), self.token.lexeme(), self.messg)}
            }
            
        }
    }

}

pub mod runtime {
    use std::fmt::Display;

    use crate::token::Token;
    pub struct EvalError {
        messg: String,
        token: Token,
    }
    
    impl EvalError {
        pub fn new(token: &Token, messg: &str)->EvalError {
            EvalError {
                messg: messg.to_string(),
                token: token.clone(),
            }
        }
    }
    
    impl Display for EvalError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(f,"[line {}] at '{}': {}",self.token.line(),self.token.lexeme(), self.messg)
        }
    }
}