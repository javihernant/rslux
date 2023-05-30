//TODO: Change name of the module to ast

pub enum Stmt {
    Print(Expr),
    Expr(Expr),
    Var {
        // name: Token,
        name: String,
        initializer: Option<Expr>,
    },
    Block(Vec<Stmt>),
}

use crate::token::Token;
use crate::value::Value;

pub enum Expr {
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    Grouping (Box<Expr>),
    Literal (Value),
    Variable (Token),
    Unary {
        op: Token,
        right: Box<Expr>,
    }
}

// impl Display for Expr {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Expr::Binary { left, op, right } => write!(f,"( {op} {left} {right} )"),
//             Expr::Grouping(e) => write!(f, "( Grouping {e} )"),
//             Expr::Literal(l) => write!(f, "{l}"),
//             Expr::Unary { op, right } => write!(f, "( {op} {right} )"),
//             Expr::Variable(a) => write!(f,"( VAR {})", a.lexeme()),
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use crate::token::{Token, TokenKind};
    use super::*;

    // fn gen_expr() -> Expr {
    //     TokenKind::Number(45.67);
    //     Expr::Binary {
    //         left: Box::new(Expr::Grouping(Box::new(Expr::Unary{
    //             op: Token::new(TokenKind::Minus, 42),
    //             right: Box::new(Expr::Literal(Token::new(TokenKind::Number(123.), 42)))
    //         }))),
    //         op: Token::new(TokenKind::Star, 42),
    //         right: Box::new(Expr::Grouping(Box::new(Expr::Literal(Token::new(TokenKind::Number(45.67), 42)))))
    //     }
    // }

    // #[test]
    // fn test_display_expr() {
    //     let expr = gen_expr();
    //     println!("{}", expr)
    // }
}