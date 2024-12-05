use core::fmt;
use std::{collections::HashSet, hash};

use fraction::Fraction;
use rand::Rng;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub const MAX_CARD_VALUE: u64 = 12;
pub const NUM_CARDS: usize = 4;
pub const TARGET_NUM: u64 = 24;

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

pub fn is_solvable(cardset: Vec<NumericExpression>) -> bool {
    if cardset.len() == 1 {
        cardset.first().unwrap().eval() == TARGET_NUM.into()
    } else {
        for idx_1 in 0..cardset.len() {
            for idx_2 in 0..cardset.len() {
                if idx_1 == idx_2 {
                    continue;
                }

                let num_1 = &cardset[idx_1];
                let num_2 = &cardset[idx_2];
                let mut new_cardset: Vec<_> = cardset
                    .iter()
                    .enumerate()
                    .filter_map(|(i, c)| {
                        if i != idx_1 && i != idx_2 {
                            Some(c.clone())
                        } else {
                            None
                        }
                    })
                    .collect();

                for operator in Operator::iter() {
                    new_cardset.push(NumericExpression::Expression {
                        num_1: Box::new(num_1.clone()),
                        num_2: Box::new(num_2.clone()),
                        operator,
                    });

                    if is_solvable(new_cardset.clone()) {
                        return true;
                    }

                    new_cardset.pop();
                }
            }
        }

        false
    }
}

pub fn find_all_solutions(cardset: Vec<NumericExpression>) -> Vec<NumericExpression> {
    if cardset.len() == 1 {
        return {
            if cardset.first().unwrap().eval() == TARGET_NUM.into() {
                vec![cardset.first().unwrap().clone()]
            } else {
                Vec::new()
            }
        };
    }

    let mut solutions = HashSet::new();

    for idx_1 in 0..cardset.len() {
        for idx_2 in 0..cardset.len() {
            if idx_1 == idx_2 {
                continue;
            }

            let num_1 = &cardset[idx_1];
            let num_2 = &cardset[idx_2];
            let mut new_cardset: Vec<_> = cardset
                .iter()
                .enumerate()
                .filter_map(|(i, c)| {
                    if i != idx_1 && i != idx_2 {
                        Some(c.clone())
                    } else {
                        None
                    }
                })
                .collect();

            for operator in Operator::iter() {
                new_cardset.push(NumericExpression::Expression {
                    num_1: Box::new(num_1.clone()),
                    num_2: Box::new(num_2.clone()),
                    operator,
                });
                solutions.extend(find_all_solutions(new_cardset.clone()));
                new_cardset.pop();
            }
        }
    }

    solutions.into_iter().collect()
}

pub fn find_distinct_solutions(cardset: Vec<NumericExpression>) -> Vec<NumericExpression> {
    find_all_solutions(cardset)
}

pub fn generate_cardset() -> Vec<NumericExpression> {
    let mut cardset;

    while {
        cardset = Vec::with_capacity(NUM_CARDS);

        for _ in 0..NUM_CARDS {
            let rand_number = rand::thread_rng().gen_range(1..=MAX_CARD_VALUE);
            cardset.push(NumericExpression::Number(rand_number));
        }

        !is_solvable(cardset.clone())
    } {}

    cardset
}
