use std::u64;

use crate::utils;

#[derive(Debug)]
struct Value {
    result: u64,
    operands: Vec<u64>,
}

fn can_get_result(target: u64, current_result: u64, operands: &Vec<u64>, i: usize) -> bool {
    if current_result == target && i == operands.len() {
        return true;
    }

    if i >= operands.len() {
        return false;
    }

    // check adding plus
    let mut plus = false;
    let mut multiply = false;

    if current_result + operands[i] <= target {
        plus = can_get_result(target, current_result + operands[i], operands, i + 1);
    }

    if current_result * operands[i] <= target {
        multiply = can_get_result(target, current_result * operands[i], operands, i + 1);
    }

    return plus || multiply;
}

fn concat(a: u64, b: u64) -> u64 {
    return (a.to_string() + &b.to_string()).parse::<u64>().unwrap();
}

fn can_get_result_with_concatenation(
    target: u64,
    current_result: u64,
    operands: &Vec<u64>,
    i: usize,
) -> bool {
    if current_result == target && i == operands.len() {
        return true;
    }

    if current_result > target || i >= operands.len() {
        return false;
    }

    // concat
    if can_get_result_with_concatenation(
        target,
        concat(current_result, operands[i]),
        operands,
        i + 1,
    ) {
        return true;
    }

    // add
    if can_get_result_with_concatenation(target, current_result + operands[i], operands, i + 1) {
        return true;
    }

    // multiply
    if can_get_result_with_concatenation(target, current_result * operands[i], operands, i + 1) {
        return true;
    }

    return false;
}

fn get_values() -> Vec<Value> {
    let file_contents = utils::read_file_to_string("./inputs/7");

    let mut values: Vec<Value> = vec![];

    for line in file_contents.lines() {
        let mut split = line.split(":");

        values.push(Value {
            result: split.next().unwrap().parse::<u64>().unwrap(),
            operands: split
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect(),
        });
    }

    return values;
}

pub fn day7p1() {
    let values = get_values();

    let mut total = 0;

    for v in &values {
        let can_do = can_get_result(v.result, v.operands[0], &v.operands, 1);

        if can_do {
            total += v.result;
        }
    }

    println!("Day7 P1: {total}");
}

// Day7 P2: 45071967642319 -> too high
pub fn day7p2() {
    let values = get_values();

    let mut total = 0;

    for v in &values {
        let can_do = can_get_result_with_concatenation(v.result, v.operands[0], &v.operands, 1);

        if can_do {
            total += v.result;
        }
    }

    println!("Day7 P2: {total}");
}
