extern crate core;

use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, Read};

fn read_from_file() -> HashMap<String, HashMap<String, Vec<usize>>> {
    let mut file = File::open("input");
    let result: HashMap<String, HashMap<String, Vec<usize>>> = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let mut result: HashMap<String, HashMap<String, Vec<usize>>> = HashMap::new();
            for line in lines {
                if let Ok(line) = line {
                    let mut splits = line.split(":");
                    let id = splits.nth(0).unwrap().split(" ").nth(1).unwrap();
                    let mut game_map = HashMap::new();
                    game_map.insert("blue".to_string(), vec![]);
                    game_map.insert("red".to_string(), vec![]);
                    game_map.insert("green".to_string(), vec![]);

                    let rounds = splits.nth(0).unwrap().split(";").collect::<Vec<&str>>();
                    println!("{:?}", &rounds);
                    rounds.iter().for_each(|round| {
                        let round_splitted = round.trim().split(",").collect::<Vec<&str>>();
                        println!("{:?}", &round_splitted);
                        round_splitted.iter().for_each(|s| {
                            let splitted = s.trim().split(" ").collect::<Vec<&str>>();
                            println!("{:?}", &splitted);
                            if splitted[1] == "blue" {
                                game_map
                                    .get_mut("blue")
                                    .unwrap()
                                    .push(splitted[0].parse::<usize>().unwrap());
                                println!("{:?}", &game_map);
                            } else if splitted[1] == "red" {
                                game_map
                                    .get_mut("red")
                                    .unwrap()
                                    .push(splitted[0].parse::<usize>().unwrap());
                            } else if splitted[1] == "green" {
                                game_map
                                    .get_mut("green")
                                    .unwrap()
                                    .push(splitted[0].parse::<usize>().unwrap());
                            }
                        });
                    });
                    println!("{:?}", &game_map);
                    result.insert(id.to_string(), game_map);
                }
            }
            result
        }
        Err(e) => panic!("Cannot process file: {}", e),
    };
    result
}

fn main() {
    let content: HashMap<String, HashMap<String, Vec<usize>>> = read_from_file();
    let red = 12;
    let blue = 14;
    let green = 13;
    let mut sum = 0;

    for game in &content {
        println!("{:?}", game);
        let red_max = game.1.get("red").unwrap().iter().max().unwrap();
        let blue_max = game.1.get("blue").unwrap().iter().max().unwrap();
        let green_max = game.1.get("green").unwrap().iter().max().unwrap();

        if red_max <= &red && blue_max <= &blue && green_max <= &green {
            println!("{:?}", game.0);
            sum += game.0.parse::<usize>().unwrap();
        }
    }
    println!("{:?}", sum);

    sum = 0;
    for game in &content {
        let red_max = game.1.get("red").unwrap().iter().max().unwrap();
        let blue_max = game.1.get("blue").unwrap().iter().max().unwrap();
        let green_max = game.1.get("green").unwrap().iter().max().unwrap();

        sum += red_max * blue_max * green_max;
    }
    println!("{:?}", sum);
}
