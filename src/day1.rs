use std::collections::{BinaryHeap};
use std::fs::File;
use std::io::{BufReader, Lines};


pub fn day1(lines: Lines<BufReader<File>>) {
    let mut s = BinaryHeap::new();
    let mut energy: u64 = 0;
    for line in lines {
        if let Ok(ip) = line {
            if ip.is_empty() {
                s.push(energy);
                energy = 0;
            } else {
                energy += ip.parse::<u64>().unwrap();
            }
        }
    }

    println!("Last element is {}", s.peek().unwrap());

    // let test : u64 = s.into_iter().take(3).sum();
    let top3: u64 = s.pop().unwrap() + s.pop().unwrap() + s.pop().unwrap();
    println!("Top 3 {}", top3);
}