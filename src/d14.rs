use ex::io;
use itertools::Itertools;

use crate::common::{print_parts, read_lines};
use std::{collections::HashMap, io::BufRead};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Deer {
    name: String,
    speed: usize,
    sprint: usize,
    rest: usize,
}

fn parse_line(line: impl AsRef<str>) -> Option<Deer> {
    match line.as_ref().split_whitespace().collect_vec()[..] {
        [name, _, _, speed, _, _, sprint, _, _, _, _, _, _, rest, _] => Some(Deer {
            name: name.to_owned(),
            speed: speed.parse().ok()?,
            sprint: sprint.parse().ok()?,
            rest: rest.parse().ok()?,
        }),
        _ => None,
    }
}

fn compute_dist(deer: &Deer, duration: usize) -> usize {
    let (n, r) = (
        duration / (deer.sprint + deer.rest),
        duration % (deer.sprint + deer.rest),
    );
    (n * deer.sprint + r.min(deer.sprint)) * deer.speed
}

fn compute_max_points(deers: &[Deer], duration: usize) -> usize {
    let mut points = HashMap::<String, usize>::from_iter(deers.iter().map(|d| (d.name.clone(), 0)));

    for t in 1..(duration + 1) {
        deers
            .iter()
            .max_set_by_key(|&deer| compute_dist(deer, t))
            .iter()
            .for_each(|&d| {
                points.insert(d.name.clone(), points[&d.name] + 1);
            })
    }
    points
        .values()
        .max()
        .expect("Empty deers ? or duration = 0 ?")
        .to_owned()
}

pub fn part1() -> io::Result<usize> {
    let duration = 2503;
    Ok(read_lines!("data/d14.txt")?
        .filter_map(parse_line)
        .map(|deer| compute_dist(&deer, duration))
        .max()
        .expect("Empty data file ?"))
}

pub fn part2() -> io::Result<usize> {
    let duration = 2503;
    let deers = read_lines!("data/d14.txt")?
        .filter_map(parse_line)
        .collect_vec();

    Ok(compute_max_points(&deers, duration))
}

pub fn main() {
    print_parts(14, part1(), part2())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let line = "Vixen can fly 19 km/s for 7 seconds, but then must rest for 124 seconds.";
        assert_eq!(
            parse_line(line),
            Some(Deer {
                name: "Vixen".to_owned(),
                speed: 19,
                sprint: 7,
                rest: 124,
            })
        )
    }

    #[test]
    fn test_compute_dist() {
        let commet = Deer {
            name: "Commet".to_owned(),
            speed: 14,
            sprint: 10,
            rest: 127,
        };
        let dancer = Deer {
            name: "Dancer".to_owned(),
            speed: 16,
            sprint: 11,
            rest: 162,
        };
        assert_eq!(compute_dist(&commet, 1), 14);
        assert_eq!(compute_dist(&dancer, 1), 16);
        assert_eq!(compute_dist(&commet, 10), 140);
        assert_eq!(compute_dist(&dancer, 10), 160);
        assert_eq!(compute_dist(&commet, 11), 140);
        assert_eq!(compute_dist(&dancer, 11), 176);
        assert_eq!(compute_dist(&commet, 12), 140);
        assert_eq!(compute_dist(&dancer, 12), 176);
        assert_eq!(compute_dist(&commet, 1000), 1120);
        assert_eq!(compute_dist(&dancer, 1000), 1056);
    }

    #[test]
    fn test_compute_max_points() {
        let commet = Deer {
            name: "Commet".to_owned(),
            speed: 14,
            sprint: 10,
            rest: 127,
        };
        let dancer = Deer {
            name: "Dancer".to_owned(),
            speed: 16,
            sprint: 11,
            rest: 162,
        };
        let deers = [commet, dancer];
        assert_eq!(compute_max_points(&deers, 1), 1);
        assert_eq!(compute_max_points(&deers, 139), 139);
        assert_eq!(compute_max_points(&deers, 140), 139);
        assert_eq!(compute_max_points(&deers, 1000), 689);
    }
}
