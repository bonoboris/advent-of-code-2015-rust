use crate::common::{print_parts, read_lines};
use ex::io;
use itertools::Itertools;
use std::io::BufRead;

type Point = (usize, usize);

#[derive(Debug, PartialEq, Eq)]
enum Action {
    Toggle,
    TurnOn,
    TurnOff,
}

type Command = (Action, Point, Point);

fn parse_coords(coords_str: &str) -> Option<Point> {
    let (sx, sy) = coords_str.split_once(",")?;
    Some((sx.parse().ok()?, sy.parse().ok()?))
}

fn parse_line(line: &str) -> Option<Command> {
    match line.split_whitespace().collect_vec().as_slice() {
        ["toggle", c1, "through", c2] => {
            Some((Action::Toggle, parse_coords(c1)?, parse_coords(c2)?))
        }
        ["turn", "on", c1, "through", c2] => {
            Some((Action::TurnOn, parse_coords(c1)?, parse_coords(c2)?))
        }
        ["turn", "off", c1, "through", c2] => {
            Some((Action::TurnOff, parse_coords(c1)?, parse_coords(c2)?))
        }
        _ => None,
    }
}

fn apply_command_1(state: &mut Vec<Vec<u8>>, command: &Command) {
    let xi = (command.1).0;
    let xj = (command.2).0;
    let yi = (command.1).1;
    let yj = (command.2).1;
    for x in xi..(xj + 1) {
        for y in yi..(yj + 1) {
            state[x][y] = match command.0 {
                Action::TurnOff => 0,
                Action::TurnOn => 1,
                Action::Toggle => 1 - state[x][y],
            }
        }
    }
}

fn apply_command_2(state: &mut Vec<Vec<u8>>, command: &Command) {
    let xi = (command.1).0;
    let xj = (command.2).0;
    let yi = (command.1).1;
    let yj = (command.2).1;
    for x in xi..(xj + 1) {
        for y in yi..(yj + 1) {
            state[x][y] = match command.0 {
                Action::TurnOff => state[x][y].saturating_sub(1),
                Action::TurnOn => state[x][y] + 1,
                Action::Toggle => state[x][y] + 2,
            }
        }
    }
}

fn get_total_brightness(state: &Vec<Vec<u8>>) -> usize {
    state
        .iter()
        .flat_map(|row| row.iter())
        .map(|x| *x as usize)
        .sum::<usize>()
}

pub fn part1() -> io::Result<usize> {
    let mut state = vec![vec![0 as u8; 1000]; 1000];
    read_lines!("data/d06.txt")?
        .filter_map(|l| parse_line(&l))
        .for_each(|c| apply_command_1(&mut state, &c));
    Ok(get_total_brightness(&state))
}

pub fn part2() -> io::Result<usize> {
    let mut state = vec![vec![0 as u8; 1000]; 1000];
    read_lines!("data/d06.txt")?
        .filter_map(|l| parse_line(&l))
        .for_each(|c| apply_command_2(&mut state, &c));
    Ok(get_total_brightness(&state))
}

pub fn main() {
    print_parts(6, part1(), part2());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("toggle 461,550 through 564,900"),
            Some((Action::Toggle, (461, 550), (564, 900)))
        );
        assert_eq!(
            parse_line("turn off 370,39 through 425,839"),
            Some((Action::TurnOff, (370, 39), (425, 839)))
        );
        assert_eq!(
            parse_line("turn on 599,989 through 806,993"),
            Some((Action::TurnOn, (599, 989), (806, 993)))
        );
    }
    #[test]
    fn test_apply_command_1() {
        let mut state = vec![vec![0 as u8; 4]; 4];
        apply_command_1(&mut state, &(Action::TurnOn, (0, 0), (1, 1)));
        assert_eq!(
            state,
            vec![
                vec![1, 1, 0, 0],
                vec![1, 1, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
            ]
        );
        apply_command_1(&mut state, &(Action::TurnOff, (1, 0), (2, 1)));
        assert_eq!(
            state,
            vec![
                vec![1, 1, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
            ]
        );
        apply_command_1(&mut state, &(Action::Toggle, (0, 1), (1, 2)));
        assert_eq!(
            state,
            vec![
                vec![1, 0, 1, 0],
                vec![0, 1, 1, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
            ]
        );
    }

    #[test]
    fn test_apply_command_2() {
        let mut state = vec![vec![0 as u8; 4]; 4];
        apply_command_2(&mut state, &(Action::TurnOn, (0, 0), (1, 1)));
        assert_eq!(
            state,
            vec![
                vec![1, 1, 0, 0],
                vec![1, 1, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
            ]
        );
        apply_command_2(&mut state, &(Action::TurnOff, (1, 0), (2, 1)));
        assert_eq!(
            state,
            vec![
                vec![1, 1, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
            ]
        );
        apply_command_2(&mut state, &(Action::Toggle, (0, 1), (1, 2)));
        assert_eq!(
            state,
            vec![
                vec![1, 3, 2, 0],
                vec![0, 2, 2, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
            ]
        );
    }

    #[test]
    fn test_get_total_brightness() {
        let state = vec![
            vec![1, 3, 2, 0],
            vec![0, 2, 2, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];
        assert_eq!(get_total_brightness(&state), 10);
    }
}
