extern crate core;

use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, Read};

fn read_from_file() -> Vec<(String, i64)> {
    let mut file = File::open("input");
    let result: Vec<(String, i64)> = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let mut result: Vec<(String, i64)> = Vec::new();
            let A=14
                let K=13 Q=12, J=11;
            for line in lines {
                if let Ok(line) = line {
                    let mut splits = line.split(' ');
                    let mut cards = splits.next().unwrap();
                    let mut chars = cards.trim().chars().collect::<Vec<char>>();
                    let mut elements = 0b000000000000000;
                    let mut counts = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
                    // A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2

                    elements
                    
                    result.push((
                        chars.iter().collect::<String>(),
                        splits.next().unwrap().parse::<i64>().unwrap(),
                    ));
                }
            }
            result
        }
        Err(e) => panic!("Cannot process file: {}", e),
    };
    result
}

fn main() {
    let mut content: Vec<(String, i64)> = read_from_file();
    content.sort_by(|a, b| a.0.cmp(&b.0).reverse());
    println!("{:?}", content);
}
