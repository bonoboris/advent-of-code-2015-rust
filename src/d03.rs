use crate::common::{print_parts, read_to_string};
use ex::io;

use std::collections::HashSet;

type Coords = (isize, isize);

fn decode(c: char) -> Option<Coords> {
    match c {
        '^' => Some((0, 1)),
        'v' => Some((0, -1)),
        '>' => Some((1, 0)),
        '<' => Some((-1, 0)),
        _ => None,
    }
}

fn add_coords(c1: Coords, c2: Coords) -> Coords {
    return (c1.0 + c2.0, c1.1 + c2.1);
}

fn do_part1(content: &str) -> usize {
    let mut cur: Coords = (0, 0);
    let mut visited = HashSet::<Coords>::new();
    visited.insert(cur);
    content.chars().filter_map(decode).for_each(|m| {
        cur = add_coords(cur, m);
        visited.insert(cur);
    });
    visited.len()
}

fn do_part2(content: &str) -> usize {
    let mut cur1: Coords = (0, 0);
    let mut cur2: Coords = (0, 0);
    let mut visited = HashSet::<Coords>::new();
    visited.insert(cur1);
    content
        .chars()
        .filter_map(decode)
        .enumerate()
        .for_each(|(i, m)| {
            if i % 2 == 0 {
                cur1 = add_coords(cur1, m);
                visited.insert(cur1);
            } else {
                cur2 = add_coords(cur2, m);
                visited.insert(cur2);
            }
        });
    visited.len()
}

pub fn part1() -> io::Result<usize> {
    Ok(do_part1(&read_to_string("data/d03.txt")?))
}

pub fn part2() -> io::Result<usize> {
    Ok(do_part2(&read_to_string("data/d03.txt")?))
}

pub fn main() {
    print_parts(3, part1(), part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_do_part1() {
        assert_eq!(do_part1(">"), 2);
        assert_eq!(do_part1("^>v<"), 4);
        assert_eq!(do_part1("^v^v^v^v^v"), 2);
    }
    #[test]
    fn test_do_part2() {
        assert_eq!(do_part2("^v"), 3);
        assert_eq!(do_part2("^>v<"), 3);
        assert_eq!(do_part2("^v^v^v^v^v"), 11);
    }
}
