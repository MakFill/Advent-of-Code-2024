use regex::Regex;

struct Register {
    a: u32,
    b: u32,
    c: u32,
}

fn get_combo_operand(operand: u32, registers: &Register) -> u32 {
    match operand {
        0..=3 => operand,
        4 => registers.a,
        5 => registers.b,
        6 => registers.c,
        _ => panic!("Wrong operand {operand}"),
    }
}

pub fn part_1() -> String {
    let mut registers = Register { a: 0, b: 0, c: 0 };
    let mut programm = vec![];
    let re = Regex::new(r"\d+").unwrap();
    include_str!("./input.txt")
        .lines()
        .collect::<Vec<_>>()
        .split(|&a| a.is_empty())
        .enumerate()
        .for_each(|(index, slice)| {
            let res = slice
                .iter()
                .flat_map(|line| {
                    re.find_iter(line)
                        .map(|i| i.as_str().parse::<u32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            if index == 0 {
                registers.a = res[0];
                registers.b = res[1];
                registers.c = res[2];
            } else {
                programm = res;
            }
        });

    let programm_last_index = programm.len() - 1;

    let mut res_vec = vec![];

    let mut instr_pointer = 0;
    while instr_pointer <= programm_last_index {
        let opcode = programm[instr_pointer];
        let operand = programm[instr_pointer + 1];

        match opcode {
            0 | 6 | 7 => {
                let operand = get_combo_operand(operand, &registers);
                let denominator = 2_u32.pow(operand);
                let res = registers.a / denominator;
                match opcode {
                    0 => registers.a = res,
                    6 => registers.b = res,
                    _ => registers.c = res,
                };
            }
            1 => {
                registers.b ^= operand;
            }
            2 => {
                let operand = get_combo_operand(operand, &registers);
                registers.b = operand % 8;
            }
            3 => {
                if registers.a != 0 {
                    instr_pointer = operand as usize;
                    continue;
                }
            }
            4 => {
                registers.b ^= registers.c;
            }
            5 => {
                let operand = get_combo_operand(operand, &registers);
                res_vec.push(operand % 8);
            }
            _ => panic!("Wrong opcode {opcode}"),
        };

        instr_pointer += 2;
    }

    res_vec
        .into_iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

pub fn part_2() -> usize {
    0
}
