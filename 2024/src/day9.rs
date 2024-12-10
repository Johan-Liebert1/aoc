use std::fmt::Debug;

use crate::utils;

#[derive(PartialEq, Clone, Copy)]
enum InputType {
    Num(u32),
    FreeSpace,
}

impl Debug for InputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputType::Num(n) => write!(f, "{}", n),
            InputType::FreeSpace => write!(f, "."),
        }
    }
}

fn print_vec(v: &Vec<InputType>) {
    for i in v {
        print!("{i:?}");
    }

    println!();
}

fn transform_input() -> (Vec<InputType>, usize, usize) {
    let file_contents = utils::read_file_to_string("./inputs/9");

    let mut input_vec: Vec<InputType> = vec![];

    let mut iterator = file_contents.chars();

    let mut block_num = 0;

    loop {
        match iterator.next() {
            Some(ch) => {
                for _ in 0..ch.to_digit(10).unwrap() {
                    input_vec.push(InputType::Num(block_num))
                }

                block_num += 1;
            }
            None => break,
        }

        match iterator.next() {
            Some(ch) => {
                if ch.is_whitespace() {
                    break;
                }

                for _ in 0..ch.to_digit(10).unwrap() {
                    input_vec.push(InputType::FreeSpace);
                }
            }

            None => break,
        }
    }

    let first_free_space = input_vec
        .iter()
        .enumerate()
        .find(|x| *(x.1) == InputType::FreeSpace)
        .unwrap()
        .0;

    let last_num = input_vec
        .iter()
        .enumerate()
        .rev()
        .find(|x| matches!(x.1, InputType::Num(..)))
        .unwrap()
        .0;

    return (input_vec, first_free_space, last_num);
}

pub fn day9p1() {
    let (mut input_vec, mut first_free_space, mut last_num) = transform_input();

    while first_free_space < last_num {
        input_vec.swap(first_free_space, last_num);

        first_free_space += 1;

        while !matches!(input_vec[first_free_space], InputType::FreeSpace) {
            first_free_space += 1;
        }

        last_num -= 1;

        while last_num > 0 && matches!(input_vec[last_num], InputType::FreeSpace) {
            last_num -= 1;
        }
    }

    let mut total = 0;

    for (i, v) in input_vec.iter().enumerate() {
        if let InputType::Num(num) = v {
            total += i * (*num) as usize;
        }
    }

    println!("Day9 P1: {total}");
}

// 6353562575505 -> too low
// 6353646580604 -> incorrect
// 6353734428112 -> too high
// 6353736246204 -> too high
pub fn day9p2() {
    let (mut input_vec, _, mut last_disk_fragment_idx) = transform_input();

    let mut last_file_type = if let InputType::Num(n) = input_vec[last_disk_fragment_idx] {
        n
    } else {
        panic!("Bruh")
    };

    while last_disk_fragment_idx > 1 {
        let mut disk_fragment_start = last_disk_fragment_idx;

        while disk_fragment_start > 0
            && matches!(input_vec[disk_fragment_start], InputType::Num(n) if n == last_file_type)
        {
            disk_fragment_start -= 1;
        }

        let number_of_files = last_disk_fragment_idx - disk_fragment_start;

        // get free space that's large enough to fit  number_of_files
        let mut free_space_start = 0;
        let mut free_space_end = free_space_start;

        while free_space_end <= disk_fragment_start {
            // println!("free_space_start: {free_space_start}");

            free_space_start = free_space_end;

            while free_space_end < input_vec.len()
                && matches!(input_vec[free_space_end], InputType::FreeSpace)
            {
                free_space_end += 1;
                // println!("free_space_end: {free_space_end}");
            }

            // found the first place where we can fit the files
            if free_space_end - free_space_start >= number_of_files {
                break;
            }

            // println!("free_space_end += 1;");
            free_space_end += 1;
        }

        let left = free_space_start;
        let right = last_disk_fragment_idx;

        // println!("free_space_start: {free_space_start}, free_space_end: {free_space_end}, disk_fragment_start: {disk_fragment_start}, last_disk_fragment_idx: {last_disk_fragment_idx}");

        let mut number_of_free_spaces = free_space_end - free_space_start;

        // ugly check
        if !matches!(input_vec[free_space_start], InputType::FreeSpace)
            || (!matches!(input_vec[free_space_start], InputType::FreeSpace)
                || !matches!(input_vec[free_space_start - 1], InputType::FreeSpace))
        {
            number_of_free_spaces = 0;
        }

        if input_vec.len() < 50 {
            print_vec(&input_vec);
        }

        if number_of_free_spaces >= number_of_files {
            for i in 0..number_of_files {
                input_vec.swap(left + i, right - i);
            }
        }

        last_disk_fragment_idx = disk_fragment_start;

        while last_disk_fragment_idx > 0
            && if let InputType::Num(n) = input_vec[last_disk_fragment_idx] {
                last_file_type == n
            } else {
                true
            }
        {
            last_disk_fragment_idx -= 1;
        }

        last_file_type = if let InputType::Num(n) = input_vec[last_disk_fragment_idx] {
            n
        } else {
            panic!("Bruh")
        };
    }

    let mut total = 0;

    for (i, v) in input_vec.iter().enumerate() {
        if let InputType::Num(num) = v {
            total += i * (*num) as usize;
        }
    }

    println!("Day9 P2: {total}");
}
