use std::collections::BTreeSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn day1() {
    let mut s = BTreeSet::new();
    if let Ok(lines) = read_lines("../day1_input.txt") {
        let mut energy: u64 = 0;
        for line in lines {
            if let Ok(ip) = line {
                if ip.is_empty() {
                    s.insert(energy);
                    energy = 0;
                } else {
                    energy += ip.parse::<u64>().unwrap();
                }
            }
        }
    }
    println!("Last element is {}", s.iter().next_back().unwrap());
}