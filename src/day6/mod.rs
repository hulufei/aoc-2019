use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("./input");

type Parent<'a> = &'a str;
type OrbitMap<'a> = HashMap<&'a str, (Parent<'a>, HashSet<&'a str>)>;

pub fn part_1() -> usize {
    total_orbits(INPUT)
}

pub fn part_2() -> usize {
    minimum_transfers(INPUT)
}

fn gen_orbits(input: &str) -> OrbitMap {
    let mut orbits: OrbitMap = HashMap::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(')').collect();
        let central = parts[0];
        let follower = parts[1];

        let (_, followers) = orbits.entry(central).or_insert((central, HashSet::new()));
        followers.insert(follower);
        orbits.entry(follower).or_insert((central, HashSet::new()));
    }

    orbits
}

fn total_orbits(input: &str) -> usize {
    let orbits: OrbitMap = gen_orbits(input);

    walk_down_count(&orbits, "COM", 0)
}

fn walk_down_count(orbits: &OrbitMap, central: &str, level: usize) -> usize {
    match orbits.get(central) {
        Some((_, followers)) => {
            followers.len() * (1 + level)
                + followers
                    .iter()
                    .map(|central| walk_down_count(orbits, central, level + 1))
                    .sum::<usize>()
        }
        None => 0,
    }
}

fn get_parents(orbits: &OrbitMap, obj: &str) -> Vec<String> {
    let mut central = obj;
    let mut ret = vec![];
    loop {
        match (central, orbits.get(central)) {
            ("COM", _) => break,
            (_, Some((parent, _))) => {
                // println!("central = {}, parent = {}", central, parent);
                ret.push(parent.to_string());
                central = parent;
            }
            _ => (),
        }
    }
    ret
}

fn minimum_transfers(input: &str) -> usize {
    let orbits = gen_orbits(input);

    println!("{:?}", orbits);
    println!("{:?}", orbits.get("YOU"));
    println!("{:?}", orbits.get("TFB"));
    println!("{:?}", orbits.get("DLZ"));

    return 0;
    get_parents(&orbits, "YOU")
        .iter()
        .collect::<HashSet<_>>()
        .symmetric_difference(&get_parents(&orbits, "SAN").iter().collect())
        .collect::<HashSet<_>>()
        .len()
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

#[test]
fn test_minimum_transfers() {
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
K)L
K)YOU
I)SAN";
    assert_eq!(minimum_transfers(input), 4);
}
