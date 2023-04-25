use std::{collections::VecDeque, str::FromStr, io::{stdin, stdout}};

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
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "+" => Ok(CalcToken::Operator(Operator::Plus)),
            "*" => Ok(CalcToken::Operator(Operator::Multiply)),
            s => Ok(CalcToken::Literal(s.parse::<i32>().map_err(|e| e.to_string())?)),
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

use std::io::Write;

fn main() {
    let mut queue = VecDeque::<CalcToken>::new();
    let mut stdout = stdout();

    print!("calc-desu> ");
    stdout.flush().unwrap();
    for lines in stdin().lines().filter_map(|l| l.ok()).filter(|l| !l.is_empty()) {
        match lines.parse() {
            Ok(token) => queue.push_back(token),
            Err(e) => println!("{e}"),
        }

        print!("calc-desu> ");
        stdout.flush().unwrap();
    }

    process_stack(queue);
}
