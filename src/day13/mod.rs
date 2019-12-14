use crate::intcode::{intcode_parser, Action, Machine};
use std::collections::HashSet;

const INPUT: &str = include_str!("./input");

pub fn part_1() -> usize {
    let program = intcode_parser(INPUT.trim());
    let mut machine = Machine::with_capacity(&program, 3000);
    let mut blocks = HashSet::new();

    while let (Action::Output(x), Action::Output(y), Action::Output(tile)) =
        (machine.run(), machine.run(), machine.run())
    {
        if tile == 2 {
            blocks.insert((x, y));
        }
    }

    blocks.len()
}
