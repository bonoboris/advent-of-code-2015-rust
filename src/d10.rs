use crate::common::print_parts;

fn get_next(cur: &str) -> String {
    let mut ret: String = String::new();
    let mut it = cur.chars();
    let mut cur = it.next().expect("Iterator shouldn't be empty here");
    let mut cur_count = 1;
    loop {
        match it.next() {
            Some(c) => {
                if c == cur {
                    cur_count += 1
                } else {
                    ret.extend(cur_count.to_string().chars());
                    ret.push(cur);
                    cur = c;
                    cur_count = 1;
                }
            }
            None => {
                ret.extend(cur_count.to_string().chars());
                ret.push(cur);
                break;
            }
        }
    }
    ret
}

fn solve(input: &str, n: usize) -> usize {
    let mut ret = input.to_owned();
    for _ in 0..n {
        ret = get_next(&ret).to_owned();
    }
    ret.len()
}

pub fn part1() -> Result<usize, String> {
    let input = "1113222113";
    Ok(solve(input, 40))
}
pub fn part2() -> Result<usize, String> {
    let input = "1113222113";
    Ok(solve(input, 50))
}

pub fn main() {
    print_parts(10, part1(), part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_next() {
        assert_eq!(get_next("1"), "11");
        assert_eq!(get_next("11"), "21");
        assert_eq!(get_next("21"), "1211");
        assert_eq!(get_next("1211"), "111221");
        assert_eq!(get_next("111221"), "312211");
    }
}
