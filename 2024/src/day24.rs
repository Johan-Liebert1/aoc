use std::collections::HashMap;

use crate::utils;

#[derive(Default, Debug)]
enum Op {
    #[default]
    And,
    Or,
    Xor,
}

impl Op {
    fn from_string(str: String) -> Self {
        match str.as_str() {
            "AND" => Op::And,
            "OR" => Op::Or,
            "XOR" => Op::Xor,

            _ => panic!("Incorrect: '{str}'"),
        }
    }
}

#[derive(Default, Debug)]
struct Operations {
    left: String,
    right: String,
    output: String,
    operation: Op,
    evaluated: bool,
}

fn parse_input() -> (HashMap<String, u32>, Vec<Operations>) {
    let file = utils::read_file_to_string("./inputs/24");

    let mut split = file.split("\n\n");

    let mut initial_values: HashMap<String, u32> = HashMap::new();

    for line in split.next().unwrap().lines() {
        let mut line_split = line.split(":");

        initial_values.insert(
            line_split.next().unwrap().to_string(),
            line_split.next().unwrap().trim().parse().unwrap(),
        );
    }

    let mut ops: Vec<Operations> = vec![];

    for line in split.next().unwrap().lines() {
        let mut line_split = line.split("->");

        let mut op_split = line_split.next().unwrap().split_whitespace();

        ops.push(Operations {
            left: op_split.next().unwrap().to_string(),
            operation: Op::from_string(op_split.next().unwrap().trim().to_string()),
            output: line_split.next().unwrap().trim().to_string(),
            right: op_split.next().unwrap().to_string(),
            evaluated: false,
        });
    }

    return (initial_values, ops);
}

pub fn part1() {
    let (mut initial_values, mut ops) = parse_input();

    loop {
        for op in &mut ops {
            if op.evaluated {
                continue;
            }

            let left = initial_values.get(&op.left);
            let right = initial_values.get(&op.right);

            if left.is_none() || right.is_none() {
                continue;
            }

            op.evaluated = true;

            let result = match op.operation {
                Op::And => left.unwrap() & right.unwrap(),
                Op::Or => left.unwrap() | right.unwrap(),
                Op::Xor => left.unwrap() ^ right.unwrap(),
            };

            match initial_values.get_mut(&op.output) {
                Some(v) => *v = result,

                None => {
                    initial_values.insert(op.output.clone(), result);
                }
            }
        }

        if ops.iter().all(|x| x.evaluated) {
            break;
        }
    }

    let mut output = [0; 64];

    for (k, v) in initial_values {
        if k.starts_with("z") {
            let index = &k[1..].parse::<usize>().unwrap();
            output[*index] = v;
        }
    }

    output.reverse();

    let s = output
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join("");

    println!("Day24 P1: {s:?}");
}

pub fn part2() {
}
