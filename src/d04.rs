use crate::common::print_parts;
use ex::io;
use md5;

fn hash(s: &[u8], n: usize) -> md5::Digest {
    let data = [s, &n.to_string().as_ref()].concat();
    md5::compute(data.as_slice())
}

fn le(a: &[u8], b: &[u8]) -> bool {
    match a.iter().zip(b.iter()).find(|(ai, bi)| ai != bi) {
        Some((ai, bi)) => ai < bi,
        None => true,
    }
}

fn simple_mine(secret: &str, max: &[u8]) -> usize {
    let mut n: usize = 0;
    let secret_bytes = secret.as_ref();
    loop {
        if le(&hash(secret_bytes, n).0, max) {
            break;
        };
        n += 1;
    }
    n
}

pub fn part1() -> io::Result<usize> {
    let secret = "ckczppom";
    Ok(simple_mine(&secret, &[0, 0, 0x10]))
}

pub fn part2() -> io::Result<usize> {
    let secret = "ckczppom";
    Ok(simple_mine(&secret, &[0, 0, 0]))
}

pub fn main() {
    print_parts(4, part1(), part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "slow"]
    fn test_simple_mine() {
        assert_eq!(simple_mine("abcdef", &[0, 0, 10]), 609043);
        assert_eq!(simple_mine("pqrstuv", &[0, 0, 10]), 1048970);
    }
}
