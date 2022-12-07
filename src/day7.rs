#![feature(linked_list_remove)]

use std::borrow::Borrow;
use std::collections::{BinaryHeap, BTreeMap, HashMap, HashSet, LinkedList};
use std::fs::File;
use std::io::{BufReader, Lines};
use std::iter::Map;
use std::rc::Rc;
use regex::Regex;
use std::{env, fs, io, path};
use std::path::{Path, PathBuf};

struct Dir {
    files_sum: i32,
    sub_dirs: LinkedList<String>,
}

fn simplify_path(path: String) -> String  {
    let mut v : Vec<String> = Vec::new();
    let n : usize = path.len();
    let mut ans :String = "".to_string();
    let mut i : usize = 0;
    while  i < n {
         let mut dir : String = String::from("");
        // forming the current directory.
        while i < n && path.chars().nth(i).unwrap() != '/' {
            dir += path.chars().nth(i).unwrap().to_string().as_str();
            i += 1;
        }

        // if ".." , we pop.
        if dir == ".." {
            if !v.is_empty() {
                // pop back
                v.pop();
            }
        }
        else if dir == "." || dir == "" {
        // do nothing (added for better understanding.)
        } else {
        // push the current directory into the vector.
            v.push(dir);
        }
        i += 1;
    }

    // forming the ans
    for i in v {
        ans += ("/".to_owned() + &i).as_str();
    }

    // vector is empty
    if ans == "" {
        return String::from("/"); 
    }

    return ans.clone();
}

fn getDirSize(dirs : &BTreeMap<String, Dir>, dir : &String) -> i32 {
    let dir : &Dir = dirs.get(dir.as_str()).unwrap();
    let mut size : i32 = dir.files_sum;
    for subdir in dir.sub_dirs.iter()  {
        size += getDirSize(&dirs, &subdir);
    }
    return size;
}

pub fn day7(lines: Lines<BufReader<File>>) {

    let mut dirs : BTreeMap<String, Dir> = BTreeMap::new();

    let mut current_dir = String::from("");
    let mut in_ls_mod : bool = false;

    for line in lines {
        let ip = line.unwrap();

        if ip.is_empty() {
            break;
        }

        if ip.chars().nth(0).unwrap() == '$' {
            in_ls_mod = false;
        }

        if in_ls_mod {
            if ip.chars().nth(0).unwrap() == 'd' {
                let re = Regex::new(r"dir ([^ ]*)").unwrap();
                for cap in re.captures_iter(&ip) {
                    let sub_dir = simplify_path(current_dir.clone() + "/" + cap[1].to_string().as_str());
                    // println!("dirname {}", sub_dir);

                    if ! dirs.contains_key(&sub_dir) {
                        dirs.insert(sub_dir.clone(), Dir { files_sum: 0, sub_dirs: Default::default() });
                    }

                    dirs.get_mut(&current_dir).unwrap().sub_dirs.push_front(sub_dir.clone());
                }
            } else {
                let re = Regex::new(r"([0-9]*) ([A-Za-z]*)").unwrap();
                for cap in re.captures_iter(&ip) {
                    let file_size = cap[1].to_string().parse::<i32>().unwrap();
                    let file_name = cap[2].to_string();
                    dirs.get_mut(&current_dir).unwrap().files_sum += file_size;
                }
            }
            continue;
        }

        if ip.chars().nth(0).unwrap() == '$' {

            if ip.chars().nth(2).unwrap() == 'l' {
                in_ls_mod = true;
            } else if ip.chars().nth(2).unwrap() == 'c' {
                let re = Regex::new(r"\$ cd ([^ ]*)").unwrap();
                for cap in re.captures_iter(&ip) {
                    let mut dir = cap[1].to_string();

                    current_dir = simplify_path(current_dir.clone() + "/" + &dir);

                    if ! dirs.contains_key(&current_dir) {
                        dirs.insert(current_dir.clone(), Dir { files_sum: 0, sub_dirs: Default::default() });
                    }
                }
            } else {
                panic!();
            }
        }
    }

    let mut to_be_freed : i32 = 30000000 - (70000000 - getDirSize(&dirs, &String::from("/")));

    let mut top_sizes : Vec<i32> = Vec::new();

    for dir in dirs.iter() {
        let dir_size : i32 = getDirSize(&dirs, &dir.0);

        if dir_size >= to_be_freed {
            top_sizes.push(dir_size);
        }
    }
    top_sizes.sort();

    println!("smallest {}", top_sizes.get(0).unwrap());

}