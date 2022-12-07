#![feature(linked_list_remove)]
use std::collections::{BinaryHeap, HashSet, LinkedList};
use std::fs::File;
use std::io::{BufReader, Lines};
use std::rc::Rc;
use regex::Regex;



pub fn day6(lines: Lines<BufReader<File>>) {
    const SIZE : usize = 14;
    let mut sequence : LinkedList<char> = LinkedList::new();
    let mut index : i32 = 0;
    for line in lines {
        let ip = line.unwrap();
        for c in ip.chars().into_iter() {
            index += 1;
            if sequence.len() >= SIZE {
                sequence.pop_front();
            }
            sequence.push_back(c);

            let mut uniq : HashSet<char> = HashSet::new();
            sequence.clone().into_iter().all(|x| uniq.insert(x));

            if uniq.clone().len() == SIZE {
                println!("{}", index);
                break;
            }

        }
    }


}