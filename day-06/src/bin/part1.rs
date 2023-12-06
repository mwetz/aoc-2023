use std::fs;
use itertools::Itertools;
use regex::Regex;
use nom::multi::{separated_list1, separated_list0};
use nom::bytes::complete::tag;
use nom::character::complete::{newline,space0,u32};
use nom::sequence::{delimited,pair};

fn read_input() -> String {
    let input: String = fs::read_to_string("src/bin/input.txt").expect("Expected to read the file");
    return input;
}

fn run(input: String) -> i32 {
    // let mut parse_time = pair(tag("Time:"), separated_list0(tag(" "), u32));
    // let mut parse_dist = pair(tag("Distance:"), separated_list0(tag(" "), u32));
    let mut li = input.lines().into_iter();
    let time: Vec<i32> = li.next().unwrap().split(":").last().unwrap().split(" ").filter(|x| x.len() > 0).map(|x| x.parse().unwrap()).collect_vec();
    let dist: Vec<i32> = li.next().unwrap().split(":").last().unwrap().split(" ").filter(|x| x.len() > 0).map(|x| x.parse().unwrap()).collect_vec();
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
        println!("{}", races)
    }
    return race_mult
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