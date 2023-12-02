extern crate core;

use regex::Regex;
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
                    let re = Regex::new(
                        r"(oneight|twone|eightwo|one|two|three|four|five|six|seven|eight|nine)",
                    )
                    .unwrap();
                    let mut return_line = line.clone();
                    println!("before: {}", return_line);
                    for (_, [cap]) in re.captures_iter(&line).map(|c| c.extract()) {
                        let rep = match cap {
                            "one" => "1",
                            "two" => "2",
                            "three" => "3",
                            "four" => "4",
                            "five" => "5",
                            "six" => "6",
                            "seven" => "7",
                            "eight" => "8",
                            "nine" => "9",
                            "oneight" => "18",
                            "twone" => "21",
                            "eightwo" => "82",
                            _ => "",
                        };
                        return_line = return_line.replacen(cap, rep, 1);
                        println!("replaced: {}", return_line);
                    }

                    println!("after: {}", return_line);

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
    let mut sum: u64 = 0;
    for str in content {
        println!("{}", str);
        let chars = str.chars();
        let mut first = '0';
        let mut last = '0';
        for c in chars {
            if c.is_digit(10) {
                if first == '0' {
                    first = c;
                }
                last = c;
            }
        }
        let num = format!("{}{}", first, last).parse::<u64>().unwrap();
        println!("{0}", num);
        sum += num;
    }
    println!("{}", sum);
}
