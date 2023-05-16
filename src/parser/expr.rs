use std::fmt::Display;

use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub struct ExprErr{
    token_kind: TokenKind,
    messg: String,
    line: usize,
    lexeme: String
}

impl ExprErr {
    pub fn new(tk: &Token, messg:&str) -> ExprErr {
        ExprErr {
            token_kind: tk.kind().clone(),
            messg: messg.to_string(),
            line: tk.line(),
            lexeme: tk.lexeme().to_string(),
        }
    }
}

impl Display for ExprErr{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.token_kind {
            TokenKind::Eof => {write!(f, "[line {}] at end: {}", self.line, self.messg)},
            _ => {write!(f, "[line {}] at '{}': {}", self.line, self.lexeme, self.messg)}
        }
    }
}
pub enum Expr {
    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    Grouping (Box<Expr>),
    Literal (Token),
    Unary {
        op: Token,
        right: Box<Expr>,
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary { left, op, right } => { write!(f,"[ {op} {left} {right} ]")?;},
            Expr::Grouping(e) => { write!(f, "[ Grouping {e} ]")?},
            Expr::Literal(l) => { write!(f, "{l}")?},
            Expr::Unary { op, right } => { write!(f, "[ {op} {right} ]")? },
        }
        Ok(())

    }
}

#[cfg(test)]
mod tests {
    use crate::token::{Token, TokenKind};
    use super::*;

    fn gen_expr() -> Expr {
        TokenKind::Number(45.67);
        Expr::Binary {
            left: Box::new(Expr::Grouping(Box::new(Expr::Unary{
                op: Token::new(TokenKind::Minus, 42),
                right: Box::new(Expr::Literal(Token::new(TokenKind::Number(123.), 42)))
            }))),
            op: Token::new(TokenKind::Star, 42),
            right: Box::new(Expr::Grouping(Box::new(Expr::Literal(Token::new(TokenKind::Number(45.67), 42)))))
        }
    }

    #[test]
    fn test_display_expr() {
        let expr = gen_expr();
        println!("{}", expr)
    }
}