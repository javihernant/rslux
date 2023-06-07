use crate::error::compiletime::ParseError;
use crate::expr::{Expr, Stmt};
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

    fn declaration(&mut self) -> Result<Stmt, ParseError>{
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
    fn var_decl(&mut self) -> Result<Stmt, ParseError> {
        
        let tk = self.peek().expect("couldnt get token");
        let name = match tk.kind() {
            TokenKind::Identifier(name) => {
                let name = name.to_string();
                let _ = self.next();
                name
            },
            _ => return Err(ParseError::new("expected a variable name", tk)),
        };

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
            return Err(ParseError::new("Expected ';'", tk));
        }
    }

    fn stmt(&mut self) -> Result<Stmt, ParseError>{
        let curr_tk = self.peek().expect("couldnt get token");
        match curr_tk.kind() {
            TokenKind::Print => {
                let _ = self.next();
                self.print_stmt()
            },
            TokenKind::LeftBrace => {
                let _ = self.next();
                Ok(Stmt::Block(self.block()?))
            },
            TokenKind::If => {
                let _ = self.next();
                self.if_stmt()
            },
            TokenKind::While => {
                let _ = self.next();
                self.while_stmt()
            },
            TokenKind::For => {
                let _ = self.next();
                self.for_stmt()
            }
            _ => { 
                self.expr_stmt()
            },
        }  
    }

    fn for_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenKind::LeftParen, "Expected '(' after 'for'.")?;
        let initializer = match self.peek().unwrap().kind() {
            TokenKind::Semicolon => {
                None
            },
            TokenKind::Var => {
                let _ = self.next();
                Some(self.var_decl()?)
            },
            _ => {
                Some(self.expr_stmt()?)
            }
        };
        let condition = {
            if let TokenKind::Semicolon = self.peek().unwrap().kind() {
                Expr::Literal(Value::Bool(true))
            } else {
                *self.expr()?
            }
        };
        self.consume(TokenKind::Semicolon, "Expecting ';' after loop condition")?;
        let increment = {
            if let TokenKind::Semicolon = self.peek().unwrap().kind() {
                None
            } else {
                Some(self.expr()?)
            }
        };
        self.consume(TokenKind::RightParent, "Expecting ')' after for clauses")?;
        let mut body =self.stmt()?;
        if let Some(increment) = increment {
            body = Stmt::Block( vec![body, Stmt::Expr(*increment)]);
        }
        body = Stmt::While { condition, body:Box::new(body)};
        if let Some(initializer) = initializer {
            body = Stmt::Block(vec![initializer,body]);
        }
        Ok(body)
    }

    fn while_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenKind::LeftParen, "Expecting '(' after 'while'")?;
        let cond = self.expr()?;
        self.consume(TokenKind::RightParent, "Expecting ')' after condition")?;
        let body = self.stmt()?;
        Ok(Stmt::While { condition: *cond, body: Box::new(body) })
    }

    fn if_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenKind::LeftParen, "Expect '(' after 'if'")?;
        let condition = self.expr()?;
        self.consume(TokenKind::RightParent, "Expect ')' after if condition")?;
        let then_br = self.stmt()?;
        let else_br = {
            if let TokenKind::Else = self.peek().unwrap().kind() {
                let _ = self.next();
                Some(Box::new(self.stmt()?))
            } else {
                None
            }
        };
        Ok(Stmt::If { condition: *condition, then_br: Box::new(then_br), else_br })
    }

    fn consume(&mut self, tkind: TokenKind, messg:&str) -> Result<(), ParseError> {
        if tkind == *self.peek().unwrap().kind() {
            let _ = self.next();
            Ok(())
        } else {
            Err(ParseError::new(messg, self.peek().unwrap()))
        }
    }

    fn block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut stmts = Vec::new();
        while let Some(tk) = self.peek() {
            match tk.kind() {
                TokenKind::Eof | TokenKind::RightBrace => break,
                _ => stmts.push(self.declaration()?)
                
            }
        }

        if self.peek().is_none() {
            panic!("Eof not found")
        }

        if let TokenKind::RightBrace = self.peek().unwrap().kind() {
            let _ = self.next();
            Ok(stmts)
        } else {
            Err(ParseError::new("Expecting '}'", self.peek().unwrap()))
        }
    }

    fn expr_stmt(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expr()?;

        if let Some(tk) = self.peek() {
            if let TokenKind::Semicolon = tk.kind() {
                let _ = self.next();
                // println!("Expr: {expr}");
                return Ok(Stmt::Expr(*expr));
            } else {
                return Err(ParseError::new("Expected ';'", tk));
            }
        } else {
            panic!("No EOF was found")
        }
    }

    fn print_stmt(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expr()?;
        
        if let Some(tk) = self.peek() {
            if let TokenKind::Semicolon = tk.kind() {
                let _ = self.next();
                // println!("Print: {expr}");
                return Ok(Stmt::Print(*expr));
            } else {
                return Err(ParseError::new("Expected ';'", tk));
            }
        } else {
            panic!("No EOF was found")
        }
    }

    fn expr(&mut self) -> Result<Box<Expr>, ParseError> {
        Ok(self.assignment()?)
    }

    fn assignment(&mut self) -> Result<Box<Expr>, ParseError> {
        let expr = self.or()?;
        if self.match_token(&[TokenKind::Equal]) {
            let equals = self.next().expect("Couldnt get token");
            if let Expr::Variable(name) = *expr {
                let value = self.assignment()?;
                return Ok(Box::new(Expr::Assign { name, value }))
            } else {
                return Err(ParseError::new("Invalid assignment target", equals))
            }
        }
        Ok(expr)
    }

    fn or(&mut self) -> Result<Box<Expr>, ParseError> {
        let mut expr = self.and()?;
        while let TokenKind::Or = self.peek().unwrap().kind() {
            let op = self.next().unwrap().clone();
            let right = self.and()?;
            expr = Box::new(Expr::Logical { left: expr, op, right });
        }
        Ok(expr)
    }

    fn and(&mut self) -> Result<Box<Expr>, ParseError> {
        let mut expr = self.equality()?;
        while let TokenKind::And = self.peek().unwrap().kind() {
            let op = self.next().unwrap().clone();
            let right = self.equality()?;
            expr = Box::new(Expr::Logical { left: expr, op, right });
        }
        Ok(expr)
    }


    fn equality(&mut self) -> Result<Box<Expr>, ParseError> {
        let mut expr = self.comparison()?;
        while self.match_token(&[TokenKind::BangEqual, TokenKind::EqualEqual]) {
            let op = self.next().unwrap().clone();
            let right = self.comparison()?;
            expr = Box::new(Expr::Binary { left: expr, op, right });
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Box<Expr>, ParseError> {
        let mut expr = self.term()?;

        while self.match_token(&[TokenKind::Greater, TokenKind::GreaterEqual, TokenKind::Less, TokenKind::LessEqual]) {
            let op = self.next().unwrap().clone();
            let right = self.term()?;
            expr = Box::new(Expr::Binary { left: expr, op, right });
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Box<Expr>, ParseError> {
        let mut expr = self.factor()?;

        while self.match_token(&[TokenKind::Minus, TokenKind::Plus]) {
            let op = self.next().unwrap().clone();
            let right = self.factor()?;
            expr = Box::new(Expr::Binary { left: expr, op, right });
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Box<Expr>, ParseError> {
        let mut expr = self.unary()?;

        while self.match_token(&[TokenKind::Slash, TokenKind::Star]) {
            let op = self.next().unwrap().clone();
            let right = self.unary()?;
            expr = Box::new(Expr::Binary { left: expr, op, right });
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Box<Expr>, ParseError> {
        if self.match_token(&[TokenKind::Minus, TokenKind::Bang]) {
            let op = self.next().unwrap().clone();
            let right = self.unary()?;
            return Ok(Box::new(Expr::Unary { op, right }))
        }

        Ok(self.primary()?)
    }

    fn primary(&mut self) -> Result<Box<Expr>, ParseError> {
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
                        return Err(ParseError::new("Expected a ')' token", &tk))
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