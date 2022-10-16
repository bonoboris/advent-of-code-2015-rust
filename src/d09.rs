use crate::common::{print_parts, read_lines};
use ex::io;

use itertools::Itertools;
use std::collections::HashMap;

fn parse_line(line: impl AsRef<str>) -> (String, String, u32) {
    match line
        .as_ref()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .as_slice()
    {
        [c1, "to", c2, "=", d] => (
            c1.to_string(),
            c2.to_string(),
            d.parse::<u32>()
                .expect(&format!("Error parsind '{}' as u32", d)),
        ),
        _ => panic!("Cannot parse line: '{}'", line.as_ref()),
    }
}

type DistMap = HashMap<String, HashMap<String, u32>>;

fn build_dist_map<'a, I: Iterator<Item = impl AsRef<str>>>(it_lines: I) -> DistMap {
    let mut ret = DistMap::new();
    it_lines
        .map(|line| parse_line(&line))
        .for_each(|(c1, c2, d)| {
            ret.entry(c1.to_owned())
                .or_insert(HashMap::<String, u32>::new())
                .insert(c2.to_owned(), d);
            ret.entry(c2.to_owned())
                .or_insert(HashMap::<String, u32>::new())
                .insert(c1.to_owned(), d);
        });
    ret
}

fn compute_path_dist(path: &Vec<&String>, dists: &DistMap) -> u32 {
    path.iter()
        .zip(path[1..].iter())
        .map(|(c1, c2)| dists[*c1][*c2])
        .sum::<u32>()
}

fn find_shortest(dists: &DistMap) -> Option<u32> {
    dists
        .keys()
        .permutations(dists.len())
        .map(|path| compute_path_dist(&path, dists))
        .min()
}
fn find_longest(dists: &DistMap) -> Option<u32> {
    dists
        .keys()
        .permutations(dists.len())
        .map(|path| compute_path_dist(&path, dists))
        .max()
}

pub fn part1() -> io::Result<u32> {
    let dists = build_dist_map(read_lines("data/d08.txt")?);
    Ok(find_shortest(&dists).unwrap_or_default())
}

pub fn part2() -> io::Result<u32> {
    let dists = build_dist_map(read_lines("data/d08.txt")?);
    Ok(find_longest(&dists).unwrap_or_default())
}

pub fn main() {
    print_parts(8, part1(), part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("London to Dublin = 464"),
            ("London".to_owned(), "Dublin".to_owned(), 464)
        );
        assert_eq!(
            parse_line("London to Belfast = 518"),
            ("London".to_owned(), "Belfast".to_owned(), 518)
        );
        assert_eq!(
            parse_line("Dublin to Belfast = 141"),
            ("Dublin".to_owned(), "Belfast".to_owned(), 141)
        );
    }

    #[test]
    fn test_build_dist_map() {
        let input = [
            "London to Dublin = 464",
            "London to Belfast = 518",
            "Dublin to Belfast = 141",
        ];
        let out = HashMap::from([
            (
                "London".to_owned(),
                HashMap::from([("Dublin".to_owned(), 464), ("Belfast".to_owned(), 518)]),
            ),
            (
                "Dublin".to_owned(),
                HashMap::from([("London".to_owned(), 464), ("Belfast".to_owned(), 141)]),
            ),
            (
                "Belfast".to_owned(),
                HashMap::from([("London".to_owned(), 518), ("Dublin".to_owned(), 141)]),
            ),
        ]);
        assert_eq!(build_dist_map(input.iter()), out);
    }

    #[test]
    fn test_find_shortest() {
        let input = HashMap::from([
            (
                "London".to_owned(),
                HashMap::from([("Dublin".to_owned(), 464), ("Belfast".to_owned(), 518)]),
            ),
            (
                "Dublin".to_owned(),
                HashMap::from([("London".to_owned(), 464), ("Belfast".to_owned(), 141)]),
            ),
            (
                "Belfast".to_owned(),
                HashMap::from([("London".to_owned(), 518), ("Dublin".to_owned(), 141)]),
            ),
        ]);
        assert_eq!(find_shortest(&input), Some(605));
    }

    #[test]
    fn test_find_longest() {
        let input = HashMap::from([
            (
                "London".to_owned(),
                HashMap::from([("Dublin".to_owned(), 464), ("Belfast".to_owned(), 518)]),
            ),
            (
                "Dublin".to_owned(),
                HashMap::from([("London".to_owned(), 464), ("Belfast".to_owned(), 141)]),
            ),
            (
                "Belfast".to_owned(),
                HashMap::from([("London".to_owned(), 518), ("Dublin".to_owned(), 141)]),
            ),
        ]);
        assert_eq!(find_longest(&input), Some(982));
    }
}
