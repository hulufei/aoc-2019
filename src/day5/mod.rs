// NOTE: include_str! will append new line in the end always
const INPUT: &str = include_str!("./input");

pub fn run(instructions: &mut [isize], inputs: &[isize]) -> Vec<isize> {
    let mut ans = vec![];
    let mut pointer = 0;
    let mut input_idx = 0;
    loop {
        let instruction_head = instructions[pointer];
        let opcode = instruction_head % 100;
        let first_param_mod = instruction_head / 100 % 10;
        let second_param_mod = instruction_head / 1000 % 10;
        let get_value = |pointer: usize, param_mod| match param_mod {
            0 => instructions[instructions[pointer] as usize],
            1 => instructions[pointer],
            _ => panic!("Invalid parameter at {} with mode {}", pointer, param_mod),
        };
        match opcode {
            99 => break,
            3 => {
                instructions[get_value(pointer + 1, 1) as usize] =
                    *inputs.get(input_idx).expect("Set input correctly");
                pointer += 2;
                input_idx += 1;
            }
            1 => {
                // Parameters that an instruction writes to will never be in immediate mode.
                instructions[get_value(pointer + 3, 1) as usize] =
                    get_value(pointer + 1, first_param_mod)
                        + get_value(pointer + 2, second_param_mod);
                pointer += 4;
            }
            2 => {
                instructions[get_value(pointer + 3, 1) as usize] =
                    get_value(pointer + 1, first_param_mod)
                        * get_value(pointer + 2, second_param_mod);
                pointer += 4;
            }
            4 => {
                ans.push(get_value(pointer + 1, first_param_mod));
                pointer += 2;
            }
            5 => {
                let first_param = get_value(pointer + 1, first_param_mod);
                let second_param = get_value(pointer + 2, second_param_mod);
                if first_param != 0 {
                    pointer = second_param as usize;
                } else {
                    pointer += 3;
                }
            }
            6 => {
                let first_param = get_value(pointer + 1, first_param_mod);
                let second_param = get_value(pointer + 2, second_param_mod);
                if first_param == 0 {
                    pointer = second_param as usize;
                } else {
                    pointer += 3;
                }
            }
            7 => {
                let first_param = get_value(pointer + 1, first_param_mod);
                let second_param = get_value(pointer + 2, second_param_mod);
                let write_pointer = get_value(pointer + 3, 1) as usize;
                instructions[write_pointer] = if first_param < second_param { 1 } else { 0 };
                pointer += 4;
            }
            8 => {
                let first_param = get_value(pointer + 1, first_param_mod);
                let second_param = get_value(pointer + 2, second_param_mod);
                let write_pointer = get_value(pointer + 3, 1) as usize;
                instructions[write_pointer] = if first_param == second_param { 1 } else { 0 };
                pointer += 4;
            }
            _ => panic!(
                "Invalid opcode {}, instruction_head = {}",
                opcode, instruction_head
            ),
        };
    }
    ans
}

pub fn part_1() -> Option<isize> {
    let mut instructions: Vec<isize> = INPUT
        .split(',')
        .filter_map(|v| v.parse::<isize>().ok())
        .collect();
    let outputs = run(&mut instructions, &[1]);

    outputs.last().copied()
}

pub fn part_2() -> Option<isize> {
    let mut instructions: Vec<isize> = INPUT
        .trim()
        .split(',')
        // use unwrap to panic on error
        .map(|v| v.parse::<isize>().unwrap())
        .collect();
    let outputs = run(&mut instructions, &[5]);

    outputs.last().copied()
}

#[test]
fn test_run_without_input_value() {
    let input = &mut [1002, 4, 3, 4, 33];
    assert_eq!(run(input, &[]), vec![]);
    assert_eq!(input.to_vec(), vec![1002, 4, 3, 4, 99]);
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(), Some(9_775_037));
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(), Some(15586959));
}

#[test]
fn test_less_than_and_equal() {
    // Test if equal to 8, position mode
    assert_eq!(
        run(&mut [3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], &[8]),
        vec![1]
    );
    assert_eq!(
        run(&mut [3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], &[9]),
        vec![0]
    );

    // Test if less than 8, position mode
    assert_eq!(
        run(&mut [3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], &[7]),
        vec![1]
    );
    assert_eq!(
        run(&mut [3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], &[9]),
        vec![0]
    );

    // Test if equal to 8, immediate mode
    assert_eq!(run(&mut [3, 3, 1108, -1, 8, 3, 4, 3, 99], &[8]), vec![1]);
    assert_eq!(run(&mut [3, 3, 1108, -1, 8, 3, 4, 3, 99], &[9]), vec![0]);

    // Test if less than 8, immediate mode
    assert_eq!(run(&mut [3, 3, 1107, -1, 8, 3, 4, 3, 99], &[7]), vec![1]);
    assert_eq!(run(&mut [3, 3, 1107, -1, 8, 3, 4, 3, 99], &[9]), vec![0]);
}

#[test]
fn test_jumps() {
    assert_eq!(
        run(
            &mut [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            &[0]
        ),
        vec![0]
    );
    assert_eq!(
        run(
            &mut [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            &[2]
        ),
        vec![1]
    );
    assert_eq!(
        run(&mut [3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], &[0]),
        vec![0]
    );
    assert_eq!(
        run(&mut [3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], &[9]),
        vec![1]
    );
}

#[test]
fn test_larger_example() {
    let instructions = [
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
        1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
        1105, 1, 46, 98, 99,
    ];
    assert_eq!(run(&mut instructions.clone(), &[2]), vec![999]);
    assert_eq!(run(&mut instructions.clone(), &[8]), vec![1000]);
    assert_eq!(run(&mut instructions.clone(), &[9]), vec![1001]);
}
