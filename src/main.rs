mod day1;
mod day2;
mod day3;

use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    // day1::day1(read_lines("day1_input.txt").unwrap());
    // day2::day2(read_lines("day2_input.txt").unwrap());
    day3::day3(read_lines("day3_input.txt").unwrap());
}
