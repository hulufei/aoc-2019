use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("./input");

type OrbitMap<'a> = HashMap<&'a str, HashSet<&'a str>>;

pub fn part_1() -> usize {
    total_orbits(INPUT)
}

fn total_orbits(input: &str) -> usize {
    let mut orbits: OrbitMap = HashMap::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(')').collect();
        let central = parts[0];
        let follower = parts[1];

        orbits
            .entry(central)
            .or_insert_with(HashSet::new)
            .insert(follower);
    }

    walk_down_count(&orbits, "COM", 0)
}

fn walk_down_count(orbits: &OrbitMap, central: &str, level: usize) -> usize {
    match orbits.get(central) {
        Some(followers) => {
            followers.len() * (1 + level)
                + followers
                    .iter()
                    .map(|central| walk_down_count(orbits, central, level + 1))
                    .sum::<usize>()
        }
        None => 0,
    }
}

#[test]
fn test_total_orbits() {
    let input = r"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";
    assert_eq!(total_orbits(input), 42);
}
