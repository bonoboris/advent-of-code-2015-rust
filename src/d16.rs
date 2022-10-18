use crate::common::{print_parts, read_lines};
use ex::io;
use std::io::BufRead;

const KEYS: [&str; 10] = [
    "children",
    "cats",
    "samoyeds",
    "pomeranians",
    "akitas",
    "vizslas",
    "goldfish",
    "trees",
    "cars",
    "perfumes",
];

#[derive(Debug, Default)]
struct Aunt {
    idx: usize,
    vals: [Option<usize>; 10],
}

fn parse_line(line: String) -> Option<Aunt> {
    let (idx_str, facts) = line.strip_prefix("Sue ")?.split_once(": ")?;
    let idx = idx_str.parse::<usize>().ok()?;
    let mut aunt = Aunt::default();
    aunt.idx = idx;
    facts.split(", ").for_each(|fact| {
        if let Some((key, val_str)) = fact.split_once(": ") {
            let val = val_str.parse::<usize>().ok();
            let pos = KEYS
                .iter()
                .position(|&x| x == key)
                .expect(&format!("unknown key: {key}"));
            aunt.vals[pos] = val;
        }
    });
    match aunt.vals == [None; 10] {
        true => None,
        false => Some(aunt),
    }
}

const FACTS: [usize; 10] = [3, 7, 2, 3, 0, 0, 5, 3, 2, 1];

fn is_aunt(aunt: &Aunt) -> bool {
    FACTS.iter().zip(aunt.vals.iter()).all(|(f, a)| match a {
        Some(x) => x == f,
        None => true,
    })
}

fn is_aunt_2(aunt: &Aunt) -> bool {
    FACTS
        .iter()
        .zip(aunt.vals.iter())
        .enumerate()
        .all(|(i, (f, a))| match a {
            Some(x) => match i {
                1 | 7 => x > f,
                3 | 6 => x < f,
                _ => x == f,
            },
            None => true,
        })
}

pub fn part1() -> io::Result<usize> {
    let aunt = read_lines!("data/d16.txt")?
        .flat_map(parse_line)
        .find(is_aunt);
    Ok(aunt.expect("No aunt found : (").idx)
}
pub fn part2() -> io::Result<usize> {
    let aunt = read_lines!("data/d16.txt")?
        .flat_map(parse_line)
        .find(is_aunt_2);
    Ok(aunt.expect("No aunt found : (").idx)
}

pub fn main() {
    print_parts(16, part1(), part2())
}
