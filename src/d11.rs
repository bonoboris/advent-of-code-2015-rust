use std::string::FromUtf8Error;

use crate::common::print_parts;

const A_U8: u8 = 'a' as u8;
const Z_U8: u8 = 'z' as u8;
const FORBIDEN: [u8; 3] = ['i' as u8, 'o' as u8, 'l' as u8];

fn has_straight_inc(pwd: &[u8]) -> bool {
    pwd.iter()
        .zip(pwd.iter().skip(1).zip(pwd.iter().skip(2)))
        .any(|(c1, (c2, c3))| c2 > c1 && c2 - c1 == 1 && c3 > c2 && c3 - c2 == 1)
}

fn has_no_forbidden_letter(pwd: &[u8]) -> bool {
    return pwd.iter().all(|c| !FORBIDEN.contains(c));
}

fn has_2_pairs(pwd: &[u8]) -> bool {
    let mut first_found = false;
    let mut pairwise_it = pwd.iter().zip(pwd.iter().skip(1));
    loop {
        match pairwise_it.next() {
            Some((c1, c2)) => {
                if c1 == c2 {
                    if !first_found {
                        first_found = true;
                        pairwise_it.next();
                    } else {
                        return true;
                    }
                }
            }
            None => break,
        }
    }
    false
}

fn is_pwd_valid(pwd: &[u8]) -> bool {
    has_straight_inc(pwd) && has_no_forbidden_letter(pwd) && has_2_pairs(pwd)
}

fn to_next_pwd(pwd: &mut [u8]) {
    let mut carry = true;
    pwd.reverse();
    for c in pwd.iter_mut() {
        match carry {
            true => {
                if *c + 1 > Z_U8 {
                    *c = A_U8;
                } else {
                    carry = false;
                    *c += 1;
                }
            }
            false => break,
        }
    }
    pwd.reverse();
}

pub fn part1() -> Result<String, FromUtf8Error> {
    let input = "cqjxjnds";
    let mut pwd = input.as_bytes().to_owned();
    while !is_pwd_valid(&pwd) {
        to_next_pwd(&mut pwd);
    }
    String::from_utf8(pwd)
}

pub fn part2() -> Result<String, FromUtf8Error> {
    let input = part1()?;
    let mut pwd = input.as_bytes().to_owned();
    to_next_pwd(&mut pwd);
    while !is_pwd_valid(&pwd) {
        to_next_pwd(&mut pwd);
    }
    String::from_utf8(pwd)
}

pub fn main() {
    print_parts(11, part1(), part2())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_straight_inc() {
        assert_eq!(has_straight_inc("aaaaaaaddddabd".as_bytes()), false);
        assert_eq!(has_straight_inc("abc".as_bytes()), true);
        assert_eq!(has_straight_inc("cbaaa".as_bytes()), false);
        assert_eq!(has_straight_inc("cbabc".as_bytes()), true);
    }

    #[test]
    fn test_has_no_forbidden_letter() {
        assert_eq!(has_no_forbidden_letter("aaa".as_bytes()), true);
        assert_eq!(has_no_forbidden_letter("aol".as_bytes()), false);
    }

    #[test]
    fn test_has_2_pairs() {
        assert_eq!(has_2_pairs("aaa".as_bytes()), false);
        assert_eq!(has_2_pairs("aaaa".as_bytes()), true);
        assert_eq!(has_2_pairs("aabcdefxx".as_bytes()), true);
        assert_eq!(has_2_pairs("abcdefffg".as_bytes()), false);
    }

    fn get_next_pwd(pwd: &[u8]) -> Vec<u8> {
        let mut new = pwd.to_owned();
        to_next_pwd(&mut new);
        new
    }

    #[test]
    fn test_get_next_pwd() {
        assert_eq!(get_next_pwd("aaa".as_bytes()), "aab".as_bytes());
        assert_eq!(get_next_pwd("aaaz".as_bytes()), "aaba".as_bytes());
        assert_eq!(get_next_pwd("azzz".as_bytes()), "baaa".as_bytes());
    }
}
