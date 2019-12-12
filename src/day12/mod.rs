use std::cmp::Ordering;
use std::iter::repeat;
use std::collections::HashSet;

const INPUT: &str = include_str!("./input");

type Position = (i32, i32, i32);
type Velocity = (i32, i32, i32);

fn gen_velocity(moons: &[Position]) -> Vec<Velocity> {
    let calc_gravity = |a: i32, b| match a.cmp(b) {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1,
    };
    let len = moons.len();
    let ret = repeat(0).take(len);
    let g = vec![0];
    for i in 0..len {
        let a = moons[i];
        for j in i+1..len {
            let b = moons[j];
            let gravity = calc_gravity(a, b);
            g.push(gravity);
            *ret.get_mut(i) += gravity;
        }
        for r in 0..i {
            *ret.get_mut(i) += 
        }
    }
    moons
        .iter()
        .map(|(x, y, z)| {
            moons
                .iter()
                .map(|(x_other, y_other, z_other)| {
                    (
                        calc_gravity(*x, x_other),
                        calc_gravity(*y, y_other),
                        calc_gravity(*z, z_other),
                    )
                })
                .fold((0, 0, 0), |(x, y, z), (gx, gy, gz)| {
                    (x + gx, y + gy, z + gz)
                })
        })
        .collect()
}

fn apply_velocity(p: &mut [Position], v: &[Velocity]) {
    for (p, v) in p.iter_mut().zip(v) {
        p.0 += v.0;
        p.1 += v.1;
        p.2 += v.2;
    }
}

fn energy_after_steps(moons: &[Position], steps: usize) -> i32 {
    let mut velocity: Vec<Velocity> = repeat((0, 0, 0)).take(moons.len()).collect();
    let mut positions: Vec<Position> = moons.to_vec();
    for _ in 0..steps {
        apply_velocity(&mut velocity, &gen_velocity(&positions));
        apply_velocity(&mut positions, &velocity);
    }
    positions
        .iter()
        .zip(velocity)
        .map(|(p, v)| (p.0.abs() + p.1.abs() + p.2.abs()) * (v.0.abs() + v.1.abs() + v.2.abs()))
        .sum()
}

fn parse_input(input: &str) -> Vec<Position> {
    input
        .lines()
        .map(|line| {
            let position = line
                .chars()
                .filter(|c| *c == '-' || *c == ',' || c.is_digit(10))
                .collect::<String>()
                .split(',')
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            (position[0], position[1], position[2])
        })
        .collect()
}

fn reproduce_steps(input: &str) -> usize {
    let moons = &parse_input(input);
    let mut velocity: Vec<Velocity> = repeat((0, 0, 0)).take(moons.len()).collect();
    let mut positions: Vec<Position> = moons.to_vec();
    let mut history: HashSet<(Vec<Position>, Vec<Velocity>)> = HashSet::new();
    let mut steps = 0;

    while history.insert((positions.clone(), velocity.clone())) {
        apply_velocity(&mut velocity, &gen_velocity(&positions));
        apply_velocity(&mut positions, &velocity);
        steps += 1;
    }

    steps
}

pub fn part_1() -> i32 {
    energy_after_steps(&parse_input(INPUT), 1000)
}

pub fn part_2() -> usize {
    reproduce_steps(INPUT)
}

#[test]
fn test_gen_velocity() {
    let moons = &[(-1, 0, 2), (2, -10, -7), (4, -8, 8), (3, 5, -1)];
    assert_eq!(
        gen_velocity(moons),
        vec![(3, -1, -1), (1, 3, 3), (-3, 1, -3), (-1, -3, 1)]
    );
}

#[test]
fn test_energy_after_steps() {
    let moons = &[(-1, 0, 2), (2, -10, -7), (4, -8, 8), (3, 5, -1)];
    assert_eq!(energy_after_steps(moons, 10), 179);
}

#[test]
fn test_parse_input() {
    assert_eq!(
        parse_input(INPUT),
        vec![(1, 4, 4), (-4, -1, 19), (-15, -14, 12), (-17, 1, 10)]
    );
}

#[test]
fn test_reproduce() {
    let mut input = r"<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";
    assert_eq!(reproduce_steps(input), 2772);

    input = r"<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>";
    assert_eq!(reproduce_steps(input), 4_686_774_924);
}
