use std::collections::HashMap;
use std::ops::Range;

fn validate_rule_1(pass: u32) -> bool {
    let digit_chars: Vec<char> = pass.to_string().chars().collect();

    digit_chars.windows(2).all(|items| items[0] <= items[1])
        && digit_chars.windows(2).any(|items| items[0] == items[1])
}

fn validate_rule_2(pass: u32) -> bool {
    let digit_chars: Vec<char> = pass.to_string().chars().collect();
    let mut pairs = HashMap::new();

    for items in digit_chars.windows(2) {
        // Refutability: Whether a Pattern Might Fail to Match
        // https://doc.rust-lang.org/book/ch18-02-refutability.html
        if let [a, b] = items {
            if a > b {
                return false;
            }

            if a == b {
                *pairs.entry(a).or_insert(1) += 1;
            }
        }
    }

    pairs.values().any(|count| *count == 2)
}

const INPUT: Range<u32> = (359_282..820_401);

pub fn part_1() -> usize {
    INPUT.filter(|v| validate_rule_1(*v)).count()
}

pub fn part_2() -> usize {
    INPUT.filter(|v| validate_rule_2(*v)).count()
}

#[test]
fn test_valid_rule_2() {
    assert_eq!(validate_rule_2(112_233), true);
    assert_eq!(validate_rule_2(111_122), true);
    assert_eq!(validate_rule_2(123_444), false);
    assert_eq!(validate_rule_2(111_123), false);
    assert_eq!(validate_rule_2(144_446), false);
    assert_eq!(validate_rule_2(455_888), true);
    assert_eq!(validate_rule_2(344_445_667), true);
}
