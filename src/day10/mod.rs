use std::collections::HashMap;
use std::f32;

const INPUT: &str = include_str!("./input");

#[derive(Debug, PartialEq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

fn gcd(a: i32, b: i32) -> i32 {
    let a = a.abs();
    let b = b.abs();
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn angle(p1: Position, p2: Position) -> (i32, i32) {
    // Top-left coordinates
    let y = p1.y - p2.y;
    let x = p2.x - p1.x;
    let gcd = gcd(y, x);

    (x / gcd, y / gcd)
}

fn gen_angles(points: &[Position], p: Position) -> (Position, Vec<(i32, i32)>) {
    (
        Position { x: p.x, y: p.y },
        points
            .iter()
            .filter(|v| **v != p)
            .map(|v| angle(*v, p))
            .collect(),
    )
}

fn gen_clockwise_angles(points: &[Position], p: Position) -> HashMap<(i32, i32), Vec<Position>> {
    let mut ret = HashMap::new();
    for target in points.iter().filter(|v| **v != p) {
        let entry = ret.entry(angle(p, *target)).or_insert_with(|| vec![]);
        entry.push(*target);
    }
    ret
}

fn vaporize(input: &str, nth: u32) -> Option<Position> {
    let points = parse_position(input);
    match max_count_detects(&points) {
        Some((central, _)) => {
            // println!("Central {:?}", central);
            let mut clockwise_map = gen_clockwise_angles(&points, central);
            let mut angles: Vec<_> = clockwise_map
                .iter_mut()
                .map(|(angle, points)| {
                    points.sort_by_key(|Position { y, .. }| y.abs());
                    (angle, points)
                })
                .collect();
            let mut pick = 0;
            angles.sort_by(|((x1, y1), _), ((x2, y2), _)| {
                clockwise_degree(*x1 as f32, *y1 as f32)
                    .partial_cmp(&clockwise_degree(*x2 as f32, *y2 as f32))
                    .unwrap()
            });
            // println!("sorted angles: {:?}", angles);
            loop {
                for (_, points_in_line) in angles.iter_mut() {
                    if let Some(v) = points_in_line.pop() {
                        pick += 1;
                        if pick == nth {
                            return Some(v);
                        }
                    }
                }
            }
        }
        _ => panic!("No max central point"),
    }
}

pub fn part_2() -> i32 {
    vaporize(INPUT, 200)
        .map(|Position { x, y }| x * 100 + y)
        .unwrap()
}

fn clockwise_degree(x: f32, y: f32) -> f32 {
    let angle = y.atan2(x);
    let mut degree = angle.to_degrees();
    if degree > 0. && degree <= 180. {
        degree = 360. - degree;
    } else {
        degree = -degree;
    }
    degree += 90.;

    degree % 360.
}

fn count_detects(points: &[Position], p: Position) -> (Position, usize) {
    let (central, mut angles) = gen_angles(points, p);
    angles.sort();
    // println!("For p = {:?}, angles sorted: {:?}", p, angles);
    angles.dedup();
    (central, angles.len())
}

fn parse_position(input: &str) -> Vec<Position> {
    let mut ret = vec![];

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if let '#' = c {
                ret.push(Position {
                    x: x as i32,
                    y: y as i32,
                })
            }
        }
    }

    ret
}

fn max_count_detects(points: &[Position]) -> Option<(Position, usize)> {
    points
        .iter()
        .map(|p| count_detects(&points, *p))
        .max_by_key(|v| v.1)
}

pub fn part_1() -> usize {
    let points = parse_position(INPUT);
    let (_, max) = max_count_detects(&points).unwrap();
    max
}

#[test]
fn test_vaporize() {
    let input = r".#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....#...###..
..#.#.....#....##";
    assert_eq!(vaporize(input, 1), Some(Position { x: 8, y: 1 }));
    assert_eq!(vaporize(input, 2), Some(Position { x: 9, y: 0 }));
}

#[test]
fn test_clockwise_angle() {
    // top-left coordinates
    let clockwise_angle_between = |start: &Position, end: &Position| -> f32 {
        clockwise_degree((end.x - start.x) as f32, (start.y - end.y) as f32)
    };

    let north = clockwise_angle_between(&Position { x: 0, y: 0 }, &Position { x: 0, y: -1 });
    let north_east = clockwise_angle_between(&Position { x: 0, y: 0 }, &Position { x: 1, y: -1 });
    let east = clockwise_angle_between(&Position { x: 0, y: 0 }, &Position { x: 1, y: 0 });
    let south_east = clockwise_angle_between(&Position { x: 0, y: 0 }, &Position { x: 1, y: 1 });
    let south = clockwise_angle_between(&Position { x: 0, y: 0 }, &Position { x: 0, y: 1 });
    let south_west = clockwise_angle_between(&Position { x: 0, y: 0 }, &Position { x: -1, y: 1 });
    let west = clockwise_angle_between(&Position { x: 0, y: 0 }, &Position { x: -1, y: 0 });
    let north_west = clockwise_angle_between(&Position { x: 0, y: 0 }, &Position { x: -1, y: -1 });
    // println!(
    //     "north = {:?}, north_east = {:?}, east = {:?}, south_east = {:?}, south = {:?}, south_west = {:?}, west = {:?}, north_west = {:?}",
    //     north, north_east, east, south_east, south, south_west, west, north_west
    // );
    assert!((north - 0.).abs() <= f32::EPSILON);
    assert!((north_east - 45.).abs() <= f32::EPSILON);
    assert!((east - 90.).abs() <= f32::EPSILON);
    assert!((south_east - 135.).abs() <= f32::EPSILON);
    assert!((south - 180.).abs() <= f32::EPSILON);
    assert!((south_west - 225.).abs() <= f32::EPSILON);
    assert!((west - 270.).abs() <= f32::EPSILON);
    assert!((north_west - 315.).abs() <= f32::EPSILON);
    // println!(
    //     "test clock degree {}",
    //     clockwise_degree(0., 3.) // clockwise_angle_between(&Position { x: 8, y: 3 }, &Position { x: 8, y: 0 })
    // );
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(18, 48), 6);
    assert_eq!(gcd(0, 0), 0);
    assert_eq!(gcd(1, 0), 1);
    assert_eq!(gcd(0, 1), 1);
    assert_eq!(gcd(18, -48), 6);
    assert_eq!(gcd(11, 13), 1);
    assert_eq!(gcd(-11, 13), 1);
}

#[test]
fn test_max_detect_count() {
    let mut input = r".#..#
.....
#####
....#
...##";

    assert_eq!(
        max_count_detects(&parse_position(input)),
        Some((Position { x: 3, y: 4 }, 8))
    );

    input = r"......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";

    assert_eq!(
        max_count_detects(&parse_position(input)),
        Some((Position { x: 5, y: 8 }, 33))
    );

    input = r".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";

    assert_eq!(
        max_count_detects(&parse_position(input)),
        Some((Position { x: 11, y: 13 }, 210))
    );
}
