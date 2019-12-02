use std::fs;

// Part 1
fn _fuel(mass: usize) -> usize {
    mass / 3 - 2
}

// Part 2
fn fuel(mass: usize) -> usize {
    match (mass / 3).overflowing_sub(2) {
        (_, true) => 0,
        (v, false) => v + fuel(v),
    }
}

pub fn answer() -> usize {
    fs::read_to_string("src/day1/input")
        .unwrap()
        .lines()
        .filter_map(|line| line.parse::<usize>().ok())
        .map(fuel)
        .sum()
}

#[test]
fn calc_fuel_part_1() {
    assert_eq!(_fuel(12), 2);
    assert_eq!(_fuel(14), 2);
    assert_eq!(_fuel(1969), 654);
    assert_eq!(_fuel(100756), 33583);
}

#[test]
fn calc_fuel_part_2() {
    assert_eq!(fuel(12), 2);
    assert_eq!(fuel(14), 2);
    assert_eq!(fuel(1969), 966);
    assert_eq!(fuel(100756), 50346);
}
