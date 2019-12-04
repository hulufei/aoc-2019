use std::collections::HashMap;
use std::ops::AddAssign;
use Direction::*;

const INPUT: &str = include_str!("./input");

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl AddAssign<&Direction> for (i32, i32) {
    fn add_assign(&mut self, other: &Direction) {
        let (x, y) = self;
        match other {
            Up => *y += 1,
            Down => *y -= 1,
            Right => *x += 1,
            Left => *x -= 1,
        }
    }
}

fn parse_instruction(instruction: &str) -> (Direction, i32) {
    let (direction, distance) = instruction.split_at(1);
    match (direction, distance.parse()) {
        ("R", Ok(v)) => (Right, v),
        ("L", Ok(v)) => (Left, v),
        ("U", Ok(v)) => (Up, v),
        ("D", Ok(v)) => (Down, v),
        _ => panic!("Invalid instruction {}", instruction),
    }
}

fn parse_wire_coords(input: &str) -> HashMap<(i32, i32), i32> {
    let instructions = input.split(',').map(parse_instruction);
    let mut coord = (0, 0);
    let mut steps = 0;
    let mut coord_map = HashMap::new();

    for (direction, distance) in instructions {
        for _ in 0..distance {
            coord += &direction;
            steps += 1;
            // Insert only once
            coord_map.entry(coord).or_insert(steps);
        }
    }

    coord_map
}

pub fn part_1() -> Option<i32> {
    let mut lines = INPUT.lines();
    let wire1 = parse_wire_coords(lines.next().expect("Failed to read puzzle input."));
    let wire2 = parse_wire_coords(lines.next().expect("Failed to read puzzle input."));

    wire2
        .keys()
        .filter(|coord| wire1.contains_key(coord))
        .map(|(x, y)| x.abs() + y.abs())
        .min()
}

pub fn part_2() -> Option<i32> {
    let mut lines = INPUT.lines();
    let wire1 = parse_wire_coords(lines.next().expect("Failed to read puzzle input."));
    let wire2 = parse_wire_coords(lines.next().expect("Failed to read puzzle input."));

    wire2
        .keys()
        .filter(|coord| wire1.contains_key(coord))
        .map(|k| wire1[k] + wire2[k])
        .min()
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(), Some(232));
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(), Some(6084));
}
