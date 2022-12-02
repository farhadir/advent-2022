use std::fs::File;
use std::io::{BufReader, Lines};

#[derive(Debug, Clone, Copy)]
enum Play {
    Rock = 0,
    Paper = 1,
    Scissors = 2
}

#[derive(Debug)]
enum MatchResult {
    Loose = 0,
    Draw = 1,
    Win = 2
}

pub fn day2(lines: Lines<BufReader<File>>) {
    let score_table_1 : [[u32; 3];3] = [
        // Rock   Paper     Scissors
        [  3 + 1, 0 + 1,    6 + 1],  // Rock
        [  6 + 2, 3 + 2,    0 + 2],  // Paper
        [  0 + 3, 6 + 3,    3 + 3]   // Scissors
    ];
    let mut  sum_1 : u32 = 0;

    let score_table_2 : [[u32; 3];3] = [
        // Rock   Paper     Scissors
        [  0 + 3, 0 + 1,    0 + 2],  // Loose
        [  3 + 1, 3 + 2,    3 + 3],  // Draw
        [  6 + 2, 6 + 3,    6 + 1]   // Win
    ];
    let mut  sum_2 : u32 = 0;

    for line in lines {
        if let Ok(ip) = line {
            let command = ip.split(' ').collect::<Vec<&str>>();
            let opponent = match command[0].chars().next().unwrap() {
                'A' => Play::Rock,
                'B' => Play::Paper,
                'C' => Play::Scissors,
                _ => panic!()
            };

            let strategy = match command[1].chars().next().unwrap() {
                'X' => Play::Rock,
                'Y' => Play::Paper,
                'Z' => Play::Scissors,
                _ => panic!()
            };

            let res_1 : u32 = score_table_1[strategy as usize][opponent as usize];
            sum_1 += res_1;

            let result = match command[1].chars().next().unwrap() {
                'X' => MatchResult::Loose,
                'Y' => MatchResult::Draw,
                'Z' => MatchResult::Win,
                _ => panic!()
            };

            let res_2 : u32 = score_table_2[result as usize][opponent as usize];

            sum_2 += res_2;
        }
    }

    println!("1: {} ", sum_1);
    println!("2: {} ", sum_2);
}