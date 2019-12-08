const INPUT: &str = include_str!("./input");

fn split_layers(input: &str, w: u32, h: u32) -> Vec<Vec<u32>> {
    input
        .trim()
        .chars()
        .map(|c| c.to_string().parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
        .chunks((w * h) as usize)
        .map(|v| v.to_vec())
        .collect()
}

pub fn part_1() -> usize {
    let count_digits = |layer: &Vec<u32>, digit| layer.iter().filter(|v| **v == digit).count();
    let layers = split_layers(INPUT, 25, 6);
    let layer = layers
        .iter()
        .min_by_key(|layer| count_digits(layer, 0))
        .unwrap();
    count_digits(&layer, 1) * count_digits(&layer, 2)
}

#[test]
fn test_split_layers() {
    assert_eq!(
        split_layers("123456789012", 3, 2),
        vec![vec![1, 2, 3, 4, 5, 6], vec![7, 8, 9, 0, 1, 2]]
    );
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(), 2975);
}
