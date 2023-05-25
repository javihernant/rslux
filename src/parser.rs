use crate::expr::{Expr, ExprErr};
use crate::token::{Token, TokenKind};
use crate::value::Value;

pub struct Parser {
    tokens: Vec<Token>,
    current_idx: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            current_idx: 0,
        }
    }

    pub fn build_tree(&mut self) -> Expr {
        match self.expr() {
            Ok(expr) => *expr,
            Err(e) => {
                eprintln!("Error: {e}");
                panic!();
            }
        }
    }

    fn expr(&mut self) -> Result<Box<Expr>, ExprErr> {
        Ok(self.equality()?)
    }

    fn equality(&mut self) -> Result<Box<Expr>, ExprErr> {
        let mut expr = self.comparison()?;
        while self.match_token(&[TokenKind::BangEqual, TokenKind::EqualEqual]) {
            let op = self.next().unwrap().clone();
            let right = self.comparison()?;
            expr = Box::new(Expr::Binary { left: expr, op, right });
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Box<Expr>, ExprErr> {
        let mut expr = self.term()?;

        while self.match_token(&[TokenKind::Greater, TokenKind::GreaterEqual, TokenKind::Less, TokenKind::LessEqual]) {
            let op = self.next().unwrap().clone();
            let right = self.term()?;
            expr = Box::new(Expr::Binary { left: expr, op, right });
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Box<Expr>, ExprErr> {
        let mut expr = self.factor()?;

        while self.match_token(&[TokenKind::Minus, TokenKind::Plus]) {
            let op = self.next().unwrap().clone();
            let right = self.factor()?;
            expr = Box::new(Expr::Binary { left: expr, op, right });
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Box<Expr>, ExprErr> {
        let mut expr = self.unary()?;

        while self.match_token(&[TokenKind::Slash, TokenKind::Star]) {
            let op = self.next().unwrap().clone();
            let right = self.unary()?;
            expr = Box::new(Expr::Binary { left: expr, op, right });
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Box<Expr>, ExprErr> {
        if self.match_token(&[TokenKind::Minus, TokenKind::Bang]) {
            let op = self.next().unwrap().clone();
            let right = self.unary()?;
            return Ok(Box::new(Expr::Unary { op, right }))
        }

        Ok(self.primary()?)
    }

    fn primary(&mut self) -> Result<Box<Expr>, ExprErr> {
        let tk = self.peek().expect("no more tokens").clone();
        match tk.kind() {
            TokenKind::False | TokenKind::True | TokenKind::Nil | TokenKind::Number | TokenKind::String => {
                let _ = self.next();
                Ok(Box::new(Expr::Literal(tk.literal().unwrap().clone())))
            },
            TokenKind::LeftParen => { 
                let _ = self.next();
                let expr = self.expr()?;
                if let Some(tk) = self.peek() {
                    if tk.kind() != &TokenKind::RightParent {
                        return Err(ExprErr::new(tk,"Expected a ')' token"))
                    }
                    let _ = self.next();
                } else {
                    panic!("No EOL token was found");
                }
                
                Ok(Box::new(Expr::Grouping(expr)))
            },
            _ => panic!("Wrong primary expression"),
        }
        
    }

    fn peek(&self) -> Option<&Token>{
        self.tokens.get(self.current_idx)
    }

    fn next(&mut self) -> Option<&Token>{
        let el = self.tokens.get(self.current_idx);
        if el.is_some() {
            self.current_idx += 1;
        }
        el
    }

    fn match_token(&self, token_types: &[TokenKind]) -> bool {
        let curr = self.peek();
        if let Some(curr) = curr {
            let curr = curr.kind();
            for kind in  token_types {
                if curr == kind {
                    return true;
                }
            }
        }
        false
    }

    fn sync(&mut self) {
        while let Some(tk) = self.peek() {
            match tk.kind() {
                TokenKind::Semicolon => { 
                    let _ = self.next();
                    break;
                },
                TokenKind::Class | TokenKind::Fun | TokenKind::Var | TokenKind::For 
                | TokenKind::If | TokenKind::While | TokenKind::Print | TokenKind::Return => { 
                    break;
                },
                _ => {
                    let _ = self.next();
                    continue;
                }
            }
        }
    }
}