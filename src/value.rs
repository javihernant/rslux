use std::fmt::Display;

#[derive(PartialEq, Clone)]
pub enum Value {
    Number (f64),
    String (String),
    Bool (bool),
    Nil,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => {write!(f,"{}",n)},
            Self::String(s) => {write!(f, "\"{}\"",s)},
            Self::Bool(b) => {write!(f, "{}", b)},
            Self::Nil => {write!(f, "nil")}
        }
    }
}

impl Value {
    fn bool(&self) -> bool {
        match self {
            Self::Bool(b) => *b,
            Self::Nil => false,
            _ => true,
        }
    }

    pub fn inv(&self) -> Value {
        let b = self.bool();
        Value::Bool(!b)
    }

    pub fn try_neg(&mut self) -> Result<(), &'static str> {
        if let Self::Number(n) = self {
            *n = -*n;
            Ok(())
        } else {
            Err("Operand must be a number")
        }
    }

    pub fn try_sub(&mut self, operand: &Value) -> Result<(), &'static str> {
        match (self, operand) {
            (Value::Number(a), Value::Number(b)) => {
                *a -= b;
                Ok(())
            },
            _ => {
                Err("operands must be numbers")
            }
        }
    }

    pub fn try_div(&mut self, operand: &Value) -> Result<(), &'static str> {
        match (self, operand) {
            (Value::Number(a), Value::Number(b)) => {
                *a /= b;
                Ok(())
            },
            _ => {
                Err("operands must be numbers")
            }
        }
    }

    pub fn try_mult(&mut self, operand: &Value) -> Result<(), &'static str> {
        match (self, operand) {
            (Value::Number(a), Value::Number(b)) => {
                *a *= b;
                Ok(())
            },
            _ => {
                Err("operands must be numbers")
            }
        }
    }
    pub fn try_sum(&mut self, operand: &Value) -> Result<(), &'static str> {
        match (self, operand) {
            (Value::Number(a), Value::Number(b)) => {
                *a += b;
                Ok(())
            },
            (Value::String(a), Value::String(b)) => {
                a.push_str(b);
                Ok(())
            },
            _ => {
                Err("operands must be both numbers or strings")
            }
        }
    }
    pub fn try_lt(&self, operand: &Value) -> Result<Value, &'static str> {
        match (self, operand) {
            (Value::Number(a), Value::Number(b)) => {
                let b = a < b;
                Ok(Value::Bool(b))
            },
            _ => {
                Err("operands must be numbers")
            }
        }
    }
    pub fn try_lte(&self, operand: &Value) -> Result<Value, &'static str> {
        match (self, operand) {
            (Value::Number(a), Value::Number(b)) => {
                let b = a <= b;
                Ok(Value::Bool(b))
            },
            _ => {
                Err("operands must be numbers")
            }
        }
    }
    pub fn try_gt(&self, operand: &Value) -> Result<Value, &'static str> {
        match (self, operand) {
            (Value::Number(a), Value::Number(b)) => {
                let b = a > b;
                Ok(Value::Bool(b))
            },
            _ => {
                Err("operands must be numbers")
            }
        }
    }
    pub fn try_gte(&self, operand: &Value) -> Result<Value, &'static str> {
        match (self, operand) {
            (Value::Number(a), Value::Number(b)) => {
                let b = a >= b;
                Ok(Value::Bool(b))
            },
            _ => {
                Err("operands must be numbers")
            }
        }
    }
    pub fn equals(&mut self, operand: &Value) -> Value {
        let b = self == operand;
        Value::Bool(b)
    }
    pub fn neq(&mut self, operand: &Value) -> Value {
        let b = self != operand;
        Value::Bool(b)
    }

}
