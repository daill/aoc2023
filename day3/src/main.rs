extern crate core;

use std::fs::File;
use std::io;
use std::io::{BufRead, Read};

fn read_from_file() -> (
    Vec<(String, (usize, usize))>,
    Vec<(String, (usize, Vec<usize>))>,
) {
    let mut file = File::open("input");
    let result: (
        Vec<(String, (usize, usize))>,
        Vec<(String, (usize, Vec<usize>))>,
    ) = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let mut numbers: Vec<(String, (usize, Vec<usize>))> = Vec::new();
            let mut controls: Vec<(String, (usize, usize))> = Vec::new();
            for (i, line) in lines.enumerate() {
                if let Ok(line) = line {
                    let chars = line.chars();
                    let mut number: String = "".to_string();
                    let mut coords = vec![];
                    for (j, c) in chars.enumerate() {
                        if c.is_digit(10) {
                            number.push(c);
                            coords.push(j);
                        } else {
                            if c == '&'
                                || c == '|'
                                || c == '-'
                                || c == '='
                                || c == '$'
                                || c == '#'
                                || c == '!'
                                || c == '*'
                            {
                                controls.push((c.to_string(), (i, j)));
                            }
                            if number.len() > 0 && number != "" {
                                numbers.push((number.clone(), (i, coords)));
                                number = "".to_string();
                                coords = vec![];
                            }
                        }
                        if i == (line.len() - 1) && number.len() > 0 && number != "" {
                            numbers.push((number.clone(), (i, coords)));
                            number = "".to_string();
                            coords = vec![];
                        }
                    }
                }
            }
            (controls, numbers)
        }
        Err(e) => panic!("Cannot process file: {}", e),
    };
    result
}

fn main() {
    let content: (
        Vec<(String, (usize, usize))>,
        Vec<(String, (usize, Vec<usize>))>,
    ) = read_from_file();
    println!("{:?}", content);
}
