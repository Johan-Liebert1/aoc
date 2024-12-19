use std::{collections::HashMap, u128, usize};

use crate::utils;

fn parse_input() -> (Vec<String>, Vec<String>) {
    let file_contents = utils::read_file_to_string("./inputs/19");

    let mut split = file_contents.split("\n\n");

    let patterns: Vec<String> = split
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.trim().to_string())
        .collect();

    let to_create: Vec<String> = split
        .next()
        .unwrap()
        .split("\n")
        .map(|x| x.trim().to_string())
        .filter(|x| x.len() != 0)
        .collect();

    return (patterns, to_create);
}

fn can_make_pattern(
    patterns: &Vec<String>,
    to_make: &String,
    cache: &mut HashMap<String, bool>,
) -> bool {
    if to_make.len() == 0 {
        return true;
    }

    match cache.get(to_make) {
        Some(val) => return *val,
        None => {}
    }

    let mut can_make = false;

    for p in patterns {
        if to_make.starts_with(p) {
            let actual_to_make = to_make[p.len()..].to_string();

            can_make = can_make || can_make_pattern(patterns, &actual_to_make, cache);

            cache.insert(actual_to_make, can_make);

            if can_make {
                return true;
            }
        }
    }

    return can_make;
}

fn num_ways_to_make_pattern(
    patterns: &Vec<String>,
    to_make: &String,
    cache: &mut HashMap<String, usize>,
) -> usize {
    if to_make.len() == 0 {
        return 1;
    }

    if let Some(val) = cache.get(to_make) {
        return *val;
    }

    let mut total = 0;

    for p in patterns {
        if to_make.starts_with(p) {
            let actual_to_make = to_make[p.len()..].to_string();

            total += num_ways_to_make_pattern(patterns, &actual_to_make, cache);
        }
    }

    cache.insert(to_make.to_string(), total);

    return total;
}

pub fn part1() {
    let (patterns, to_create) = parse_input();

    let mut how_many_can_we_make = 0;

    let mut cache: HashMap<String, bool> = HashMap::new();

    for to_make in to_create {
        if can_make_pattern(&patterns, &to_make, &mut cache) {
            how_many_can_we_make += 1;
        }
    }

    println!("Day19 P1: {how_many_can_we_make}");
}

pub fn part2() {
    let (patterns, to_create) = parse_input();

    let mut how_many_can_we_make: u128 = 0;

    let mut cache: HashMap<String, usize> = HashMap::new();

    for to_make in to_create {
        how_many_can_we_make += num_ways_to_make_pattern(&patterns, &to_make, &mut cache) as u128;
    }

    println!("Day19 P1: {how_many_can_we_make}");
}
