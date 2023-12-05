extern crate core;

use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, Read};

fn read_from_file() -> Vec<(i32, Vec<i32>, Vec<i32>)> {
    let mut file = File::open("input");
    let result: Vec<(i32, Vec<i32>, Vec<i32>)> = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let mut result: Vec<(i32, Vec<i32>, Vec<i32>)> = Vec::new();
            for line in lines {
                if let Ok(line) = line {
                    let mut splits = line.split(":");
                    let mut card_info = splits.next().unwrap();
                    let mut cards = splits.next().unwrap().split(" | ");
                    let wins = cards.next().unwrap().trim();
                    println!("{}", wins);
                    let wins_nums = wins
                        .split(" ")
                        .filter(|x| !x.is_empty())
                        .map(|x| {
                            println!("{}", x);
                            x.parse::<i32>().unwrap()
                        })
                        .collect::<Vec<i32>>();
                    let numbers = cards.next().unwrap().trim();
                    println!("{}", numbers);
                    let numbers_nums = numbers
                        .split(" ")
                        .filter(|x| !x.is_empty())
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();
                    let card_id = card_info.split(" ").last().unwrap().parse::<i32>().unwrap();

                    result.push((card_id, wins_nums, numbers_nums));
                }
            }
            result
        }
        Err(e) => panic!("Cannot process file: {}", e),
    };
    result
}

fn main() {
    let content: Vec<(i32, Vec<i32>, Vec<i32>)> = read_from_file();

    let mut sum = 0;
    /*    for (card, winn, nums) in &content {
            let mut multi = 0;
            for win in winn {
                if nums.contains(&win) {
                    if multi == 0 {
                        multi = 1;
                    } else {
                        multi *= 2;
                    }
                }
            }
            sum += multi;
        }
        println!("{:?}", sum);
    */

    let mut card_map = HashMap::<i32, i32>::new();

    for (card, winn, nums) in &content {
        println!("{:?}", card);
        let mut l = 1;
        if card_map.contains_key(&card) {
            l += card_map.get(&card).unwrap();
        }

        let mut multi = 0;
        for win in winn {
            if nums.contains(&win) {
                multi += 1;
            }
        }

        for i in 0..multi {
            let mut val = l;
            if card_map.contains_key(&(i + card)) {
                val += card_map.get(&(i + card)).unwrap();
            }
            card_map.insert(i + card, val);
            println!("{:?}", card_map);
        }
    }
}
