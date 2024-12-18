use crate::utils;

enum Instruction {
    ADV, // division of value in A register by 2^operand. Stores in reg A
    BXL, // Bitwise XOR register B and literal OP. Stores in B
    BST, // combo operand modulo 8 and stores in B
    JNZ, // does nothing if A register is 0, else jumps by the operand
    BXC, // Bitwise XOR of register B and register C and stores in register B. Ignores the operand
    OUT, // combo operand modulo 8 and outputs the value
    BDV, // Exactly like ADV, but stores result in B
    CDV, // Exactly like ADV, but stores result in C
}

impl Instruction {
    fn from_u8(value: u8) -> Instruction {
        match value {
            0 => Instruction::ADV,
            1 => Instruction::BXL,
            2 => Instruction::BST,
            3 => Instruction::JNZ,
            4 => Instruction::BXC,
            5 => Instruction::OUT,
            6 => Instruction::BDV,
            7 => Instruction::CDV,
            _ => panic!("bleh"),
        }
    }

    fn handle_operand(
        &self,
        i: &mut usize,
        regs: &mut Registers,
        v: &Vec<u8>,
        result: &mut Vec<u64>,
    ) {
        match self {
            Instruction::ADV => {
                let operand = Operands::from_u8(v[*i + 1], regs);
                regs.a = regs.a / 2_u64.pow(operand as u32);

                *i += 2;
            }

            Instruction::BXL => {
                regs.b = regs.b ^ v[*i + 1] as u64;
                *i += 2;
            }

            Instruction::BST => {
                let operand = Operands::from_u8(v[*i + 1], regs);
                regs.b = operand % 8;

                *i += 2;
            }

            Instruction::JNZ => {
                let operand = v[*i + 1];

                if regs.a == 0 {
                    *i += 2;
                } else {
                    *i = operand as usize;
                }
            }

            Instruction::BXC => {
                regs.b = regs.b ^ regs.c;
                *i += 2;
            }

            Instruction::OUT => {
                let operand = Operands::from_u8(v[*i + 1], regs);

                result.push(operand % 8);

                *i += 2;
            }

            Instruction::BDV => {
                let operand = Operands::from_u8(v[*i + 1], regs);
                regs.b = regs.a / 2_u64.pow(operand as u32);

                *i += 2;
            }

            Instruction::CDV => {
                let operand = Operands::from_u8(v[*i + 1], regs);
                regs.c = regs.a / 2_u64.pow(operand as u32);

                *i += 2;
            }
        }
    }
}

enum Operands {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

#[derive(Default, Debug)]
struct Registers {
    a: u64,
    b: u64,
    c: u64,
}

impl Operands {
    fn from_u8(val: u8, r: &Registers) -> u64 {
        match val {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => r.a,
            5 => r.b,
            6 => r.c,
            v => v.into(),
        }
    }
}

fn parse_input() -> (Registers, Vec<u8>) {
    let file = utils::read_file_to_string("./inputs/17");

    let mut r = Registers::default();

    let mut file_split = file.split("\n\n");

    for (idx, line) in file_split.next().unwrap().lines().enumerate() {
        if line.len() == 0 {
            continue;
        }

        let value = line.split(":").nth(1).unwrap().trim().parse().unwrap();

        if idx == 0 {
            r.a = value;
        } else if idx == 1 {
            r.b = value;
        } else {
            r.c = value;
        }
    }

    return (
        r,
        file_split
            .next()
            .unwrap()
            .split(":")
            .nth(1)
            .unwrap()
            .split(",")
            .map(|x| x.trim().parse().unwrap())
            .collect(),
    );
}

pub fn day17p1() {
    let (mut registers, program) = parse_input();

    let mut i = 0;

    let mut result = vec![];

    println!("regs: {registers:?}, program: {program:?}");

    while i < program.len() {
        let inst = program[i];
        let inst = Instruction::from_u8(inst);

        let prev_i = i;

        inst.handle_operand(&mut i, &mut registers, &program, &mut result);

        // println!("registers: {registers:?}, i: {i}");

        if i == prev_i {
            println!("VM is stuck");
            break;
        }
    }

    println!("registers: {registers:?}");

    println!(
        "{}",
        result
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<String>>()
            .join(",")
    );
}
