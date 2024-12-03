use crate::utils;

pub fn day2p1() {
    let file_contents = utils::read_file_to_string("./inputs/2");

    let mut num_safe = 0;

    for (_, line) in file_contents.split("\n").enumerate() {
        if line.len() == 0 {
            continue;
        }

        let v: Vec<i32> = line.split(" ").map(|y| y.parse::<i32>().unwrap()).collect();

        let mut first = v[0];
        let mut second = v[1];

        if first == second || first.abs_diff(second) > 3 {
            continue;
        }

        let inc = first < second;

        first = second;

        let mut is_safe = true;

        'o: for i in 2..v.len() {
            second = v[i];

            if (inc && second <= first) || (!inc && second >= first) || first.abs_diff(second) > 3 {
                is_safe = false;
                break 'o;
            }

            first = second;
        }

        if is_safe {
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

        let mut removed = 0;
        let mut is_safe = true;

        let mut increasing = v[1] > v[0];

        for i in 1..v.len() {
            if removed > 1 {
                is_safe = false;
                break;
            }

            let second = v[i];
            let first = v[i - 1];

            if first == second || first.abs_diff(second) > 3 {
                removed += 1;
                continue;
            }

            if (increasing && second < first) || (!increasing && second > first) {
                removed += 1;

                if i > 1 && ((second > v[i - 2]) != increasing) {
                    is_safe = false;
                    break;
                }

                continue;
            }

            increasing = second > first;
        }

        if is_safe {
            num_safe += 1;
        }
    }

    println!("Part2: {}", num_safe);
}
