use crate::common::{print_parts, read_lines};
use ex::io;

fn get_wrapping_paper(dims: &[u32; 3]) -> u32 {
    let mut areas = [dims[0] * dims[1], dims[0] * dims[2], dims[1] * dims[2]];
    areas.sort();
    2 * (areas[0] + areas[1] + areas[2]) + areas[0]
}

fn get_ribbon(dims: &[u32; 3]) -> u32 {
    let mut sorted = dims.clone();
    sorted.sort();
    2 * (sorted[0] + sorted[1]) + sorted[0] * sorted[1] * sorted[2]
}

fn parse_dims(s: &str) -> Option<[u32; 3]> {
    let dims: Vec<u32> = s
        .split("x")
        .map(|x| x.parse::<u32>())
        .collect::<Result<Vec<u32>, _>>()
        .ok()?;
    match dims.len() {
        3 => Some([dims[0], dims[1], dims[2]]),
        _ => None,
    }
}

pub fn part1() -> io::Result<u32> {
    Ok(read_lines("data/d02.txt")?
        .filter_map(|l| parse_dims(&l))
        .map(|dims| get_wrapping_paper(&dims))
        .sum())
}

pub fn part2() -> io::Result<u32> {
    Ok(read_lines("data/d02.txt")?
        .filter_map(|l| parse_dims(&l))
        .map(|dims| get_ribbon(&dims))
        .sum())
}

pub fn main() {
    print_parts(2, part1(), part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_wrapping_paper() {
        assert_eq!(get_wrapping_paper(&[0, 0, 0]), 0);
        assert_eq!(get_wrapping_paper(&[0, 0, 1]), 0);
        assert_eq!(get_wrapping_paper(&[0, 1, 1]), 2);
        assert_eq!(get_wrapping_paper(&[1, 1, 1]), 7);
        assert_eq!(get_wrapping_paper(&[1, 2, 1]), 11);
    }

    #[test]
    fn test_get_ribbon() {
        assert_eq!(get_ribbon(&[2, 3, 4]), 34);
        assert_eq!(get_ribbon(&[1, 1, 10]), 14);
    }

    #[test]
    fn test_parse_dims() {
        assert_eq!(parse_dims("0x0x0").unwrap(), [0, 0, 0]);
        assert_eq!(parse_dims("0x1x0").unwrap(), [0, 1, 0]);
        assert_eq!(parse_dims("123x4x789").unwrap(), [123, 4, 789]);
    }
}
