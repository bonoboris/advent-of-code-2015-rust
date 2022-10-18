use crate::common::{print_parts, read_lines};
use derive_more::{Add, Sum};
use ex::io;
use itertools::Itertools;
use std::io::BufRead;

fn clamp_0(val: isize) -> usize {
    val.clamp(0, isize::MAX) as usize
}

#[derive(Add, Sum, PartialEq, Eq, Debug)]
struct Specs {
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: usize,
}

impl Specs {
    fn scale(&self, k: usize) -> Self {
        let k_isize = k as isize;
        Self {
            capacity: k_isize * self.capacity,
            durability: k_isize * self.durability,
            flavor: k_isize * self.flavor,
            texture: k_isize * self.texture,
            calories: k * self.calories,
        }
    }

    fn score(&self) -> usize {
        clamp_0(self.capacity)
            * clamp_0(self.durability)
            * clamp_0(self.flavor)
            * clamp_0(self.texture)
    }
}

fn parse_line(line: impl AsRef<str>) -> Option<Specs> {
    match line
        .as_ref()
        .replace(",", "")
        .split_whitespace()
        .collect_vec()[..]
    {
        [_, _, capacity, _, durability, _, flavor, _, texture, _, calories] => Some(Specs {
            capacity: capacity.parse().unwrap(),
            durability: durability.parse().unwrap(),
            flavor: flavor.parse().unwrap(),
            texture: texture.parse().unwrap(),
            calories: calories.parse().unwrap(),
        }),
        _ => None,
    }
}
fn compute_receipe(combi: &[usize], sum: usize) -> Vec<usize> {
    let mut receipe = vec![combi[0]];
    receipe.extend(combi.iter().zip(combi.iter().skip(1)).map(|(p, n)| n - p));
    receipe.push(sum - combi[combi.len() - 1]);
    receipe
}
fn receipes(n: usize, sum: usize) -> impl Iterator<Item = Vec<usize>> {
    (1..sum)
        .combinations(n - 1)
        .map(move |combi| compute_receipe(&combi, sum))
}

fn compute_score(specs: &[Specs], receipe: &[usize]) -> usize {
    specs
        .iter()
        .zip(receipe.iter())
        .map(|(spec, ammount)| spec.scale(*ammount))
        .sum::<Specs>()
        .score()
}

fn compute_score_500cal(specs: &[Specs], receipe: &[usize]) -> Option<usize> {
    let receipe_specs = specs
        .iter()
        .zip(receipe.iter())
        .map(|(spec, ammount)| spec.scale(*ammount))
        .sum::<Specs>();
    match receipe_specs.calories {
        500 => Some(receipe_specs.score()),
        _ => None,
    }
}

fn get_highest_score(specs: &[Specs], sum: usize) -> usize {
    receipes(specs.len(), sum)
        .map(|receipe| compute_score(specs, &receipe))
        .max()
        .expect("Empty specs ?")
}

fn get_highest_score_500cal(specs: &[Specs], sum: usize) -> usize {
    receipes(specs.len(), sum)
        .filter_map(|receipe| compute_score_500cal(specs, &receipe))
        .max()
        .expect("Empty specs ?")
}

pub fn part1() -> io::Result<usize> {
    let specs = read_lines!("data/d15.txt")?
        .flat_map(parse_line)
        .collect_vec();
    Ok(get_highest_score(&specs, 100))
}
pub fn part2() -> io::Result<usize> {
    let specs = read_lines!("data/d15.txt")?
        .flat_map(parse_line)
        .collect_vec();
    Ok(get_highest_score_500cal(&specs, 100))
}

pub fn main() {
    print_parts(15, part1(), part2())
}

#[cfg(test)]
mod tests {
    use super::*;

    const LINE1: &str = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8";
    const LINE2: &str = "Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";
    const SPECS1: Specs = Specs {
        capacity: -1,
        durability: -2,
        flavor: 6,
        texture: 3,
        calories: 8,
    };
    const SPECS2: Specs = Specs {
        capacity: 2,
        durability: 3,
        flavor: -2,
        texture: -1,
        calories: 3,
    };

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line(LINE1), Some(SPECS1));
        assert_eq!(parse_line(LINE2), Some(SPECS2));
    }

    #[test]
    fn test_get_highest_score_500cal() {
        let specs = [SPECS1, SPECS2];
        assert_eq!(get_highest_score_500cal(&specs, 100), 57600000);
    }
}
