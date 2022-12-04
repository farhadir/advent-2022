use std::collections::{BinaryHeap};
use std::fs::File;
use std::io::{BufReader, Lines};

fn split_range(line : &str) -> (i32, i32) {
    let range = line.split('-').collect::<Vec<&str>>();
    return (range.get(0).unwrap().parse::<i32>().unwrap(), range.get(1).unwrap().parse::<i32>().unwrap())
}

pub fn day4(lines: Lines<BufReader<File>>) {
    let mut silver: i32 = 0;
    let mut gold: i32 = 0;
    for line in lines {
        if let Ok(ip) = line {
            let ranges = ip.split(',').collect::<Vec<&str>>();

            let first_range = split_range(ranges.get(0).unwrap());

            let second_range = split_range(ranges.get(1).unwrap());

            if (first_range.1 >= second_range.0 && first_range.0 <= second_range.1) || (second_range.1 >= first_range.0 && first_range.0 >= second_range.0)  {
                gold += 1;
            }

            if first_range.0 <= second_range.0 && first_range.1 >= second_range.1 ||
                second_range.0 <= first_range.0 && second_range.1 >= first_range.1 {
                silver += 1;
            }
        }
    }

    println!("total count silver {} gold {}", silver, gold);
}