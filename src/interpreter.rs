use std::fmt::Display;

use crate::{expr::{Expr, ExprErr}, token::{TokenKind, Token}};

use crate::value::Value;
pub struct Interpreter;

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

fn eval_expr(expr: &Expr) -> Result<Value, EvalErr> {
    match expr {
        Expr::Literal(v) => Ok(v.clone()),
        Expr::Binary { left, op, right } => {
            let mut left = eval_expr(left)?;
            let right = eval_expr(right)?;
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
            Ok(eval_expr(e)?)
        },
        Expr::Unary { op, right } => {
            match op.kind() {
                TokenKind::Minus => {
                    let mut val = eval_expr(right)?;
                    match val.try_neg() {
                        Ok(_) => Ok(val),
                        Err(msg) => Err(EvalErr::new(op, msg)),
                    }
                },
                TokenKind::Bang => {
                    let val = eval_expr(right)?;
                    Ok(val.inv())
                },
                _ => unreachable!(),
            }
        },
    }
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter
    }

    pub fn interpret(&self, expr: Expr) {
        match eval_expr(&expr) {
            Ok(v) => {
                println!("{v}");
            }
            Err(e) => {
                eprintln!("{e}")
            }
        }
    }

    
}

