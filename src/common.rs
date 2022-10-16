use ex::fs;
use ex::io;
use std::fmt::Display;
use std::io::{BufRead, BufReader, Lines};
use std::iter::FilterMap;
use std::path::Path;

pub struct LineIter(FilterMap<Lines<BufReader<fs::File>>, OkFn>);

impl Iterator for LineIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

pub fn read_to_string<P>(filename: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    fs::read_to_string(filename)
}

type OkFn = fn(std::io::Result<String>) -> Option<String>;

pub fn read_lines<P>(filename: P) -> io::Result<LineIter>
where
    P: AsRef<Path>,
{
    let file = fs::File::open(filename)?;
    fn ok(ret: std::io::Result<String>) -> Option<String> {
        ret.ok()
    }
    let ret = BufReader::new(file).lines().filter_map(ok as OkFn);
    Ok(LineIter(ret))
}

pub trait StringResults {
    fn unwrap_as_string(&self) -> String;
}

impl<T: ToString, E: ToString> StringResults for Result<T, E> {
    fn unwrap_as_string(&self) -> String {
        match self {
            Ok(v) => v.to_string(),
            Err(e) => e.to_string(),
        }
    }
}

pub fn print_parts(day: impl Display, ret1: impl StringResults, ret2: impl StringResults) {
    println!("[AOC 2015] Day {day:02}");
    println!("\t> part 1: {}", ret1.unwrap_as_string());
    println!("\t> part 2: {}", ret2.unwrap_as_string());
}
