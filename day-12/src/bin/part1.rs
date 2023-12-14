use itertools::Itertools;
use nom::{
    bytes::complete::{is_a, is_not, tag},
    character::complete::{alphanumeric1, digit1, i64, newline, space0, space1, u32},
    error::Error,
    multi::{count, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult, Parser, Slice,
};
use rayon::prelude::*;
use std::{fs, ops::Add, vec};

fn read_input() -> String {
    fs::read_to_string("src/bin/input.txt").expect("Expected input.txt")
}

fn parse_line(input: &str) -> IResult<&str, (&str, Vec<u32>)> {
    preceded(
        space0,
        separated_pair(is_a("#.?"), space1, separated_list1(tag(","), u32)),
    )(input)
}

fn get_input(input: &str) -> Vec<(&str, Vec<u32>)> {
    separated_list1(newline, parse_line)(input)
        .expect("Expected to parse file")
        .1
}

// fn get_all_sequences(seq_fix: String, seq_rem: String, patterns: Vec<u32>) -> Vec<String> {
fn get_all_sequences(seq_fix: String, seq_rem: String, patterns: Vec<u32>) -> u64 {
    let mut substrings = 0;
    if !seq_rem.is_empty() {
        let start = seq_rem
            .chars()
            .next()
            .expect("Expected char in next position");
        let prev = seq_fix.chars().last();
        match start {
            '.' => {
                // Move string up 1 char, repeat
                return get_all_sequences(
                    seq_fix.add("."),
                    seq_rem.strip_prefix('.').expect("Dot expected").to_string(),
                    patterns,
                );
            }
            _ => {
                if start == '?' {
                    // Match ? to ., move string up 1 char, repeat
                    let all_seq = get_all_sequences(
                        seq_fix.clone().add("."),
                        seq_rem
                            .strip_prefix('?')
                            .expect("Questionmark expected")
                            .to_string(),
                        patterns.clone(),
                    );
                    substrings += all_seq;
                }
                let next_dot = seq_rem.find('.');
                let limit: usize = if next_dot.is_some() {
                    std::cmp::min(seq_rem.len(), next_dot.unwrap())
                } else {
                    seq_rem.len()
                };
                if !patterns.is_empty()
                    && patterns[0] as usize <= limit
                    && (prev.is_none() || prev.is_some_and(|x| x != '#'))
                {
                    let mut pattern_new = patterns.clone();
                    pattern_new.remove(0);
                    // Sequence can fit, repeat with remaining sequences
                    let all_seq = get_all_sequences(
                        seq_fix
                            .clone()
                            .add("#".repeat(patterns[0] as usize).as_str()),
                        seq_rem.clone().split_off(patterns[0] as usize),
                        pattern_new,
                    );
                    // substrings.extend(all_seq);
                    substrings += all_seq;
                }
                return substrings;
            }
        }
    }
    if patterns.is_empty() && seq_rem.is_empty() {
        // return vec![seq_fix];
        return 1;
    }
    0
}

fn run(input: String) -> u64 {
    let seq_inst = get_input(input.as_str());
    let seq_all = seq_inst
        .par_iter()
        .map(|(seq, patterns)| get_all_sequences("".to_string(), seq.to_string(), patterns.clone()))
        .sum::<u64>();
    dbg!(&seq_all);
    seq_all
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
        let input: &'static str = "???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1";
        assert_eq!(run(input.to_string()), 21);
    }
}
