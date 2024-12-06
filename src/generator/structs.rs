use std::{fmt, hash};

use fraction::Fraction;
use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, EnumIter)]
pub enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

#[derive(Clone, Debug)]
pub enum NumericExpression {
    Number(u64),
    Expression {
        num_1: Box<NumericExpression>,
        num_2: Box<NumericExpression>,
        operator: Operator,
    },
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str_op = match self {
            Operator::Addition => "+",
            Operator::Subtraction => "-",
            Operator::Multiplication => "*",
            Operator::Division => "/",
        };
        write!(f, "{}", str_op)
    }
}

impl NumericExpression {
    pub fn eval(&self) -> Fraction {
        match self {
            &NumericExpression::Number(n) => n.into(),
            NumericExpression::Expression {
                num_1,
                num_2,
                operator,
            } => {
                let val_1 = num_1.eval();
                let val_2 = num_2.eval();

                match *operator {
                    Operator::Addition => val_1 + val_2,
                    Operator::Subtraction => val_1 - val_2,
                    Operator::Multiplication => val_1 * val_2,
                    Operator::Division => val_1 / val_2,
                }
            }
        }
    }
}

impl fmt::Display for NumericExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NumericExpression::Number(n) => write!(f, "{}", n),
            NumericExpression::Expression {
                num_1,
                num_2,
                operator,
            } => write!(f, "({} {} {})", num_1, operator, num_2),
        }
    }
}

impl hash::Hash for NumericExpression {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        format!("{}", self).hash(state);
    }
}

impl PartialEq for NumericExpression {
    fn eq(&self, other: &Self) -> bool {
        format!("{}", self) == format!("{}", other)
    }
}

impl Eq for NumericExpression {}
