extern crate core;

use std::fs::File;
use std::io;
use std::io::{BufRead, Read};

fn read_from_file() -> Vec<String> {
    let mut file = File::open("input");
    let result: Vec<String> = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let mut result: Vec<String> = Vec::new();
            for line in lines {
                if let Ok(line) = line {
                    result.push(return_line);
                }
            }
            result
        }
        Err(e) => panic!("Cannot process file: {}", e),
    };
    result
}

fn main() {
    let content: Vec<String> = read_from_file();
    println!("{}", sum);
}
