use std::{fmt::Display, collections::HashMap};

use crate::{expr::{Expr, ExprErr}, token::{TokenKind, Token}};

use crate::value::Value;

use crate::expr::{Stmt, StmtErr};
pub struct Interpreter {
    env_vars: HashMap<String, Value>,
}

struct EvalErr {
    messg: String,
    line: usize,
}

impl EvalErr {
    pub fn new(token: &Token, messg: &str)->EvalErr{
        EvalErr {
            messg: messg.to_string(),
            line: token.line(),
        }
    }
}

impl Display for EvalErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f,"{}",self.messg)?;
        write!(f,"[line {}]",self.line)
    }
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

    fn eval_stmt(&mut self, stmt: &Stmt) -> Result<(), EvalErr>{
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

    fn eval_expr(&self, expr: &Expr) -> Result<Value, EvalErr> {
        match expr {
            Expr::Literal(v) => Ok(v.clone()),
            Expr::Variable(var_name) => {
                match self.env_vars.get(var_name.lexeme()) {
                    Some(val) => Ok(val.clone()),
                    None => return Err(EvalErr::new(var_name, "Undefined variable"))
                }
            }
            Expr::Binary { left, op, right } => {
                let mut left = self.eval_expr(left)?;
                let right = self.eval_expr(right)?;
                match op.kind() {
                    TokenKind::Minus => {
                        match left.try_sub(&right) {
                            Ok(_) => Ok(left),
                            Err(msg) => Err(EvalErr::new(op, msg))
                        }
                    },
                    TokenKind::Plus => {
                        match left.try_sum(&right) {
                            Ok(_) => Ok(left),
                            Err(msg) => Err(EvalErr::new(op, msg))
                        }
                    },
                    TokenKind::Slash => {
                        match left.try_div(&right) {
                            Ok(_) => Ok(left),
                            Err(msg) => Err(EvalErr::new(op, msg))
                        }
                    },
                    TokenKind::Star => {
                        match left.try_mult(&right) {
                            Ok(_) => Ok(left),
                            Err(msg) => Err(EvalErr::new(op, msg))
                        }
                    },
                    TokenKind::Greater => {
                        match left.try_gt(&right) {
                            Ok(b) => Ok(b),
                            Err(msg) => Err(EvalErr::new(op, msg))
                        }
                    },
                    TokenKind::GreaterEqual => {
                        match left.try_gte(&right) {
                            Ok(b) => Ok(b),
                            Err(msg) => Err(EvalErr::new(op, msg))
                        }
                    },
                    TokenKind::Less => {
                        match left.try_lt(&right) {
                            Ok(b) => Ok(b),
                            Err(msg) => Err(EvalErr::new(op, msg))
                        }
                    },
                    TokenKind::LessEqual => {
                        match left.try_lte(&right) {
                            Ok(b) => Ok(b),
                            Err(msg) => Err(EvalErr::new(op, msg))
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
                            Err(msg) => Err(EvalErr::new(op, msg)),
                        }
                    },
                    TokenKind::Bang => {
                        let val = self.eval_expr(right)?;
                        Ok(val.inv())
                    },
                    _ => unreachable!(),
                }
            },
        }
    }

    
}

