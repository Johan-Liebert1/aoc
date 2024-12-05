use std::{collections::HashMap, process::exit, usize};

use crate::utils;

fn parse_number(file_contents: &Vec<u8>, i: &mut usize) -> i32 {
    let mut string = String::new();

    while *i < file_contents.len() {
        if !file_contents[*i].is_ascii_digit() {
            break;
        }

        string += &(file_contents[*i] as char).to_string();

        *i += 1;
    }

    return match string.parse::<i32>() {
        Ok(v) => v,
        Err(_) => -1,
    };
}

pub fn day5p1() {
    let file_contents = utils::read_file_to_u8("./inputs/5");

    let (index, _) = file_contents
        .iter()
        .enumerate()
        .find(|(idx, char)| **char == b'\n' && file_contents[idx + 1] == b'\n')
        .unwrap();

    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();

    let mut i = 0;

    while i < index {
        let before = parse_number(&file_contents, &mut i);

        // consume "|"
        i += 1;

        let after = parse_number(&file_contents, &mut i);

        // consume "\n"
        i += 1;

        match map.get_mut(&after) {
            Some(v) => {
                v.push(before);
            }

            None => {
                map.insert(after, vec![before]);
            }
        }
    }

    // now we need to get all print instructions
    i = index + 2;

    let mut total = 0;

    while i < file_contents.len() {
        let mut instructions = vec![];

        while i < file_contents.len() {
            let print_command = parse_number(&file_contents, &mut i);

            instructions.push(print_command);

            if i < file_contents.len() && file_contents[i] == b'\n' {
                break;
            }

            // consume ','
            i += 1;
        }

        i += 1;

        let mut valid = true;

        'outer: for (idx, inst) in instructions.iter().enumerate() {
            match map.get(&inst) {
                Some(all_before) => {
                    for j in idx..instructions.len() {
                        if all_before.contains(&instructions[j]) {
                            valid = false;
                            break 'outer;
                        }
                    }
                }

                None => {
                    // nothing can come before this
                    if idx != 0 {
                        valid = false;
                        break 'outer;
                    }
                }
            }
        }

        if valid {
            total += instructions[instructions.len() / 2];
        }
    }

    println!("Day5 P1: {total}");
}

pub fn day5p2() {
    let file_contents = utils::read_file_to_u8("./inputs/5");

    let (index, _) = file_contents
        .iter()
        .enumerate()
        .find(|(idx, char)| **char == b'\n' && file_contents[idx + 1] == b'\n')
        .unwrap();

    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();

    let mut i = 0;

    while i < index {
        let before = parse_number(&file_contents, &mut i);

        // consume "|"
        i += 1;

        let after = parse_number(&file_contents, &mut i);

        // consume "\n"
        i += 1;

        match map.get_mut(&after) {
            Some(v) => {
                v.push(before);
            }

            None => {
                map.insert(after, vec![before]);
            }
        }
    }

    // now we need to get all print instructions
    i = index + 2;

    let mut total = 0;

    while i < file_contents.len() {
        let mut instructions = vec![];

        while i < file_contents.len() {
            let print_command = parse_number(&file_contents, &mut i);

            instructions.push(print_command);

            if i < file_contents.len() && file_contents[i] == b'\n' {
                break;
            }

            // consume ','
            i += 1;
        }

        i += 1;

        let mut valid = true;

        'outer: for (idx, inst) in instructions.iter().enumerate() {
            match map.get(inst) {
                Some(all_before) => {
                    for j in idx..instructions.len() {
                        if all_before.contains(&instructions[j]) {
                            valid = false;
                            break 'outer;
                        }
                    }
                }

                None => {
                    // nothing can come before this
                    if idx != 0 {
                        valid = false;
                        break 'outer;
                    }
                }
            }
        }

        if !valid {
            // arrange this in a way that this becomes correctly updated

            let mut valid_ordering: Vec<i32> = vec![-1; instructions.len()];

            for inst in &instructions {
                // get the current instruction in the map and check how many that
                // should appear before it appear in this array. The final number
                // will be the position of this instruction in the new vector

                let mut position = 0;

                match map.get(inst) {
                    Some(all_before) => {
                        for before in all_before {
                            if instructions.contains(before) {
                                position += 1;
                            }
                        }
                    }

                    None => {
                        // nothing can come before it
                        position = 0;
                    }
                }

                if position > instructions.len() || (position == 0 && valid_ordering[0] != -1) {
                    eprintln!("Could not find correct position for instruction {inst}");
                    exit(1);
                }

                valid_ordering[position] = *inst;
            }

            total += valid_ordering[valid_ordering.len() / 2];
        }
    }

    println!("Day5 P1: {total}");
}
