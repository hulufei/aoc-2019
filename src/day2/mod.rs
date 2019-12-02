use std::fs;

fn run(codes: &mut [usize]) {
    for i in (0..codes.len()).step_by(4) {
        let op = codes[i];

        match op {
            1 => codes[codes[i + 3]] = codes[codes[i + 1]] + codes[codes[i + 2]],
            2 => codes[codes[i + 3]] = codes[codes[i + 1]] * codes[codes[i + 2]],
            99 => break,
            _ => panic!("Unknow opcode {}", op),
        }
    }
}

fn run_with(codes: &mut [usize], noun: usize, verb: usize) -> usize {
    codes[1] = noun;
    codes[2] = verb;
    run(codes);
    codes[0]
}

fn parse_input() -> Vec<usize> {
    fs::read_to_string("src/day2/input")
        .unwrap()
        .split(',')
        .filter_map(|v| v.parse().ok())
        .collect()
}

pub fn part_1() -> usize {
    let mut codes = parse_input();

    run_with(&mut codes, 12, 2)
}

pub fn part_2() -> usize {
    let codes = parse_input();
    let mut noun = 0;
    let mut verb = 0;

    for i in 0..=99 {
        for j in 0..=99 {
            if run_with(&mut codes.clone(), i, j) == 19690720 {
                noun = i;
                verb = j;
                break;
            }
        }
    }

    100 * noun + verb
}

#[test]
fn test_run() {
    let mut codes = vec![1, 0, 0, 0, 99];
    run(&mut codes);
    assert_eq!(codes, vec![2, 0, 0, 0, 99]);

    codes = vec![2, 3, 0, 3, 99];
    run(&mut codes);
    assert_eq!(codes, vec![2, 3, 0, 6, 99]);

    codes = vec![2, 4, 4, 5, 99, 0];
    run(&mut codes);
    assert_eq!(codes, vec![2, 4, 4, 5, 99, 9801]);

    codes = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
    run(&mut codes);
    assert_eq!(codes, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(), 4138658);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(), 7264);
}
