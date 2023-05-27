use crate::expr::{Expr, ExprErr};
use crate::token::{Token, TokenKind};
use crate::value::Value;
use crate::expr::{Stmt, StmtErr};

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

    // pub fn build_tree(&mut self) -> Expr {

    //     match self.expr() {
    //         Ok(expr) => *expr,
    //         Err(e) => {
    //             eprintln!("Error: {e}");
    //             panic!();
    //         }
    //     }
    // }

    pub fn stmts(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while let Some(tk) = self.peek() {
            if tk.kind() == &TokenKind::Eof {
                let _ = self.next();
                break
            }
            match self.declaration() {
                Ok(stmt) => {
                    stmts.push(stmt);
                },
                Err(e) => {
                    eprintln!("{e}")
                }
            }
        }
        stmts
    }

    fn declaration(&mut self) -> Result<Stmt, StmtErr>{
        let curr_tk = self.peek().expect("couldnt get token");
        match curr_tk.kind() {
            TokenKind::Var => {
                let _ = self.next();
                self.var_decl()
            },
            _ => { 
                self.stmt()
            },
        } 
    }

    //var name (= value);
    fn var_decl(&mut self) -> Result<Stmt, StmtErr> {
        
        let tk = self.peek().expect("couldnt get token");
        let name = match tk.kind() {
            TokenKind::Identifier(name) => {
                let name = name.to_string();
                let _ = self.next();
                name
            },
            _ => return Err(StmtErr::InvalidStmt("expected a variable name".to_string(), tk.clone())),
        };

        // let name = {
        //     let tk = self.peek().expect("couldnt get token").clone();
        //     match self.peek().expect("couldnt get token").kind() {
        //         TokenKind::Identifier(_) => {
        //             let _ = self.next();
        //             tk
        //         },
        //         _ => return Err(StmtErr::InvalidStmt("expected a variable name".to_string(), tk))
        //     }
        // };
      

        let initializer = match self.peek().expect("couldnt get token").kind() {
            TokenKind::Equal => {
                let _ = self.next();
                let expr = *self.expr()?;
                Some(expr)
            },
            _ => None
        };

        let tk = self.peek().expect("couldnt get token");
        if let TokenKind::Semicolon = tk.kind() {
            let _ = self.next();
            Ok(
                Stmt::Var { 
                name,
                initializer,
            })
        }
        else {
            return Err(StmtErr::InvalidStmt("Expected ';'".to_string(), tk.clone()));
        }
    }

    fn stmt(&mut self) -> Result<Stmt, StmtErr>{
        let curr_tk = self.peek().expect("couldnt get token");
        match curr_tk.kind() {
            TokenKind::Print => {
                let _ = self.next();
                self.print_stmt()
            },
            _ => { 
                self.expr_stmt()
            },
        }  
    }

    fn expr_stmt(&mut self) -> Result<Stmt, StmtErr> {
        let expr = self.expr()?;

        if let Some(tk) = self.peek() {
            if let TokenKind::Semicolon = tk.kind() {
                let _ = self.next();
                // println!("Expr: {expr}");
                return Ok(Stmt::Expr(*expr));
            } else {
                return Err(StmtErr::InvalidStmt("Expected ';'".to_string(), tk.clone()));
            }
        } else {
            panic!("No EOF was found")
        }
    }

    fn print_stmt(&mut self) -> Result<Stmt, StmtErr> {
        let expr = self.expr()?;
        
        if let Some(tk) = self.peek() {
            if let TokenKind::Semicolon = tk.kind() {
                let _ = self.next();
                // println!("Print: {expr}");
                return Ok(Stmt::Print(*expr));
            } else {
                return Err(StmtErr::InvalidStmt("Expected ';'".to_string(), tk.clone()));
            }
        } else {
            panic!("No EOF was found")
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
            TokenKind::Identifier(_) => {
                let _ = self.next();
                Ok(Box::new(Expr::Variable(tk)))
            }
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