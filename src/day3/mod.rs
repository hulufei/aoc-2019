use std::fs;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point(isize, isize);

impl Point {
    fn new((x, y): (isize, isize)) -> Self {
        Self(x, y)
    }
}

#[derive(Debug, PartialEq)]
struct Line(Point, Point);

impl Line {
    fn draw(start: Point, dest: &str) -> Self {
        let Point(x, y) = start;
        let (direction, distance) = dest.split_at(1);
        let distance: isize = distance.parse().unwrap();
        let end = match direction {
            "R" => (x + distance, y),
            "L" => (x - distance, y),
            "D" => (x, y - distance),
            "U" => (x, y + distance),
            _ => panic!("Unknow direction {}", dest),
        };

        Self(start, Point::new(end))
    }

    // https://www.geeksforgeeks.org/program-for-point-of-intersection-of-two-lines/
    fn intersection(&self, other: &Self) -> Option<Point> {
        let Self(Point(self_start_x, self_start_y), Point(self_end_x, self_end_y)) = self;
        let Self(Point(other_start_x, other_start_y), Point(other_end_x, other_end_y)) = other;

        let a1 = self_end_y - self_start_y;
        let b1 = self_start_x - self_end_x;
        let c1 = a1 * self_start_x + b1 * self_start_y;

        let a2 = other_end_y - other_start_y;
        let b2 = other_start_x - other_end_x;
        let c2 = a2 * other_start_x + b2 * other_start_y;

        let delta = a1 * b2 - a2 * b1;

        if delta == 0 {
            return None;
        }

        Some(Point(
            (b2 * c1 - b1 * c2) / delta,
            (a1 * c2 - a2 * c1) / delta,
        ))
    }
}

fn draw_line(path: &str) -> Vec<Line> {
    let mut start = Point::new((0, 0));

    path.split(',')
        .map(|dest| {
            let line = Line::draw(start, dest);
            start = line.1;
            line
        })
        .collect()
}

fn intersect_points(wire1: &Vec<Line>, wire2: &Vec<Line>) -> Vec<Point> {
    wire2
        .iter()
        .flat_map(|line| wire1.iter().filter_map(move |x| x.intersection(line)))
        .collect()
}

fn closest_intersection_manhattan_distance(intersections: Vec<Point>) -> Option<isize> {
    intersections
        .iter()
        .filter(|p| p.0 > 0 && p.1 > 0)
        .map(manhattan_distance)
        .min()
}

fn manhattan_distance(point: &Point) -> isize {
    point.0.abs() + point.1.abs()
}

pub fn part_1() -> isize {
    let wires: Vec<Vec<Line>> = fs::read_to_string("src/day3/input")
        .unwrap()
        .lines()
        .map(draw_line)
        .collect();

    closest_intersection_manhattan_distance(intersect_points(&wires[0], &wires[1])).unwrap()
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(), 4);
}

#[test]
fn test_line_draw() {
    assert_eq!(Point::new((0, 0)), Point(0, 0));

    assert_eq!(
        Line::draw(Point(0, 0), "R12"),
        Line(Point(0, 0), Point(12, 0))
    );
    assert_eq!(
        Line::draw(Point(0, 0), "L12"),
        Line(Point(0, 0), Point(-12, 0))
    );
    assert_eq!(
        Line::draw(Point(0, 0), "D12"),
        Line(Point(0, 0), Point(0, -12))
    );
    assert_eq!(
        Line::draw(Point(0, 0), "U12"),
        Line(Point(0, 0), Point(0, 12))
    );
}

#[test]
fn test_line_intersection() {
    let horizon_line = Line::draw(Point(0, 0), "R12");
    let vertical_line = Line::draw(Point(0, 0), "U12");

    assert_eq!(horizon_line.intersection(&horizon_line), None);
    assert_eq!(
        horizon_line.intersection(&Line::draw(Point(0, 0), "R13")),
        None
    );
    assert_eq!(
        horizon_line.intersection(&Line::draw(Point(0, 0), "L1")),
        None
    );
    assert_eq!(
        horizon_line.intersection(&Line::draw(Point(0, 0), "U12")),
        Some(Point(0, 0))
    );
    assert_eq!(
        horizon_line.intersection(&Line::draw(Point(6, 10), "D12")),
        Some(Point(6, 0))
    );

    assert_eq!(vertical_line.intersection(&vertical_line), None);
    assert_eq!(
        vertical_line.intersection(&Line::draw(Point(0, 0), "U13")),
        None
    );
    assert_eq!(
        vertical_line.intersection(&Line::draw(Point(0, 0), "D1")),
        None
    );
    assert_eq!(
        vertical_line.intersection(&Line::draw(Point(-1, 1), "R10")),
        Some(Point(0, 1))
    )
}
