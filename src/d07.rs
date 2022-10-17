use crate::common::{print_parts, read_lines};
use ex::io;

use std::collections::HashMap;
use std::io::BufRead;
use std::vec;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Operand {
    Wire(String),
    Signal(u16),
}

impl Operand {
    fn parse(val: &str) -> Self {
        match val.parse::<u16>() {
            Ok(x) => Self::Signal(x),
            _ => Self::Wire(val.to_owned()),
        }
    }

    fn get_value(self, state: &HashMap<String, u16>) -> u16 {
        match self {
            Self::Signal(x) => x,
            Self::Wire(key) => *state.get(&key).unwrap(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Operation {
    Rshift(Operand, Operand),
    Lshift(Operand, Operand),
    Or(Operand, Operand),
    And(Operand, Operand),
    Not(Operand),
    Assign(Operand),
}

impl Operation {
    fn compute(&self, state: &HashMap<String, u16>) -> u16 {
        match self.clone() {
            Self::Rshift(op1, op2) => op1.get_value(state) >> op2.get_value(state),
            Self::Lshift(op1, op2) => op1.get_value(state) << op2.get_value(state),
            Self::Or(op1, op2) => op1.get_value(state) | op2.get_value(state),
            Self::And(op1, op2) => op1.get_value(state) & op2.get_value(state),
            Self::Not(op) => !op.get_value(state),
            Self::Assign(op) => op.get_value(state),
        }
    }
    fn operands(&self) -> Vec<Operand> {
        match self.clone() {
            Self::Rshift(op1, op2) => vec![op1, op2],
            Self::Lshift(op1, op2) => vec![op1, op2],
            Self::Or(op1, op2) => vec![op1, op2],
            Self::And(op1, op2) => vec![op1, op2],
            Self::Not(op) => vec![op],
            Self::Assign(op) => vec![op],
        }
    }

    fn wires(self) -> Vec<String> {
        self.operands()
            .iter()
            .filter_map(|o| match o {
                Operand::Wire(v) => Some(v.to_owned()),
                Operand::Signal(_) => None,
            })
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Command {
    operation: Operation,
    target: String,
}

impl Command {
    fn is_appliable(&self, state: &HashMap<String, u16>) -> bool {
        self.operation
            .clone()
            .wires()
            .iter()
            .all(|k| state.contains_key(k))
    }

    fn apply(&self, state: &mut HashMap<String, u16>) {
        state.insert(self.target.clone(), self.operation.compute(state));
    }
}

fn parse_line(line: impl AsRef<str> + std::fmt::Display) -> Command {
    let tokens: Vec<&str> = line.as_ref().split_whitespace().collect();
    match tokens.as_slice() {
        [op1, "LSHIFT", op2, "->", target] => Command {
            operation: Operation::Lshift(Operand::parse(op1), Operand::parse(op2)),
            target: (*target).to_owned(),
        },
        [op1, "RSHIFT", op2, "->", target] => Command {
            operation: Operation::Rshift(Operand::parse(op1), Operand::parse(op2)),
            target: (*target).to_owned(),
        },
        [op1, "AND", op2, "->", target] => Command {
            operation: Operation::And(Operand::parse(op1), Operand::parse(op2)),
            target: (*target).to_owned(),
        },
        [op1, "OR", op2, "->", target] => Command {
            operation: Operation::Or(Operand::parse(op1), Operand::parse(op2)),
            target: (*target).to_owned(),
        },
        ["NOT", op, "->", target] => Command {
            operation: Operation::Not(Operand::parse(op)),
            target: (*target).to_owned(),
        },
        [sig, "->", target] => Command {
            operation: Operation::Assign(Operand::parse(sig)),
            target: (*target).to_owned(),
        },
        _ => panic!("Cannot parse line: '{}' as Command", line),
    }
}

fn get_wire_a(commands: &[Command]) -> u16 {
    let num_commands = commands.len();
    let mut num_executed = 0;
    let mut executeds = vec![false; num_commands];
    let mut state = HashMap::<String, u16>::new();
    while num_executed != num_commands {
        for (cmd, is_executed) in commands.iter().zip(executeds.iter_mut()) {
            if cmd.is_appliable(&mut state) && !*is_executed {
                cmd.apply(&mut state);
                *is_executed = true;
                num_executed += 1;
            }
        }
    }
    state["a"]
}

pub fn part1() -> io::Result<u16> {
    let commands: Vec<Command> = Vec::from_iter(read_lines!("data/d07.txt")?.map(parse_line));
    Ok(get_wire_a(&commands))
}

pub fn part2() -> io::Result<u16> {
    let b_val = part1()?;
    let mut commands: Vec<Command> = Vec::from_iter(read_lines!("data/d07.txt")?.map(parse_line));
    if let Some(assign_b_command) = commands.iter_mut().find(|cmd| match cmd.operation {
        Operation::Assign(_) => cmd.target == "b",
        _ => false,
    }) {
        assign_b_command.operation = Operation::Assign(Operand::Signal(b_val));
    }
    Ok(get_wire_a(&commands))
}

pub fn main() {
    print_parts(7, part1(), part2());
}
