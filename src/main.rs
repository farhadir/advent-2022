mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

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
    // day3::day3(read_lines("day3_input.txt").unwrap());
    // day4::day4(read_lines("day4_input.txt").unwrap());
    day5::day5(read_lines("day5_input.txt").unwrap());
}
