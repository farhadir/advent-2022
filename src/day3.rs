use std::fs::File;
use std::io::{BufReader, Lines};

pub fn day3(lines: Lines<BufReader<File>>) {
    let mut  priority_sum : u32 = 0;

    let mut  index : u32 = 0;
    const ARRAY_SIZE : usize = ('z' as i8 - 'A' as i8) as usize + 1;
    let mut repetition : [u8; ARRAY_SIZE] = [0; ARRAY_SIZE];

    for line in lines {
        if let Ok(ip) = line {
            if index % 3 == 0 {
                repetition = [0; ARRAY_SIZE];
            }
            index += 1;

            let compartment_len = ip.len();
            let first = &ip[..compartment_len];

            let mut repetition_current : [u8; ARRAY_SIZE] = [0; ARRAY_SIZE];

            for chr in first.chars() {

                let char_index = (chr as i8 - 'A' as i8) as usize;
                if repetition_current[char_index] == 0 {
                    repetition[char_index] += 1;
                    if repetition[char_index] == 3 {
                        let priority = if chr <= 'Z' { char_index as u32 + 27 } else {char_index as u32 - 31};
                        priority_sum += priority;
                        break;
                    }
                }
                repetition_current[char_index] += 1;
            }
        }
    }
    println!("priority_sum: {}", priority_sum);
}