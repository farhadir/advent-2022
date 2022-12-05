use std::collections::{BinaryHeap};
use std::fs::File;
use std::io::{BufReader, Lines};
use std::rc::Rc;
use regex::Regex;

fn print_stack(stacks : &Vec<Vec<char>>) {
    let mut lasts : Vec<char> = Vec::new();
    for test in stacks {
        lasts.push(*test.last().unwrap());
    }

    println!("");
    for last in lasts {
        print!("{}", last);
    }
    println!("");
}

pub fn day5(lines: Lines<BufReader<File>>) {
    const SIZE : usize = 9;
    let mut stacks = vec![Vec::new(); SIZE];

    let mut instruction_started = false;

    for line in lines {
        let ip = line.unwrap().clone() ;
        if instruction_started {
            let re = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
            for cap in re.captures_iter(&ip) {
                let mut move_count = cap[1].parse::<usize>().unwrap();
                let mut from = cap[2].parse::<usize>().unwrap();
                let mut to = cap[3].parse::<usize>().unwrap();

                let index = stacks[from - 1].len() - move_count;

                let mut new_test = stacks[from - 1][index..].to_owned();
                stacks[to - 1].extend(new_test);
                stacks[from - 1].drain( index..);

                // for _ in 0..move_count {
                //     let c = stacks[from - 1].pop().unwrap();
                //     stacks[to - 1].push(c);
                // }
            }
        } else if !ip.is_empty() {
            for i in 0..SIZE {
                if ip.chars().nth(4 * i + 1).unwrap().is_alphabetic() {
                    let mut vec = stacks.get_mut(i).unwrap();
                    vec.push(ip.chars().nth(4 * i + 1).unwrap());
                }
            }
        } else {
            instruction_started = true;
            for mut vec in stacks.iter_mut() {
                vec.reverse();
            }
        }
    }
    print_stack(&stacks);

}