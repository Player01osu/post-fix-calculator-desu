use std::{collections::VecDeque, str::FromStr, io::stdin};

#[derive(Debug)]
pub enum Operator {
    Plus,
    Multiply,
}

#[derive(Debug)]
pub enum CalcToken {
    Literal(i32),
    Operator(Operator),
}

impl FromStr for CalcToken {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "+" => Ok(CalcToken::Operator(Operator::Plus)),
            "*" => Ok(CalcToken::Operator(Operator::Multiply)),
            s => Ok(CalcToken::Literal(s.parse().unwrap())),
        }
    }
}

type CalcStack = VecDeque<CalcToken>;

pub fn process_operator(stack: &mut CalcStack, operator: Operator) {
    let mut acc = None;
    while let Some(CalcToken::Literal(num)) = stack.pop_front() {
        match acc.as_mut() {
            Some(acc) => {
                match operator {
                    Operator::Plus => *acc += num,
                    Operator::Multiply => *acc *= num,
                }
            }
            None => acc = Some(num),
        }
    };
    let acc = acc.unwrap();
    println!("{acc}");
}

pub fn process_stack(mut stack: CalcStack) {
    while let Some(token) = stack.pop_front() {
        match token {
            CalcToken::Operator(operator) => process_operator(&mut stack, operator),
            k => panic!("Expected an operator, got {k:?}"),
        }
    }
}

fn main() {
    let mut queue = VecDeque::<CalcToken>::new();

    for lines in stdin().lines().filter_map(|l| l.ok()).filter(|l| !l.is_empty()) {
        queue.push_back(lines.parse().unwrap());
    }

    process_stack(queue);
}
