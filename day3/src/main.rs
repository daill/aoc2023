extern crate core;

use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, Read};
use std::iter::Map;

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
                        if c.is_ascii_digit() {
                            number.push(c);
                            coords.push(j);
                            println!("{} {} {:?}", c, j, number);
                        } else {
                            if c == '&'
                                || c == '|'
                                || c == '-'
                                || c == '='
                                || c == '$'
                                || c == '#'
                                || c == '!'
                                || c == '*'
                                || c == '+'
                                || c == '/'
                                || c == '\\'
                                || c == '@'
                                || c == '%'
                            {
                                controls.push((c.to_string(), (i, j)));
                            }
                            if number.len() > 0 && number != "" {
                                numbers.push((number.clone(), (i, coords)));
                                number = "".to_string();
                                coords = vec![];
                            }
                        }
                        if j == (line.len() - 1) && number.len() > 0 && number != "" {
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

    println!("Controls: {:?}", content.0);
    println!("Numbers: {:?}", content.1);
    let mut sum = 0;

    /*'outer: for number in content.1 {
        'inner: for control in &content.0 {
            if number.1 .0 == control.1 .0
                || number.1 .0 == control.1 .0 + 1
                || number.1 .0 == control.1 .0 - 1
            {
                if number.1 .1.contains(&control.1 .1)
                    || number.1 .1.contains(&(&control.1 .1 - 1))
                    || number.1 .1.contains(&(&control.1 .1 + 1))
                {
                    println!("{:?} {:?}", number, control);
                    sum += number.0.parse::<i32>().unwrap();
                    continue 'outer;
                }
            }
        }
        println!(" sum {:?}", number);
    }*/

    let mut ctrl_num: HashMap<(String, (usize, usize)), Vec<(String, (usize, Vec<usize>))>> =
        HashMap::new();

    'outer: for number in content.1 {
        'inner: for control in &content.0 {
            if control.0 != '*'.to_string() {
                continue 'inner;
            }
            if number.1 .0 == control.1 .0
                || number.1 .0 == control.1 .0 + 1
                || number.1 .0 == control.1 .0 - 1
            {
                if number.1 .1.contains(&control.1 .1)
                    || number.1 .1.contains(&(&control.1 .1 - 1))
                    || number.1 .1.contains(&(&control.1 .1 + 1))
                {
                    println!("{:?} {:?}", number, control);
                    if (ctrl_num.contains_key(&control)) {
                        let mut number_vec = ctrl_num.get_mut(&control).unwrap();
                        number_vec.push(number.clone());
                    } else {
                        ctrl_num.insert(control.clone(), vec![number.clone()]);
                    }
                }
            }
        }
    }

    for (control, numbers) in ctrl_num {
        if numbers.len() == 2 {
            sum += numbers[0].0.parse::<i32>().unwrap() * numbers[1].0.parse::<i32>().unwrap();
            println!("{:?} {:?}", control, numbers);
        }
    }
    println!("sum: {}", sum);
}
