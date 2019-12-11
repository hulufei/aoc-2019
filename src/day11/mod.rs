use crate::intcode::{intcode_parser, Action, Machine};
use std::collections::HashMap;
use Color::*;

const INPUT: &str = include_str!("./input");

type Coord = (i32, i32);

struct Grid {
    panels: HashMap<Coord, Color>,
    locate: (Coord, Direction),
    machine: Machine,
}

impl Grid {
    fn new(program: &str, start_color: Color) -> Self {
        let mut panels = HashMap::new();
        let start = (0, 0);
        let program = intcode_parser(program);
        let mut machine = Machine::with_capacity(&program, 2000);

        machine.push_input(start_color as isize);
        panels.insert(start, Black);

        Grid {
            panels,
            machine,
            locate: (start, Direction::Up),
        }
    }

    fn run(&mut self) {
        loop {
            // First output color
            match self.machine.run() {
                Action::Output(v) => {
                    self.panels.insert(self.locate.0, Color::from(v));
                }
                Action::Halt => break,
            }
            // Second output direction
            match self.machine.run() {
                Action::Output(v) => {
                    let (coord, direction) = &self.locate;
                    let (x, y) = *coord;
                    self.locate = match (Turn::from(v), direction) {
                        (Turn::Left, Direction::Up) | (Turn::Right, Direction::Down) => {
                            ((x - 1, y), Direction::Left)
                        }
                        (Turn::Left, Direction::Down) | (Turn::Right, Direction::Up) => {
                            ((x + 1, y), Direction::Right)
                        }
                        (Turn::Left, Direction::Left) | (Turn::Right, Direction::Right) => {
                            ((x, y - 1), Direction::Down)
                        }
                        (Turn::Left, Direction::Right) | (Turn::Right, Direction::Left) => {
                            ((x, y + 1), Direction::Up)
                        }
                    };
                    let next_panel_color = self.panels.get(&self.locate.0).unwrap_or(&Black);
                    self.machine.push_input(*next_panel_color as isize)
                }
                Action::Halt => break,
            }
        }
    }
}

#[derive(Copy, Clone)]
enum Color {
    White = 1,
    Black = 0,
}

impl From<isize> for Color {
    fn from(v: isize) -> Color {
        match v {
            1 => White,
            0 => Black,
            _ => unimplemented!(),
        }
    }
}

enum Turn {
    Left = 0,
    Right = 1,
}

impl From<isize> for Turn {
    fn from(v: isize) -> Turn {
        match v {
            1 => Turn::Right,
            0 => Turn::Left,
            _ => unimplemented!(),
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn part_1() -> usize {
    let mut grid = Grid::new(INPUT.trim(), Black);
    grid.run();
    grid.panels.keys().len()
}

pub fn part_2() {
    let mut grid = Grid::new(INPUT.trim(), White);
    grid.run();
    let coords = grid.panels.keys();
    let min_x = coords.clone().min_by_key(|k| k.0).unwrap().0;
    let max_x = coords.clone().max_by_key(|k| k.0).unwrap().0;
    let min_y = coords.clone().min_by_key(|k| k.1).unwrap().1;
    let max_y = coords.clone().max_by_key(|k| k.1).unwrap().1;
    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            let color = match grid.panels.get(&(x, y)) {
                Some(White) => "⬜️",
                _ => "⬛️",
            };
            print!("{}", color);
        }
        println!();
    }
}
