use ex::fs;
use ex::io;
use std::fmt::Display;
use std::path::Path;

pub fn read_to_string<P>(filename: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    fs::read_to_string(filename)
}
macro_rules! read_lines {
    ($filepath:expr) => {
        ::ex::fs::File::open($filepath).and_then(|file| {
            Ok(::std::io::BufReader::new(file)
                .lines()
                .filter_map(|l| l.ok()))
        })
    };
}

pub(crate) use read_lines;

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
