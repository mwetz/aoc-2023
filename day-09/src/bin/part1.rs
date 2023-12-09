use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, digit1, i64, newline, space0, space1},
    multi::separated_list1,
    sequence::{delimited, pair, preceded, separated_pair},
    IResult, Parser,
};

use std::{collections::HashMap, fs};

fn read_input() -> String {
    let input: String = fs::read_to_string("src/bin/input.txt").expect("Expected input.txt");
    input
}

fn parse_line(input: &str) -> IResult<&str, Vec<i64>> {
    preceded(space0, separated_list1(space1, i64))(input)
}

fn parse_block(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    separated_list1(newline, parse_line)(input)
}

fn get_input(input: &str) -> Vec<Vec<i64>> {
    parse_block(input).expect("Expected to parse file").1
}

fn get_next_element(measures: &[i64]) -> i64 {
    let mut measures_new = measures.to_owned();
    let mut last_reading: Vec<i64> = Vec::new();
    for _i in 0.. {
        if measures_new.iter().all(|&x| x == 0) {
            return last_reading.iter().sum();
        } else {
            last_reading.push(*measures_new.iter().last().expect("Last element expected"));
            let measures_old = measures_new;
            measures_new = measures_old
                .iter()
                .tuple_windows()
                .map(|(a, b)| b - a)
                .collect_vec();
        }
    }
    0
}

fn run(input: String) -> i64 {
    let full_readings = get_input(input.as_str());
    full_readings.iter().map(|x| get_next_element(x)).sum()
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
        let input: &'static str = "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45";
        assert_eq!(run(input.to_string()), 114);
    }
}
