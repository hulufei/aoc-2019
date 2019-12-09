use crate::intcode::{intcode_parser, Action, Machine};

const INPUT: &str = include_str!("./input");

pub fn part_1() -> isize {
    let program = intcode_parser(INPUT.trim());
    let mut machine = Machine::with_capacity(&program, 2000);
    machine.push_input(1);
    match machine.run() {
        Action::Output(v) => v,
        Action::Halt => panic!("No output"),
    }
}

pub fn part_2() -> isize {
    let program = intcode_parser(INPUT.trim());
    let mut machine = Machine::with_capacity(&program, 2000);
    machine.push_input(2);
    match machine.run() {
        Action::Output(v) => v,
        Action::Halt => panic!("No output"),
    }
}
