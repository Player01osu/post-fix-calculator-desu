use std::{collections::VecDeque, str::FromStr, io::{stdin, stdout}};
use std::io::Write;

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Add,
    Sub,
    Mult,
    Div,
}

#[derive(Debug)]
pub enum CalcToken {
    Literal(f64),
    Operator(Operator),
    PrintStack,
    ClearStack,
}

type Stack = VecDeque<f64>;

impl FromStr for CalcToken {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "+" => Ok(CalcToken::Operator(Operator::Add)),
            "-" => Ok(CalcToken::Operator(Operator::Sub)),
            "*" => Ok(CalcToken::Operator(Operator::Mult)),
            "/" => Ok(CalcToken::Operator(Operator::Div)),
            "p" => Ok(CalcToken::PrintStack),
            "c" => Ok(CalcToken::ClearStack),
            s => Ok(CalcToken::Literal(s.parse::<f64>().map_err(|e| e.to_string())?)),
        }
    }
}

fn print_stack(stack: &Stack) {
    for i in stack {
        println!("{i}");
    }
}

fn pop_two(stack: &mut Stack) -> Option<(f64, f64)> {
    if let Some(a) = stack.pop_front() {
        if let Some(b) = stack.pop_front() {
            return Some((a, b));
        }
        stack.push_front(a);
    }
    None
}

fn pop_two_warn(stack: &mut Stack) -> Option<(f64, f64)> {
    pop_two(stack).or_else(|| {
        eprintln!("Stack is empty");
        None
    })
}

fn exec_operator(stack: &mut Stack, operator: Operator) {
    let (a, b) = match pop_two_warn(stack) {
        Some(v) => v,
        None => return,
    };
    match operator {
        Operator::Add => stack.push_front(a + b),
        Operator::Sub => stack.push_front(a - b),
        Operator::Mult => stack.push_front(a * b),
        Operator::Div => stack.push_front(a / b),
    }
}

fn main() {
    let mut stack = Stack::new();
    let mut stdout = stdout();

    print!("calc-desu> ");
    stdout.flush().unwrap();
    for lines in stdin().lines().filter_map(|l| l.ok()).filter(|l| !l.is_empty()) {
        match lines.parse() {
            Ok(token) => {
                match token {
                    CalcToken::PrintStack => print_stack(&stack),
                    CalcToken::ClearStack => stack.clear(),
                    CalcToken::Operator(operator) => exec_operator(&mut stack, operator),
                    CalcToken::Literal(n) => stack.push_back(n),
                }
            }
            Err(e) => println!("{e}"),
        }

        print!("calc-desu> ");
        stdout.flush().unwrap();
    }
}
