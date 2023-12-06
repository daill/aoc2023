extern crate core;

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, Read};

fn read_from_file() -> (Vec<i32>, HashMap<String, Vec<(i32, i32, i32)>>) {
    let mut file = File::open("input");
    let result: (Vec<i32>, HashMap<String, Vec<(i32, i32, i32)>>) = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let mut result: HashMap<String, Vec<(i32, i32, i32)>> = HashMap::new();
            let mut seeds: Vec<i32> = Vec::new();

            let re = Regex::new(r"([0-9]+)\s([0-9]+)\s([0-9]+)").unwrap();

            for line in lines {
                if let Ok(line) = line {
                    if line.starts_with("seeds:") {
                        seeds = line
                            .split(" ")
                            .filter_map(|s| s.parse::<i32>().ok())
                            .collect();
                    } else {
                        let mut current_line = "".to_string();

                        if line.ends_with("map:") {
                            current_line = line.clone();
                            continue;
                        }

                        let mut numbers = Vec::new();


                        if !line.is_empty() {
                            let Some(caps) = re.captures(&line) else {
                                continue;                                
                            };
                            let (a, b, c) = (
                                &caps[1].parse::<i32>().unwrap(),
                                &caps[2].parse::<i32>().unwrap(),
                                &caps[3].parse::<i32>().unwrap(),
                            );
                            numbers.push((*a, *b, *c));
                            println!("{:?}", numbers);
                        } else {
                            result.insert(current_line.to_string(), numbers);
                            println!("{:?}", result);
                            current_line = "".to_string();
                        }
                    }
                }
            }
            (seeds, result)
        }
        Err(e) => panic!("Cannot process file: {}", e),
    };
    result
}

fn main() {
    let content: (Vec<i32>, HashMap<String, Vec<(i32, i32, i32)>>) = read_from_file();
    println!("{:?}", content);
}
