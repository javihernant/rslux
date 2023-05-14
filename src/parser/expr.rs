use std::fmt::Display;

use crate::token::{Token};

#[derive(Debug)]
pub struct ExprErr(pub &'static str);
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
                op: Token::new(TokenKind::Minus),
                right: Box::new(Expr::Literal(Token::new(TokenKind::Number(123.))))
            }))),
            op: Token::new(TokenKind::Star),
            right: Box::new(Expr::Grouping(Box::new(Expr::Literal(Token::new(TokenKind::Number(45.67))))))
        }
    }

    #[test]
    fn test_display_expr() {
        let expr = gen_expr();
        println!("{}", expr)
    }
}