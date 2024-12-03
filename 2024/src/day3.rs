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

pub fn day3p1() {
    let file_contents = utils::read_file_to_u8("./inputs/3");

    let mul = "mul(";

    let mut i = 0;

    let mut total = 0;

    while i < file_contents.len() - mul.len() {
        if file_contents[i..i + mul.len()] != *mul.as_bytes() {
            i += 1;
            continue;
        }

        i += mul.len();

        // parse number here
        let num1 = parse_number(&file_contents, &mut i);

        if num1 == -1 || file_contents[i] != b',' {
            i += 1;
            continue;
        }

        i += 1;

        let num2 = parse_number(&file_contents, &mut i);

        if num2 == -1 || file_contents[i] != b')' {
            i += 1;
            continue;
        }

        i += 1;

        total += num1 * num2;
    }

    println!("Day3 P1 total: {}", total);
}

pub fn day3p2() {
    let file_contents = utils::read_file_to_u8("./inputs/3");

    let mul = "mul(";
    let doo = "do()";
    let dont = "don't()";

    let mut mul_enabled = true;

    let mut i = 0;
    let mut total = 0;

    while i < file_contents.len() - mul.len() {
        let is_eq_to_mul = file_contents[i..i + mul.len()] == *mul.as_bytes();

        if !is_eq_to_mul {
            if file_contents[i..i + doo.len()] == *doo.as_bytes() {
                mul_enabled = true;
                i += doo.len();
                continue;
            }

            if i + dont.len() < file_contents.len()
                && file_contents[i..i + dont.len()] == *dont.as_bytes()
            {
                mul_enabled = false;
                i += dont.len();
                continue;
            }

            i += 1;
            continue;
        }

        i += mul.len();

        // parse number here
        let num1 = parse_number(&file_contents, &mut i);

        if num1 == -1 || file_contents[i] != b',' {
            i += 1;
            continue;
        }

        i += 1;

        let num2 = parse_number(&file_contents, &mut i);

        if num2 == -1 || file_contents[i] != b')' {
            i += 1;
            continue;
        }

        i += 1;

        if mul_enabled {
            total += num1 * num2;
        }
    }

    println!("Day3 P2 total: {}", total);
}
