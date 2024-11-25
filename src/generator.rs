use core::fmt;
use std::{collections::HashSet, hash};

use fraction::Fraction;
use rand::Rng;

const MAX_CARD_VALUE: u64 = 100;
const NUM_CARDS: u64 = 4;
const TARGET_NUM: u64 = 24;

#[derive(Clone, Debug)]
pub enum NumericExpression {
    Number(u64),
    Expression {
        num_1: Box<NumericExpression>,
        num_2: Box<NumericExpression>,
        operator: &'static str,
    },
}

impl NumericExpression {
    fn _eval(&self) -> Fraction {
        match self {
            &NumericExpression::Number(n) => n.into(),
            NumericExpression::Expression {
                num_1,
                num_2,
                operator,
            } => {
                let val_1 = num_1._eval();
                let val_2 = num_2._eval();

                match *operator {
                    "+" => val_1 + val_2,
                    "-" => val_1 - val_2,
                    "*" => val_1 * val_2,
                    "/" => val_1 / val_2,
                    _ => unreachable!(),
                }
            }
        }
    }

    fn eval(&self) -> f64 {
        self._eval().try_into().unwrap()
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

pub fn find_solutions(cardset: Vec<NumericExpression>) -> Vec<NumericExpression> {
    if cardset.len() == 1 {
        return {
            if cardset.first().unwrap().eval() == TARGET_NUM as f64 {
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

            for operator in ["+", "-", "*", "/"] {
                // enforce +, -, * to have to have big_num op small_num
                if (operator == "+" || operator == "-" || operator == "*")
                    && (num_1.eval() < num_2.eval())
                {
                    continue;
                }

                new_cardset.push(NumericExpression::Expression {
                    num_1: Box::new(num_1.clone()),
                    num_2: Box::new(num_2.clone()),
                    operator,
                });
                solutions.extend(find_solutions(new_cardset.clone()));
                new_cardset.pop();
            }
        }
    }

    solutions.into_iter().collect()
}

pub fn generate_cardset() -> (Vec<u64>, Vec<NumericExpression>) {
    let mut cardset;
    let mut solutions;

    while {
        cardset = Vec::new();

        for _ in 0..NUM_CARDS {
            let rand_number = rand::thread_rng().gen_range(1..=MAX_CARD_VALUE);
            cardset.push(rand_number);
        }

        solutions = find_solutions(
            cardset
                .iter()
                .map(|c| NumericExpression::Number(*c))
                .collect(),
        );

        solutions.is_empty()
    } {}

    (cardset, solutions)
}
