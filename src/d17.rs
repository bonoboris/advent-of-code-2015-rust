use ex::io;
use itertools::Itertools;
use std::io::BufRead;

use crate::common::{print_parts, read_lines};

fn parse_line(line: String) -> Option<usize> {
    line.parse::<usize>().ok()
}

fn get_num_combi(vals: &Vec<usize>, target: usize) -> usize {
    (1..(vals.len() + 1))
        .flat_map(|k| vals.clone().into_iter().combinations(k))
        .filter(|perm| perm.iter().sum::<usize>() == target)
        .count()
}

fn get_num_combi_lowest_k(vals: &Vec<usize>, target: usize) -> usize {
    (1..(vals.len() + 1))
        .map(|k| {
            vals.clone()
                .into_iter()
                .combinations(k)
                .filter(|perm| perm.iter().sum::<usize>() == target)
                .count()
        })
        .find(|c| *c > 0)
        .expect("No ammount of containers can hold target ammount")
}

pub fn part1() -> io::Result<usize> {
    let vals = read_lines!("data/d17.txt")?
        .filter_map(parse_line)
        .collect_vec();
    Ok(get_num_combi(&vals, 150))
}

pub fn part2() -> io::Result<usize> {
    let vals = read_lines!("data/d17.txt")?
        .filter_map(parse_line)
        .collect_vec();
    Ok(get_num_combi_lowest_k(&vals, 150))
}

pub fn main() {
    print_parts(17, part1(), part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_num_combi() {
        assert_eq!(get_num_combi(&vec![20, 15, 10, 5, 5], 25), 4);
    }
    #[test]
    fn test_get_num_combi_lowest_k() {
        assert_eq!(get_num_combi_lowest_k(&vec![20, 15, 10, 5, 5], 25), 3);
    }
}
