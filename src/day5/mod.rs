const INPUT: &str = include_str!("./input");

fn run(input: &mut [isize]) -> Vec<isize> {
    match input.split_at(2) {
        (&[3, save_pos], _) => {
            input[save_pos as usize] = 1;
            run_instructions(input, 2)
        }
        _ => panic!("No input entry"),
    }
}

fn run_instructions(input: &mut [isize], start: usize) -> Vec<isize> {
    let mut ans = vec![];
    let mut pointer = start;
    loop {
        match input[pointer] {
            99 => break,
            instruction_head => {
                let opcode = instruction_head % 100;
                let first_param_mod = instruction_head / 100 % 10;
                let second_param_mod = instruction_head / 1000 % 10;
                let get_value = |pointer: usize, param_mod| match param_mod {
                    0 => input[input[pointer] as usize],
                    1 => input[pointer],
                    _ => panic!("Invalid parameter at {} with mode {}", pointer, param_mod),
                };
                match opcode {
                    1 => {
                        // Parameters that an instruction writes to will never be in immediate mode.
                        input[get_value(pointer + 3, 1) as usize] =
                            get_value(pointer + 1, first_param_mod)
                                + get_value(pointer + 2, second_param_mod);
                        pointer += 4;
                    }
                    2 => {
                        input[get_value(pointer + 3, 1) as usize] =
                            get_value(pointer + 1, first_param_mod)
                                * get_value(pointer + 2, second_param_mod);
                        pointer += 4;
                    }
                    4 => {
                        ans.push(get_value(pointer + 1, first_param_mod));
                        pointer += 2;
                    }
                    _ => panic!(
                        "Invalid opcode {}, instruction_head = {}",
                        opcode, instruction_head
                    ),
                };
            }
        }
    }
    ans
}

pub fn part_1() -> Option<isize> {
    let mut instructions: Vec<isize> = INPUT
        .split(',')
        .filter_map(|v| v.parse::<isize>().ok())
        .collect();
    let outputs = run(&mut instructions);

    outputs.last().copied()
}

#[test]
fn test_run_instructions() {
    let input = &mut [1002, 4, 3, 4, 33];
    assert_eq!(run_instructions(input, 0), vec![]);
    assert_eq!(input.to_vec(), vec![1002, 4, 3, 4, 99]);
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(), Some(9_775_037));
}
