use std::collections::HashMap;
use std::cell::RefCell;

use crate::expr::Expr;
use crate::token::{TokenKind, Token};
use crate::error::runtime::EvalError;
use crate::value::Value;
use crate::expr::Stmt;

struct Environment {
    enclosing: Option<Box<Environment>>,
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            enclosing:None,
            values: HashMap::new(),
        }
    }

    pub fn push(self) -> Environment {
        Environment { 
            enclosing: Some(Box::new(self)), 
            values: HashMap::new(), 
        }
    }

    pub fn pop(self) -> Environment {
        *self.enclosing.unwrap()
    }
    pub fn define(&mut self, name: String, value:Value) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name_tk: &Token, value:Value) -> Result<(), EvalError> {
        let name = name_tk.lexeme();
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
            Ok(())
        } else {
            match &mut self.enclosing {
                Some(enclosing) => {
                    enclosing.assign(name_tk, value)?;
                    Ok(())
                },
                None => {
                    Err(EvalError::new("Variable is not defined", name_tk))
                }
            }
        }
    }

    pub fn get(&self, name_tk: &Token) -> Result<Value, EvalError> {
        match self.values.get(name_tk.lexeme()) {
            Some(v) => Ok(v.clone()),
            None => {
                match &self.enclosing {
                    Some(enclosing) => {
                        Ok(enclosing.get(name_tk)?)
                    },
                    None => Err(EvalError::new("Variable is not defined", name_tk))
                }
            }
        }
    }
}
pub struct Interpreter {
    environment: Option<Environment>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            environment: Some(Environment::new()),
        }
    }

    pub fn interpret(&mut self, mut stmts:Vec<Stmt>) {
        for stmt in stmts.iter_mut() {
            if let Err(e) = self.eval_stmt(stmt) {
                eprintln!("{e}");
                //set runtimeErr = true
            }
        }
    }

    fn eval_stmt(&mut self, stmt: &Stmt) -> Result<(), EvalError>{
        match stmt {
            Stmt::Expr(e) => {
                let _ = self.eval_expr(e)?;
                Ok(())
            },
            Stmt::Print(e) => {
                let val = self.eval_expr(e)?;
                println!("{val}");
                Ok(())
            }
            Stmt::Var { name, initializer } => {
                let val = match initializer {
                    Some(expr) => {
                        self.eval_expr(expr)?
                    },
                    None => Value::Nil
                };
                self.environment.as_mut().unwrap().define(name.to_string(), val);
                Ok(())
            },
            Stmt::Block(stmts) => {
                if let Some(e) = self.environment.take() {
                    self.environment = Some(e.push());
                }
                for stmt in stmts {
                    if let Err(e) = self.eval_stmt(stmt) {
                        if let Some(e) = self.environment.take() {
                            self.environment = Some(e.pop());
                        }
                        return Err(e);
                    }
                }
                
                if let Some(e) = self.environment.take() {
                    self.environment = Some(e.pop());
                }

                Ok(())
            },
            Stmt::If { condition, then_br, else_br } => {
                if self.eval_expr(condition)?.bool() {
                    self.eval_stmt(then_br)?;
                } else {
                    if let Some(else_br) = else_br {
                        self.eval_stmt(else_br)?;
                    }
                }
                Ok(())
            }
            Stmt::While { condition, body } => {
                while self.eval_expr(condition)?.bool() {
                    self.eval_stmt(body)?;
                }
                Ok(())
            },
        }
    }

    fn eval_expr(&mut self, expr: &Expr) -> Result<Value, EvalError> {
        match expr {
            Expr::Literal(v) => Ok(v.clone()),
            Expr::Variable(var_name) => {
                Ok(self.environment.as_ref().unwrap().get(var_name)?)
            }
            Expr::Binary { left, op, right } => {
                let mut left = self.eval_expr(left)?;
                let right = self.eval_expr(right)?;
                match op.kind() {
                    TokenKind::Minus => {
                        match left.try_sub(&right) {
                            Ok(_) => Ok(left),
                            Err(msg) => Err(EvalError::new(msg, op))
                        }
                    },
                    TokenKind::Plus => {
                        match left.try_sum(&right) {
                            Ok(_) => Ok(left),
                            Err(msg) => Err(EvalError::new(msg, op))
                        }
                    },
                    TokenKind::Slash => {
                        match left.try_div(&right) {
                            Ok(_) => Ok(left),
                            Err(msg) => Err(EvalError::new(msg, op))
                        }
                    },
                    TokenKind::Star => {
                        match left.try_mult(&right) {
                            Ok(_) => Ok(left),
                            Err(msg) => Err(EvalError::new(msg, op))
                        }
                    },
                    TokenKind::Greater => {
                        match left.try_gt(&right) {
                            Ok(b) => Ok(b),
                            Err(msg) => Err(EvalError::new(msg, op))
                        }
                    },
                    TokenKind::GreaterEqual => {
                        match left.try_gte(&right) {
                            Ok(b) => Ok(b),
                            Err(msg) => Err(EvalError::new(msg, op))
                        }
                    },
                    TokenKind::Less => {
                        match left.try_lt(&right) {
                            Ok(b) => Ok(b),
                            Err(msg) => Err(EvalError::new(msg, op))
                        }
                    },
                    TokenKind::LessEqual => {
                        match left.try_lte(&right) {
                            Ok(b) => Ok(b),
                            Err(msg) => Err(EvalError::new(msg, op))
                        }
                    },
                    TokenKind::EqualEqual => {
                        Ok(left.equals(&right))
                    },
                    TokenKind::BangEqual => {
                        Ok(left.neq(&right))
                    }
                    _ => { unreachable!()}
                }
            },
            Expr::Grouping(e) => {
                Ok(self.eval_expr(e)?)
            },
            Expr::Unary { op, right } => {
                match op.kind() {
                    TokenKind::Minus => {
                        let mut val = self.eval_expr(right)?;
                        match val.try_neg() {
                            Ok(_) => Ok(val),
                            Err(msg) => Err(EvalError::new(msg, op)),
                        }
                    },
                    TokenKind::Bang => {
                        let val = self.eval_expr(right)?;
                        Ok(val.inv())
                    },
                    _ => unreachable!(),
                }
            },
            Expr::Assign { name, value } => {
                let value = self.eval_expr(value)?;
                let _ = self.environment.as_mut().unwrap().assign(name, value.clone())?;
                Ok(value)
            },
            Expr::Logical { left, op, right } => {
                let left = self.eval_expr(left)?;
                match op.kind() {
                    TokenKind::And => {
                        if left.bool() == false {
                            return Ok(left)
                        }
                    },
                    TokenKind::Or => {
                        if left.bool() == true {
                            return Ok(left)
                        }
                    },
                    _ => unreachable!(),
                }
                Ok(self.eval_expr(right)?)
            },
        }
    }
}

