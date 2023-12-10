extern crate core;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, Read};

fn read_from_file() -> Vec<(String, i32, u64, i32, u64)> {
    let mut file = File::open("input");
    let result: Vec<(String, i32, u64, i32, u64)> = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let mut result: Vec<(String, i32, u64, i32, u64)> = Vec::new();
            for line in lines {
                if let Ok(line) = line {
                    let mut splits = line.split(' ');
                    let mut cards = splits.next().unwrap();
                    let win = splits.next().unwrap().parse::<i32>().unwrap();
                    let chars = cards.trim().chars().collect::<Vec<char>>();
                    let mut elements: i32 = 0b000000000000000;
                    let mut counts:u64 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
                    let base: u64 = 2;

                    let max = cards
                        .trim()
                        .chars()
                        .collect::<Vec<char>>()
                        .into_iter()
                        .fold(HashMap::<char, usize>::new(), |mut m, x| {
                            *m.entry(x).or_default() += 1;
                            m
                        });

                    max.sort_keys(|a, b| {
                        if a
                    });

                    println!("{:?}", max);
                    // A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2
                    for c in &chars {
                        elements |= match c {
                            'A' => {
                                counts +=
                                    base.pow(14 * 4) * (((counts / base.pow(14 * 4)) & 15) + 1);
                                1 << 14
                            }
                            'K' => {
                                counts +=
                                    base.pow(13 * 4) * (((counts / base.pow(13 * 4)) & 15) + 1);
                                1 << 13
                            }
                            'Q' => {
                                counts +=
                                    base.pow(12 * 4) * (((counts / base.pow(12 * 4)) & 15) + 1);
                                1 << 12
                            }
                            'J' => {
                                counts +=
                                    base.pow(11 * 4) * (((counts / base.pow(11 * 4)) & 15) + 1);
                                1 << 11
                            }
                            'T' => {
                                counts +=
                                    base.pow(10 * 4) * (((counts / base.pow(10 * 4)) & 15) + 1);
                                1 << 10
                            }
                            '9' => {
                                counts += base.pow(9 * 4) * (((counts / base.pow(9 * 4)) & 15) + 1);
                                1 << 9
                            }
                            '8' => {
                                counts += base.pow(8 * 4) * (((counts / base.pow(8 * 4)) & 15) + 1);
                                1 << 8
                            }
                            '7' => {
                                counts += base.pow(7 * 4) * (((counts / base.pow(7 * 4)) & 15) + 1);
                                1 << 7
                            }
                            '6' => {
                                counts += base.pow(6 * 4) * (((counts / base.pow(6 * 4)) & 15) + 1);
                                1 << 6
                            }
                            '5' => {
                                counts += base.pow(5 * 4) * (((counts / base.pow(5 * 4)) & 15) + 1);
                                1 << 5
                            }
                            '4' => {
                                counts += base.pow(4 * 4) * (((counts / base.pow(4 * 4)) & 15) + 1);
                                1 << 4
                            }
                            '3' => {
                                counts += base.pow(3 * 4) * (((counts / base.pow(3 * 4)) & 15) + 1);
                                1 << 3
                            }
                            '2' => {
                                counts += base.pow(2 * 4) * (((counts / base.pow(2 * 4)) & 15) + 1);
                                1 << 2
                            }
                            _ => panic!("Unknown card: {}", c),
                        };
                    }
                    let mut num = 1;
                    println!("{:b}", elements);
                    if ((elements / (elements & -elements)) == 31
                        || (elements == 0b0100000000111100))
                    {
                        num = 3;
                    }
                    let mut rank_num = counts % 15 - num;
                    if rank_num == 0 {
                        if elements.count_ones() == 1 {
                            rank_num = 20;
                        } else {
                            rank_num = 18;
                        }
                    }
                    if rank_num == 6 {
                        rank_num = 16;
                    }
                    if rank_num == 16 {
                        rank_num = 7;
                    }
                    if rank_num == 2 {
                        rank_num = 4;
                    }

                    println!("{:0>15b} {} {}", elements, line, rank_num);
                    result.push((cards.to_string(), elements, counts, win, rank_num));
                }
            }
            result
        }
        Err(e) => panic!("Cannot process file: {}", e),
    };
    result
}

fn get_value(a: char) -> i32 {
    match a {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => a.to_digit(10).unwrap() as i32,
    }
}

fn main() {
    let mut content: Vec<(String, i32, u64, i32, u64)> = read_from_file();

    content.sort_by(|a, b| {
        a.4.cmp(&b.4).then({
            let a_cards = a.0.chars().collect::<Vec<char>>();
            let b_cards = b.0.chars().collect::<Vec<char>>();
            let mut order = Ordering::Equal;
            let mut char_a = 0;
            let mut char_b = 0;
            for i in 0..5 {
                char_a = get_value(a_cards[i]);
                char_b = get_value(b_cards[i]);
                if char_a != char_b {
                    order = char_a.cmp(&char_b);
                    break;
                }
            }
            order
        })
    });
    println!("{:?}", content);

    let mut sum = 0;
    for (i, set) in content.iter().enumerate() {
        sum += set.3 * (i + 1) as i32;
    }

    println!("{}", sum);
}
