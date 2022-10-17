use std::{collections::HashMap, error::Error};

use itertools::Itertools;

use crate::common::{print_parts, read_lines};
use std::io::BufRead;

type Affinity = (String, String, i32);
type AffinityMap = HashMap<String, HashMap<String, i32>>;

fn parse_line(line: impl AsRef<str>) -> Option<Affinity> {
    match line
        .as_ref()
        .trim_end_matches(".")
        .split_whitespace()
        .collect_vec()[..]
    {
        [pers, _, "gain", ammount, "happiness", "units", "by", "sitting", "next", "to", other] => {
            Some((
                String::from(pers),
                String::from(other),
                ammount.parse::<i32>().ok()?,
            ))
        }
        [pers, _, "lose", ammount, "happiness", "units", "by", "sitting", "next", "to", other] => {
            Some((
                String::from(pers),
                String::from(other),
                -ammount.parse::<i32>().ok()?,
            ))
        }
        _ => None,
    }
}

fn build_map(affinities: &[Affinity]) -> AffinityMap {
    let mut map = AffinityMap::default();
    affinities.iter().for_each(|(from, to, val)| {
        map.entry(from.to_owned())
            .or_default()
            .insert(to.to_owned(), *val);
    });
    map
}

fn compute_combi_affinity(combi: &[&String], affinity_map: &AffinityMap) -> i32 {
    let mut aff = 0;
    let n = combi.len();
    for i in 0..n {
        let cur = combi[i];
        let nex = combi[(i + 1) % n];
        let pre = combi[(i + n - 1) % n];
        aff += affinity_map[cur][pre];
        aff += affinity_map[cur][nex];
    }
    aff
}

fn solve(affinity_map: &AffinityMap) -> i32 {
    affinity_map
        .keys()
        .permutations(affinity_map.len())
        .map(|combi| compute_combi_affinity(&combi, affinity_map))
        .max()
        .expect("Empty affinities")
}

pub fn part1() -> Result<i32, Box<dyn Error>> {
    let affinities = read_lines!("data/d13.txt")?
        .filter_map(parse_line)
        .collect_vec();
    let affinity_map = build_map(&affinities);
    Ok(solve(&affinity_map))
}

pub fn part2() -> Result<i32, Box<dyn Error>> {
    let affinities = read_lines!("data/d13.txt")?
        .filter_map(parse_line)
        .collect_vec();
    let mut affinity_map = build_map(&affinities);
    let me_map = HashMap::<String, i32>::from_iter(affinity_map.keys().map(|k| (k.clone(), 0)));
    affinity_map.values_mut().for_each(|submap| {
        submap.insert("me".to_owned(), 0);
    });
    affinity_map.insert("me".to_owned(), me_map);
    dbg!(&affinity_map);
    Ok(solve(&affinity_map))
}

pub fn main() {
    print_parts(13, part1(), part2())
}
