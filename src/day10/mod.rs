const INPUT: &str = include_str!("./input");

#[derive(Debug, PartialEq)]
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

fn angle(p1: &Position, p2: &Position) -> (i32, i32) {
    let y = p1.y - p2.y;
    let x = p1.x - p2.x;
    let gcd = gcd(y, x);

    (x / gcd, y / gcd)
}

fn count_detects(points: &[Position], p: &Position) -> usize {
    let mut angles: Vec<_> = points
        .iter()
        .filter(|v| *v != p)
        .map(|v| angle(v, p))
        .collect();
    angles.sort();
    angles.dedup();
    angles.len()
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

fn max_count_detects(input: &str) -> Option<usize> {
    let points = parse_position(input);
    points.iter().map(|p| count_detects(&points, p)).max()
}

pub fn part_1() -> usize {
    max_count_detects(INPUT).unwrap()
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

    assert_eq!(max_count_detects(input), Some(8));

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

    assert_eq!(max_count_detects(input), Some(33));

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

    assert_eq!(max_count_detects(input), Some(210));
}
