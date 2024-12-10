use crate::utils;

fn is_sorted(v: &Vec<i32>) -> bool {
    let mut first = v[0];
    let mut second = v[1];

    if first == second || first.abs_diff(second) > 3 {
        return false;
    }

    let inc = first < second;

    first = second;

    for i in 2..v.len() {
        second = v[i];

        if (inc && second <= first) || (!inc && second >= first) || first.abs_diff(second) > 3 {
            return false;
        }

        first = second;
    }

    return true;
}

pub fn day2p1() {
    let file_contents = utils::read_file_to_string("./inputs/2");

    let mut num_safe = 0;

    for (_, line) in file_contents.split("\n").enumerate() {
        if line.len() == 0 {
            continue;
        }

        let v: Vec<i32> = line.split(" ").map(|y| y.parse::<i32>().unwrap()).collect();

        if is_sorted(&v) {
            num_safe += 1;
        }
    }

    println!("Part1: {}", num_safe);
}

pub fn day2p2() {
    let file_contents = utils::read_file_to_string("./inputs/2");

    let mut num_safe = 0;

    for line in file_contents.lines() {
        if line.trim().len() == 0 {
            continue;
        }

        let v: Vec<i32> = line
            .split_whitespace()
            .map(|y| y.parse::<i32>().unwrap())
            .collect();

        let mut is_safe = false;

        for i in 0..v.len() {
            // remove i
            
            let left = &mut v[0..i].to_vec();
            left.extend(v[i + 1..].to_vec());

            if is_sorted(left) {
                is_safe = true;
                break;
            }
        }

        if is_safe {
            num_safe += 1;
        }
    }

    println!("Part2: {}", num_safe);
}
