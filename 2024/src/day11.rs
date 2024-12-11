// If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
//
// If the stone is engraved with a number that has an even number of digits, it is replaced by two stones.
// The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone.
// (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
//
// If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 2024 is engraved on the new stone.

use std::{collections::HashMap, usize};

use crate::utils;

type Iterations = u8;
type MapKey = (String, Iterations);

fn iterate(string: String, iterations: Iterations, map: &mut HashMap<MapKey, usize>) -> usize {
    if iterations == 0 {
        return 1;
    }

    let mut total: usize = 0;

    match map.get(&(string.clone(), iterations)) {
        Some(val) => {
            total += val;
        }

        None => {
            match string.as_str() {
                "0" => {
                    total += iterate("1".into(), iterations - 1, map);
                }

                stone => {
                    if stone.len() % 2 == 0 {
                        let half = stone.len() / 2;

                        // split the stone
                        let s1 = stone[..half].to_string();
                        let s2 = stone[half..].parse::<u64>().unwrap().to_string();

                        total +=
                            iterate(s1, iterations - 1, map) + iterate(s2, iterations - 1, map);
                    } else {
                        let number = stone.parse::<u64>().unwrap();
                        let string = (number * 2024).to_string();

                        total += iterate(string, iterations - 1, map);
                    }
                }
            }
        }
    }

    map.insert((string, iterations), total);

    return total;
}

pub fn day11p1() {
    let file_contents = utils::read_file_to_string("./inputs/11");

    let mut input = file_contents
        .split_whitespace()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let mut new_vec: Vec<String> = vec![];

    let iterations = 25;

    for _ in 0..iterations {
        for i in input {
            match i.as_str() {
                "0" => {
                    new_vec.push("1".into());
                }

                stone => {
                    if stone.len() % 2 == 0 {
                        let half = stone.len() / 2;

                        // split the stone
                        new_vec.push(stone[..half].to_string());

                        new_vec.push(stone[half..].parse::<u64>().unwrap().to_string());
                    } else {
                        let number = stone.parse::<u64>().unwrap();

                        let string = (number * 2024).to_string();

                        new_vec.push(string);
                    }
                }
            }
        }

        input = new_vec;
        new_vec = vec![];
    }

    println!("Day 11 P1: {}", input.len());
}

pub fn day11p2() {
    let file_contents = utils::read_file_to_string("./inputs/11");

    let og_input = file_contents
        .split_whitespace()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let iterations = 75;

    let mut total = 0;

    let mut map: HashMap<MapKey, usize> = HashMap::new();

    for og_i in &og_input {
        match map.get(&(og_i.to_string(), iterations)) {
            Some(val) => {
                total += val;
            }

            None => {
                total += iterate(og_i.to_string(), iterations, &mut map);
            }
        }
    }

    println!("Day 11 P2: {}", total);
}
