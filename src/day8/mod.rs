use Color::*;

const INPUT: &str = include_str!("./input");

#[derive(Debug, Clone, Copy, PartialEq)]
enum Color {
    Black = 0,
    White = 1,
    Transparent = 2,
}

impl From<u8> for Color {
    fn from(v: u8) -> Self {
        match v {
            0 => Black,
            1 => White,
            2 => Transparent,
            _ => panic!("Invalid color {}", v),
        }
    }
}

fn mix_pixel_color(first_layer: Color, second_layer: Color) -> Color {
    match (first_layer, second_layer) {
        (Transparent, c) => c,
        (c, _) => c,
    }
}

fn mix_pixel_color_in_layers(colors: &[Color]) -> Color {
    colors
        .iter()
        .fold(Transparent, |first, second| mix_pixel_color(first, *second))
}

fn split_layers(input: &str, w: usize, h: usize) -> Vec<Vec<u8>> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).expect("invalid input") as u8)
        .collect::<Vec<_>>()
        .chunks(w * h)
        .map(|v| v.to_vec())
        .collect()
}

pub fn part_1() -> usize {
    let count_digits = |layer: &Vec<u8>, digit| layer.iter().filter(|v| **v == digit).count();
    let layers = split_layers(INPUT, 25, 6);
    let layer = layers
        .iter()
        .min_by_key(|layer| count_digits(layer, 0))
        .unwrap();
    count_digits(&layer, 1) * count_digits(&layer, 2)
}

pub fn part_2() -> String {
    let w: usize = 25;
    let h: usize = 6;
    let layers = split_layers(INPUT, w, h);
    (0..w * h)
        .map(|i| {
            layers
                .iter()
                .filter_map(|layer| layer.get(i).map(|v| Color::from(*v)))
                .collect::<Vec<_>>()
        })
        .map(|colors| match mix_pixel_color_in_layers(&colors) {
            White => "⬜️",
            Black => "⬛️",
            _ => " ",
        })
        .collect::<Vec<_>>()
        .chunks(w)
        .map(|rows| rows.iter().map(|c| c.to_string()).collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn test_split_layers() {
    assert_eq!(
        split_layers("123456789012", 3, 2),
        vec![vec![1, 2, 3, 4, 5, 6], vec![7, 8, 9, 0, 1, 2]]
    );
}

#[test]
fn test_mix_pixel_color_in_layers() {
    let to_color = |input: Vec<u8>| input.into_iter().map(Color::from).collect::<Vec<_>>();
    assert_eq!(
        mix_pixel_color_in_layers(&to_color(vec![0, 1, 2, 0])),
        Black
    );
    assert_eq!(
        mix_pixel_color_in_layers(&to_color(vec![2, 1, 2, 0])),
        White
    );
    assert_eq!(
        mix_pixel_color_in_layers(&to_color(vec![2, 2, 1, 0])),
        White
    );
    assert_eq!(
        mix_pixel_color_in_layers(&to_color(vec![2, 2, 2, 0])),
        Black
    );
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(), 2975);
}
