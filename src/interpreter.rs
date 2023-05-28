use std::collections::HashMap;

use crate::expr::Expr;
use crate::token::TokenKind;
use crate::error::runtime::EvalError;
use crate::value::Value;
use crate::expr::Stmt;

pub struct Interpreter {
    env_vars: HashMap<String, Value>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            env_vars: HashMap::new(),
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
                self.env_vars.insert(name.to_string(), val);
                Ok(())
            },
        }
    }

    fn eval_expr(&mut self, expr: &Expr) -> Result<Value, EvalError> {
        match expr {
            Expr::Literal(v) => Ok(v.clone()),
            Expr::Variable(var_name) => {
                match self.env_vars.get(var_name.lexeme()) {
                    Some(val) => Ok(val.clone()),
                    None => return Err(EvalError::new(var_name, "Undefined variable"))
                }
            }
            Expr::Binary { left, op, right } => {
                let mut left = self.eval_expr(left)?;
                let right = self.eval_expr(right)?;
                match op.kind() {
                    TokenKind::Minus => {
                        match left.try_sub(&right) {
                            Ok(_) => Ok(left),
                            Err(msg) => Err(EvalError::new(op, msg))
                        }
                    },
                    TokenKind::Plus => {
                        match left.try_sum(&right) {
                            Ok(_) => Ok(left),
                            Err(msg) => Err(EvalError::new(op, msg))
                        }
                    },
                    TokenKind::Slash => {
                        match left.try_div(&right) {
                            Ok(_) => Ok(left),
                            Err(msg) => Err(EvalError::new(op, msg))
                        }
                    },
                    TokenKind::Star => {
                        match left.try_mult(&right) {
                            Ok(_) => Ok(left),
                            Err(msg) => Err(EvalError::new(op, msg))
                        }
                    },
                    TokenKind::Greater => {
                        match left.try_gt(&right) {
                            Ok(b) => Ok(b),
                            Err(msg) => Err(EvalError::new(op, msg))
                        }
                    },
                    TokenKind::GreaterEqual => {
                        match left.try_gte(&right) {
                            Ok(b) => Ok(b),
                            Err(msg) => Err(EvalError::new(op, msg))
                        }
                    },
                    TokenKind::Less => {
                        match left.try_lt(&right) {
                            Ok(b) => Ok(b),
                            Err(msg) => Err(EvalError::new(op, msg))
                        }
                    },
                    TokenKind::LessEqual => {
                        match left.try_lte(&right) {
                            Ok(b) => Ok(b),
                            Err(msg) => Err(EvalError::new(op, msg))
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
                            Err(msg) => Err(EvalError::new(op, msg)),
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
                let var_name = name.lexeme();
                if self.env_vars.contains_key(var_name) {
                    let value = self.eval_expr(value)?;
                    self.env_vars.insert(var_name.to_string(), value.clone());
                    Ok(value)
                } else {
                    Err(EvalError::new(name, "Undefined variable"))
                }
            },
        }
    }

    
}

