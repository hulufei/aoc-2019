use std::env;

mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod intcode;

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
        Ok(51) => println!("{:?}", day5::part_1()),
        Ok(52) => println!("{:?}", day5::part_2()),
        Ok(61) => println!("{:?}", day6::part_1()),
        Ok(62) => println!("{:?}", day6::part_2()),
        Ok(71) => println!("{:?}", day7::part_1()),
        Ok(72) => println!("{:?}", day7::part_2()),
        Ok(81) => println!("{:?}", day8::part_1()),
        Ok(82) => println!("{}", day8::part_2()),
        Ok(91) => println!("{:?}", day9::part_1()),
        Ok(92) => println!("{:?}", day9::part_2()),
        Ok(101) => println!("{:?}", day10::part_1()),
        _ => panic!("Invalid pick"),
    }
}
