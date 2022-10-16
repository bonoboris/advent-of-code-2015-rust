use crate::common::{print_parts, read_to_string};
use ex::io;

fn iter_decode(it_chars: impl IntoIterator<Item = char>) -> impl Iterator<Item = isize> {
    it_chars.into_iter().map(|c| match c {
        '(' => 1,
        ')' => -1,
        _ => 0,
    })
}

fn get_floors(content: &str) -> isize {
    iter_decode(content.chars()).sum()
}

fn get_pos_into_basement(content: &str) -> Option<usize> {
    let mut floor = 0;
    iter_decode(content.chars())
        .position(|m| {
            floor += m;
            floor == -1
        })
        .and_then(|p| Some(p + 1))
}

fn part1() -> io::Result<isize> {
    let content = read_to_string("data/d01.txt")?;
    Ok(get_floors(&content))
}
fn part2() -> io::Result<usize> {
    let content = read_to_string("data/d01.txt")?;
    Ok(get_pos_into_basement(&content).unwrap_or_default())
}

pub fn main() {
    print_parts(1, part1(), part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_floors() {
        assert_eq!(get_floors(""), 0);
        assert_eq!(get_floors("("), 1);
        assert_eq!(get_floors("(()"), 1);
        assert_eq!(get_floors("())(("), 1);
        assert_eq!(get_floors("(a) )f( ("), 1);
    }

    #[test]
    fn test_get_pos_into_basement() {
        assert_eq!(get_pos_into_basement(""), None);
        assert_eq!(get_pos_into_basement("("), None);
        assert_eq!(get_pos_into_basement("(()))"), Some(5));
        assert_eq!(get_pos_into_basement("(()))())()(()())()()()"), Some(5));
    }
}
