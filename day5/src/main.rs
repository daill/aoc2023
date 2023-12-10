extern crate core;

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, Read};
use std::{io, vec};

fn read_from_file() -> (Vec<i64>, HashMap<String, Vec<(i64, i64, i64)>>) {
    let mut file = File::open("input");
    let result: (Vec<i64>, HashMap<String, Vec<(i64, i64, i64)>>) = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let mut result: HashMap<String, Vec<(i64, i64, i64)>> = HashMap::new();
            let mut seeds: Vec<i64> = Vec::new();

            let re = Regex::new(r"([0-9]+)\s([0-9]+)\s([0-9]+)").unwrap();

            let mut numbers = Vec::new();

            let mut current_line = "".to_string();

            for line in lines {
                if let Ok(line) = line {
                    if line.starts_with("seeds:") {
                        seeds = line
                            .split(" ")
                            .filter_map(|s| s.parse::<i64>().ok())
                            .collect();
                    } else {
                        if line.ends_with("map:") {
                            current_line = line.clone();
                            continue;
                        }

                        if !line.is_empty() {
                            let Some(caps) = re.captures(&line) else {
                                continue;
                            };
                            let (a, b, c) = (
                                &caps[1].parse::<i64>().unwrap(),
                                &caps[2].parse::<i64>().unwrap(),
                                &caps[3].parse::<i64>().unwrap(),
                            );
                            numbers.push((*a, *b, *c));
                            println!("{:?}", numbers);
                        } else {
                            result.insert(current_line.to_string(), numbers.clone());
                            println!("{:?}", result);
                            current_line = "".to_string();
                            numbers = Vec::new();
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
    let order = vec![
        "seed-to-soil map:",
        "soil-to-fertilizer map:",
        "fertilizer-to-water map:",
        "water-to-light map:",
        "light-to-temperature map:",
        "temperature-to-humidity map:",
        "humidity-to-location map:",
    ];

    let (mut seeds, maps): (Vec<i64>, HashMap<String, Vec<(i64, i64, i64)>>) = read_from_file();
    let mut seed_solution = Vec::new();

    println!("see: {:?} {:?}", &seeds, &maps);

    for s in (0..seeds.len()).step_by(2) {
        for seed_num in seeds[s]..(seeds[s] + seeds[s + 1]) {
            let mut seed = seed_num.clone();
            'outer: for i in 0..order.len() {
                let ord = order[i];
                //println!("{}", ord);
                let mapping = &maps[ord];
                for map in mapping {
                    let range = map.1..=(map.1 + map.2 - 1);
                    if range.contains(&seed) {
                        //println!("{}", seed);
                        seed = map.0 + seed - map.1;
                        //println!("after {} in {} {:?}", seed, ord, map);
                        continue 'outer;
                    } else {
                        //println!("{} not in {} {:?}", seed, ord, range);
                    }
                }
            }
            seed_solution.push(seed);
        }
    }

    let solution = seed_solution.iter().min();

    println!("{:?} {:?}", &seeds, &maps);
    println!("Solution: {:?}", solution);
}
