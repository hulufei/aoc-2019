use std::env;

mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let solution = env::args().nth(1).unwrap_or_default();

    match solution.parse::<usize>() {
        Ok(1) => println!("day1 answer: {}", day1::answer()),
        Ok(21) => println!("{:?}", day2::part_1()),
        Ok(22) => println!("{:?}", day2::part_2()),
        Ok(31) => println!("{:?}", day3::part_1()),
        Ok(32) => println!("{:?}", day3::part_2()),
        Ok(41) => println!("{:?}", day4::part_1()),
        Ok(42) => println!("{:?}", day4::part_2()),
        _ => panic!("Invalid pick"),
    }
}
