pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;

use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Lines};
use std::iter::Map;

pub fn get_file_lines(name: &str) -> Map<Lines<BufReader<File>>, fn(io::Result<String>) -> String> {
    let file = File::open(name).unwrap();
    BufReader::new(file).lines().map(|line| line.unwrap())
}
