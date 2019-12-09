mod permutations;

use super::day5::run;
use crate::intcode::{intcode_parser, Action, IntcodeProgram, Machine};
use permutations::permutations;

const INPUT: &str = include_str!("./input");

fn signal(program: Vec<isize>, phase_settings: &[usize]) -> isize {
    phase_settings.iter().fold(0, |input, setting| {
        *run(&mut program.clone(), &[*setting as isize, input])
            .last()
            .unwrap()
    })
}

fn process_input(input: &str) -> Vec<isize> {
    input
        .trim()
        .split(',')
        .map(|v| v.parse::<isize>().expect("Number"))
        .collect()
}

fn signal_loop_mode(program: &IntcodeProgram, phase_settings: &[isize]) -> isize {
    let mut output_a = Machine::new(&program);
    let mut output_b = Machine::new(&program);
    let mut output_c = Machine::new(&program);
    let mut output_d = Machine::new(&program);
    let mut output_e = Machine::new(&program);

    output_a.push_input(phase_settings[0]);
    output_b.push_input(phase_settings[1]);
    output_c.push_input(phase_settings[2]);
    output_d.push_input(phase_settings[3]);
    output_e.push_input(phase_settings[4]);

    output_a.push_input(0);

    let mut ret = None;

    loop {
        match output_a.run() {
            Action::Output(v) => output_b.push_input(v),
            Action::Halt => break,
        }
        match output_b.run() {
            Action::Output(v) => output_c.push_input(v),
            Action::Halt => break,
        }
        match output_c.run() {
            Action::Output(v) => output_d.push_input(v),
            Action::Halt => break,
        }
        match output_d.run() {
            Action::Output(v) => output_e.push_input(v),
            Action::Halt => break,
        }
        match output_e.run() {
            Action::Output(v) => {
                output_a.push_input(v);
                ret = Some(v);
            }
            Action::Halt => break,
        }
    }
    ret.expect("No output")
}

pub fn part_1() -> isize {
    let program = process_input(INPUT);
    permutations(5)
        .map(|settings| signal(program.clone(), settings.as_slice()))
        .max()
        .unwrap()
}

pub fn part_2() -> isize {
    let program = intcode_parser(INPUT.trim());
    permutations(5)
        .map(|settings| {
            settings
                .iter()
                .map(|v| (v + 5) as isize)
                .collect::<Vec<_>>()
        })
        .map(|v| signal_loop_mode(&program, &v))
        .max()
        .unwrap()
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(), 880_726);
}

#[test]
fn test_signal_loop() {
    assert_eq!(
        signal_loop_mode(
            &vec![
                3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28,
                -1, 28, 1005, 28, 6, 99, 0, 0, 5
            ],
            &[9, 8, 7, 6, 5]
        ),
        139_629_729
    );
    assert_eq!(
        signal_loop_mode(
            &vec![
                3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001,
                54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53,
                55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
            ],
            &[9, 7, 8, 5, 6]
        ),
        18216
    );
}

#[test]
fn test_signal() {
    assert_eq!(
        signal(
            vec![3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0],
            &[4, 3, 2, 1, 0]
        ),
        43210
    );
    assert_eq!(
        signal(
            vec![
                3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                23, 99, 0, 0
            ],
            &[0, 1, 2, 3, 4]
        ),
        54321
    );
    assert_eq!(
        signal(
            vec![
                3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33,
                1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
            ],
            &[1, 0, 4, 3, 2]
        ),
        65210
    );
}
