use crate::common::{print_parts, read_lines};
use ex::io;
use std::io::BufRead;
use std::{char, collections::HashMap};

fn is_vowel(c: char) -> bool {
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}

fn is_forbidden_pair(c1: char, c2: char) -> bool {
    (c1, c2) == ('a', 'b')
        || (c1, c2) == ('c', 'd')
        || (c1, c2) == ('p', 'q')
        || (c1, c2) == ('x', 'y')
}

fn is_twin(c1: char, c2: char) -> bool {
    c1 == c2
}

fn is_nice1(s: &str) -> bool {
    let mut it = s.chars();
    let mut prev = it.next().unwrap_or_default();
    let mut vowel_count: u32 = 0;
    if is_vowel(prev) {
        vowel_count += 1;
    }

    let mut has_twin = false;
    for c in it {
        if is_vowel(c) {
            vowel_count += 1;
        }
        has_twin = has_twin || is_twin(prev, c);
        if is_forbidden_pair(prev, c) {
            return false;
        }
        prev = c
    }

    has_twin && vowel_count >= 3
}

fn has_sandwich(s: &str) -> bool {
    let mut has_sandwich = false;
    let mut it = s.chars();
    if it.clone().count() >= 3 {
        let mut p2 = it.next().unwrap_or_default();
        let mut p1 = it.next().unwrap_or_default();
        for c in it {
            if c == p2 {
                has_sandwich = true;
                break;
            }
            (p2, p1) = (p1, c)
        }
    }

    has_sandwich
}

fn has_repeating_pair(s: &str) -> bool {
    let mut bigrams_positions: HashMap<(char, char), Vec<usize>> = HashMap::new();
    let mut it = s.chars();
    if it.clone().count() >= 1 {
        let mut p = it.next().unwrap();
        for (i, c) in it.enumerate() {
            let bi = (p, c);
            if !bigrams_positions.contains_key(&bi) {
                bigrams_positions.insert(bi, vec![]);
            }
            bigrams_positions.get_mut(&bi).unwrap().push(i);
            p = c;
        }
    }
    for v in bigrams_positions.values() {
        if v.len() > 2  // Has a bigram repeating more than once ==> cannot be overlaping
            || (v.len() == 2 && v[1] - v[0] >= 2)
        {
            return true;
        }
    }
    return false;
}

fn is_nice2(s: &str) -> bool {
    has_sandwich(s) && has_repeating_pair(&s)
}

pub fn part1() -> io::Result<usize> {
    Ok(read_lines!("data/d05.txt")?.filter(|l| is_nice1(l)).count())
}

pub fn part2() -> io::Result<usize> {
    Ok(read_lines!("data/d05.txt")?.filter(|l| is_nice2(l)).count())
}

pub fn main() {
    print_parts(5, part1(), part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_vowel() {
        assert_eq!(is_vowel('a'), true);
        assert_eq!(is_vowel('b'), false);
        assert_eq!(is_vowel('c'), false);
        assert_eq!(is_vowel('d'), false);
        assert_eq!(is_vowel('e'), true);
        assert_eq!(is_vowel('i'), true);
        assert_eq!(is_vowel('o'), true);
        assert_eq!(is_vowel('u'), true);
        assert_eq!(is_vowel('y'), false);
    }

    #[test]
    fn test_is_forbidden_pair() {
        assert_eq!(is_forbidden_pair('a', 'a'), false);
        assert_eq!(is_forbidden_pair('a', 'b'), true);
        assert_eq!(is_forbidden_pair('b', 'a'), false);
        assert_eq!(is_forbidden_pair('a', 'c'), false);
        assert_eq!(is_forbidden_pair('c', 'd'), true);
        assert_eq!(is_forbidden_pair('p', 'q'), true);
        assert_eq!(is_forbidden_pair('x', 'y'), true);
    }

    #[test]
    fn test_is_twin() {
        assert_eq!(is_twin('a', 'a'), true);
        assert_eq!(is_twin('a', 'b'), false);
        assert_eq!(is_twin('b', 'b'), true);
        assert_eq!(is_twin('i', 'j'), false);
    }

    #[test]
    fn test_is_nice1() {
        assert_eq!(is_nice1("ugknbfddgicrmopn"), true);
        assert_eq!(is_nice1("aaa"), true);
        assert_eq!(is_nice1("jchzalrnumimnmhp"), false);
        assert_eq!(is_nice1("haegwjzuvuyypxyu"), false);
        assert_eq!(is_nice1("dvszwmarrgswjxmb"), false);
    }

    #[test]
    fn test_has_sandwich() {
        assert_eq!(has_sandwich("aaa"), true);
        assert_eq!(has_sandwich("xyx"), true);
        assert_eq!(has_sandwich("abcdefeghi"), true);
        assert_eq!(has_sandwich("abcdeffeghi"), false);
        assert_eq!(has_sandwich("qjhvhtzxzqqjkmpb"), true);
        assert_eq!(has_sandwich("ieodomkazucvgmuy"), true);
        assert_eq!(has_sandwich("uurcxstgmygtbstg"), false);
    }

    #[test]
    fn test_has_repeating_pair() {
        assert_eq!(has_repeating_pair("aaa"), false);
        assert_eq!(has_repeating_pair("xyx"), false);
        assert_eq!(has_repeating_pair("xyxy"), true);
        assert_eq!(has_repeating_pair("qjhvhtzxzqqjkmpb"), true);
        assert_eq!(has_repeating_pair("ieodomkazucvgmuy"), false);
        assert_eq!(has_repeating_pair("uurcxstgmygtbstg"), true);
    }

    #[test]
    fn test_is_nice2() {
        assert_eq!(is_nice2("qjhvhtzxzqqjkmpb"), true);
        assert_eq!(is_nice2("xxyxx"), true);
        assert_eq!(is_nice2("uurcxstgmygtbstg"), false);
        assert_eq!(is_nice2("ieodomkazucvgmuy"), false);
    }
}
