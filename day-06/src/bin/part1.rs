use itertools::Itertools;
use nom::{
    bytes::complete::{is_not, tag, take_until},
    character::complete::{digit1, newline, space0, space1},
    combinator::map_res,
    multi::{many0, separated_list0, separated_list1},
    sequence::{delimited, pair, terminated, tuple},
    IResult,
};
use regex::Regex;
use std::fs;

fn read_input() -> String {
    let input: String = fs::read_to_string("src/bin/input.txt").expect("Expected input.txt");
    return input;
}

fn parse_numbers(input: &str) -> IResult<&str, u32> {
    return map_res(digit1, |s: &str| s.parse::<u32>())(input);
}

fn parse_list(input: &str) -> IResult<&str, Vec<u32>> {
    return separated_list1(space1, parse_numbers)(input);
}

fn parse_line(input: &str) -> IResult<&str, (&str, &str, &str, Vec<u32>)> {
    return tuple((is_not(":"), tag(":"), space1, parse_list))(input);
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    return input
        .lines()
        .map(|x| parse_line(x).unwrap().1 .3)
        .collect_vec();
}

fn run(input: String) -> i32 {
    let matrix = parse_input(input.as_str());
    let time = &matrix[0];
    let dist = &matrix[1];

    let mut race_mult = 1;
    for (i, j) in time.iter().zip(dist.iter()) {
        let mut races = 0;
        for k in 0..*i {
            let reach = k * (i - k);
            if reach > *j {
                races += 1;
            }
        }
        race_mult = race_mult * races;
    }
    return race_mult;
}

fn main() {
    let input: String = read_input();
    let sum = run(input);
    println!("{sum}")
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test() {
        let input: &'static str = "Time:      7  15   30
        Distance:  9  40  200";
        assert_eq!(run(input.to_string()), 288);
    }
}
