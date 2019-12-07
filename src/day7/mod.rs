mod permutations;

use super::day5::run;
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

pub fn part_1() -> isize {
    let program = process_input(INPUT);
    permutations(5)
        .map(|settings| signal(program.clone(), settings.as_slice()))
        .max()
        .unwrap()
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(), 880_726);
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
