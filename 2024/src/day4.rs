use crate::utils;

#[inline]
fn get_index(i: usize, j: usize, width: usize) -> usize {
    i * width + j
}

fn parse_file() -> (Vec<u8>, usize, usize) {
    let file_contents = utils::read_file_to_u8("./inputs/4");

    let (mut width, _) = file_contents
        .iter()
        .enumerate()
        .find(|(_, x)| **x == b'\n')
        .unwrap();

    // we do +1 to compensate for the fact that there will be "width" number of "\n"
    width += 1;

    let height = file_contents.len() / width;

    (file_contents, width, height)
}

pub fn day4p1() {
    let (file_contents, width, height) = parse_file();

    let xmas = "XMAS";
    let samx = "SAMX";

    let mut total = 0;

    for row in 0..height {
        for col in 0..width - 1 {
            let idx = get_index(row, col, width);

            // check to the right
            if (col + xmas.len()) <= width {
                let word = &file_contents[idx..idx + xmas.len()];

                if word == xmas.as_bytes() || word == samx.as_bytes() {
                    total += 1;
                }
            }

            // check down
            if (row + xmas.len()) <= height {
                let mut word = vec![];
                let mut r = row;

                for _ in 0..xmas.len() {
                    word.push(file_contents[get_index(r, col, width)]);
                    r += 1;
                }

                if word == xmas.as_bytes() || word == samx.as_bytes() {
                    total += 1;
                }
            }

            // right diaognal
            if (col + xmas.len()) < width && (row + xmas.len()) <= height {
                let mut word = vec![];
                let mut r = row;
                let mut c = col;

                for _ in 0..xmas.len() {
                    word.push(file_contents[get_index(r, c, width)]);
                    r += 1;
                    c += 1;
                }

                if word == xmas.as_bytes() || word == samx.as_bytes() {
                    total += 1;
                }
            }

            // left diaognal
            if col >= xmas.len() - 1 && (row + xmas.len()) <= height {
                let mut r = row;
                let mut c = col;
                let mut word = vec![file_contents[get_index(r, c, width)]];

                for _ in 0..3 {
                    r += 1;
                    c -= 1;
                    word.push(file_contents[get_index(r, c, width)]);
                }

                if word == xmas.as_bytes() || word == samx.as_bytes() {
                    total += 1;
                }
            }
        }
    }

    println!("Day4 P1: {total}");
}

pub fn day4p2() {
    let (file_contents, width, height) = parse_file();

    let mas = "MAS";
    let sam = "SAM";

    let mut total = 0;

    for row in 0..height {
        for col in 0..width - 1 {
            // right diaognal
            if (col + mas.len()) < width && (row + mas.len()) <= height {
                let mut right_diag_word = vec![];
                let mut r = row;
                let mut c = col;

                for _ in 0..mas.len() {
                    right_diag_word.push(file_contents[get_index(r, c, width)]);
                    r += 1;
                    c += 1;
                }

                r = row;
                c = col + 2;

                // left diagonal
                let mut left_diag_word = vec![file_contents[get_index(r, c, width)]];

                for _ in 0..2 {
                    r += 1;
                    c -= 1;
                    left_diag_word.push(file_contents[get_index(r, c, width)]);
                }

                if (right_diag_word == mas.as_bytes() || right_diag_word == sam.as_bytes())
                    && (left_diag_word == mas.as_bytes() || left_diag_word == sam.as_bytes())
                {
                    total += 1;
                }
            }
        }
    }

    println!("Day4 P2: {total}");
}
