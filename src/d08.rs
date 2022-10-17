use crate::common::{print_parts, read_lines};
use ex::io;
use std::io::BufRead;

fn get_diff(line: impl AsRef<str>) -> usize {
    let l = line.as_ref();
    let code_count = l.len();
    let mut char_count = 0;
    let mut it = l.chars();
    loop {
        match it.next() {
            Some('"') => {}
            Some('\\') => {
                match it.next() {
                    Some('"') | Some('\\') => char_count += 1,
                    Some('x') => {
                        it.next();
                        it.next();
                        char_count += 1;
                    } // Consuming 2 more characters for escape code
                    _ => panic!("Invalid use of escape character '\\' in line: {}", l),
                }
            }
            Some(_) => char_count += 1,
            None => break,
        }
    }
    code_count - char_count
}

fn encode(line: impl AsRef<str>) -> String {
    let mut encoded = "\"".to_owned();
    let it = line.as_ref().chars().map(|c| match c {
        '"' => "\\\"".to_owned(),
        '\\' => "\\\\".to_owned(),
        c => c.to_string(),
    });
    encoded.extend(it);
    encoded.push('"');
    encoded
}

pub fn part1() -> io::Result<usize> {
    Ok(read_lines!("data/d08.txt")?.map(get_diff).sum::<usize>())
}

pub fn part2() -> io::Result<usize> {
    Ok(read_lines!("data/d08.txt")?
        .map(encode)
        .map(get_diff)
        .sum::<usize>())
}

pub fn main() {
    print_parts(8, part1(), part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_diff() {
        assert_eq!(
            read_lines!("data/test/d08.txt")
                .unwrap()
                .map(get_diff)
                .sum::<usize>(),
            12
        );
    }

    #[test]
    fn test_encode_diff() {
        assert_eq!(
            read_lines!("data/test/d08.txt")
                .unwrap()
                .map(encode)
                .map(get_diff)
                .sum::<usize>(),
            19
        );
    }
}
