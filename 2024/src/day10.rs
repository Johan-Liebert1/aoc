use crate::utils;

type Input = Vec<Vec<u32>>;

fn parse_input() -> Input {
    let file_contents = utils::read_file_to_string("./inputs/10");
    let mut vector: Input = vec![];

    for line in file_contents.lines() {
        vector.push(line.chars().map(|x| x.to_digit(10).unwrap()).collect());
    }

    return vector;
}

fn check_score(
    vec: &Input,
    row: usize,
    col: usize,
    last_value: u32,
    visiting: &mut Input,
    find_distinct: bool,
) -> i32 {
    if row >= vec.len() || col >= vec[0].len() {
        return 0;
    }

    if vec[row][col] < last_value || vec[row][col] - last_value != 1 {
        return 0;
    }

    if vec[row][col] == 9 {
        let retval = if visiting[row][col] == 1 && find_distinct {
            // already been to this 9
            // don't recount
            0
        } else {
            visiting[row][col] = 1;
            1
        };

        return retval;
    }

    if visiting[row][col] == 1 {
        return 0;
    }

    visiting[row][col] = 1;

    let left = if col > 0 {
        check_score(vec, row, col - 1, vec[row][col], visiting, find_distinct)
    } else {
        0
    };

    let right = check_score(vec, row, col + 1, vec[row][col], visiting, find_distinct);

    let up = if row > 0 {
        check_score(vec, row - 1, col, vec[row][col], visiting, find_distinct)
    } else {
        0
    };

    let down = check_score(vec, row + 1, col, vec[row][col], visiting, find_distinct);

    // reset visited so that if we come from another direction we still get to use this number
    visiting[row][col] = 0;

    return left + right + up + down;
}

// 936 -> wrong
pub fn day10p1() {
    let vector = parse_input();

    let mut total = 0;

    let mut visiting = vec![vec![0; vector[0].len()]; vector.len()];

    for row in 0..vector.len() {
        for col in 0..vector[0].len() {
            if vector[row][col] == 0 {
                total += check_score(&vector, row, col + 1, 0, &mut visiting, true);
                total += check_score(&vector, row + 1, col, 0, &mut visiting, true);

                total += if row > 0 {
                    check_score(&vector, row - 1, col, 0, &mut visiting, true)
                } else {
                    0
                };

                total += if col > 0 {
                    check_score(&vector, row, col - 1, 0, &mut visiting, true)
                } else {
                    0
                };

                // reset visited for another 0
                visiting = vec![vec![0; vector[0].len()]; vector.len()];
            }
        }
    }

    println!("Day10 P1: {total}");
}

pub fn day10p2() {
    let vector = parse_input();

    let mut total = 0;

    let mut visiting = vec![vec![0; vector[0].len()]; vector.len()];

    for row in 0..vector.len() {
        for col in 0..vector[0].len() {
            if vector[row][col] == 0 {
                total += check_score(&vector, row, col + 1, 0, &mut visiting, false);
                total += check_score(&vector, row + 1, col, 0, &mut visiting, false);

                total += if row > 0 {
                    check_score(&vector, row - 1, col, 0, &mut visiting, false)
                } else {
                    0
                };

                total += if col > 0 {
                    check_score(&vector, row, col - 1, 0, &mut visiting, false)
                } else {
                    0
                };

                // reset visited for another 0
                visiting = vec![vec![0; vector[0].len()]; vector.len()];
            }
        }
    }

    println!("Day10 P2: {total}");
}
