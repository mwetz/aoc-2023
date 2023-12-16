use itertools::Itertools;
use nom::{
    bytes::complete::{is_a, is_not, tag},
    character::complete::{alphanumeric1, digit1, i32, i64, newline, space0, space1},
    error::Error,
    multi::{count, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult, Parser, Slice,
};
use rayon::prelude::*;
use rstest::rstest;
use std::{collections::VecDeque, fs, ops::Add, vec};

fn read_input() -> String {
    fs::read_to_string("src/bin/input.txt").expect("Expected input.txt")
}

fn parse_line(input: &str) -> IResult<&str, (&str, Vec<i32>)> {
    preceded(
        space0,
        separated_pair(is_a("#.?"), space1, separated_list1(tag(","), i32)),
    )(input)
}

#[derive(Debug, Clone)]
struct Sequence {
    fix: String,
    rem: String,
    patterns: VecDeque<i32>,
}

fn get_input(input: &str) -> Vec<(&str, Vec<i32>)> {
    separated_list1(newline, parse_line)(input)
        .expect("Expected to parse file")
        .1
}

fn get_all_sequences(seq: Sequence) -> u64 {
    let mut combinations = 0;
    let mut active_seq = vec![seq];
    while active_seq.len() > 0 {
        let mut next_seq = vec![];
        active_seq
            .into_iter()
            .map(|seq| {
                if seq.rem.is_empty() {
                    if seq.patterns.is_empty(){
                        // Found correct sequence, increment combinations
                        combinations += 1;
                    }
                } else {
                    // Remaining sequence needs to be longer than requirement from pattern
                    let start = seq
                        .rem
                        .chars()
                        .next()
                        .expect("Expected char in next position");
                    let prev = seq.fix.chars().last();
                    let req = seq.patterns.iter().map(|x| x + 1).sum::<i32>() - 1;
                    if req <= seq.rem.len() as i32 {
                        match start {
                            '.' => {
                                // Move string up 1 char, add to next
                                next_seq.push(Sequence {
                                    fix: seq.fix.clone().add("."),
                                    rem: seq
                                        .rem
                                        .strip_prefix('.')
                                        .expect("Dot expected")
                                        .to_string(),
                                    patterns: seq.patterns.clone(),
                                })
                            }
                            _ => {
                                if start == '?' {
                                    // Match ? to ., move string up 1 char, repeat
                                    next_seq.push(Sequence {
                                        fix: seq.fix.clone().add("."),
                                        rem: seq
                                            .rem
                                            .strip_prefix('?')
                                            .expect("Dot expected")
                                            .to_string(),
                                        patterns: seq.patterns.clone(),
                                    })
                                }
                                let next_dot = seq.rem.find('.');
                                let limit: usize = if next_dot.is_some() {
                                    std::cmp::min(seq.rem.len(), next_dot.unwrap())
                                } else {
                                    seq.rem.len()
                                };
                                if !seq.patterns.is_empty()
                                    && seq.patterns[0] as usize <= limit
                                    && (prev.is_none() || prev.is_some_and(|x| x != '#'))
                                {
                                    // Sequence can fit, repeat with remaining sequences
                                    let mut patterns_new = seq.patterns.clone();
                                    patterns_new.remove(0);
                                    next_seq.push(Sequence {
                                        fix: seq
                                            .fix
                                            .clone()
                                            .add("#".repeat(seq.patterns[0] as usize).as_str()),
                                        rem: seq.rem.clone().split_off(seq.patterns[0] as usize),
                                        patterns: patterns_new,
                                    })
                                }
                            }
                        }
                    }
                }
            })
            .collect_vec();
        // dbg!(&next_seq);
        active_seq = next_seq;
    }
    combinations
}

fn unfold_seq(seq: Sequence) -> Sequence {
    let seq_new = [seq.rem.as_str(); 5].join("?");
    let pattern_new = vec![seq.patterns; 5].into_iter().flatten().collect_vec();
    Sequence {
        fix: String::new(),
        rem: seq_new.to_owned(),
        patterns: VecDeque::from(pattern_new),
    }
}

fn run(input: String) -> u64 {
    let seq_inst = get_input(input.as_str())
        .into_iter()
        .map(|(seq, patterns)| Sequence {
            fix: String::new(),
            rem: seq.to_owned(),
            patterns: VecDeque::from(patterns),
        })
        .collect_vec();
    // Unfold for part 2
    let seq_inst = seq_inst
        .into_iter()
        .map(|seq| unfold_seq(seq))
        .collect_vec();
    let seq_all = seq_inst
        .into_par_iter()
        .map(|seq| get_all_sequences(seq))
        .sum::<u64>();
    seq_all
}

fn main() {
    let input: String = read_input();
    let sum = run(input);
    println!("{sum}")
}

#[cfg(test)]
#[rstest]
#[case("???.### 1,1,3", 1)]
#[case(".??..??...?##. 1,1,3", 16384)]
#[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
#[case("????.#...#... 4,1,1", 16)]
#[case("????.######..#####. 1,6,5", 2500)]
#[case("?###???????? 3,2,1", 506250)] // case 6
#[case("?????????????#. 5,1,4", 60000)]
#[case(".??.?#??##???. 1,6", 42725)]
#[case("?#???.#??#?????.? 3,1,3,1,1", 1259712)] // case 9 (med)
#[case("??#????#??#???.?? 4,7,1", 3498125)] // case 10 (med)  15 sec / 4 sec release

// #[case("??#?.?#?#???#?#?? 1,11", 0)]
// #[case("?????????? 1,4,1", 0)] // case 12 (med)
// #[case("?#?#????..?????? 6,4", 0)]
// #[case("?#?#?.#?#???????..? 5,1,1,3,2,1", 0)]
// #[case("?.#??#??.??????.? 6,1,1,1", 0)] // case 15 (extreme)
// #[case("##???###?.##?? 2,4,2", 0)]
// #[case("???.#??#????.??? 3,7,2", 0)]
// #[case("????#??..?# 1,3,1", 0)]
// #[case(".?..#????????#????? 1,5,6", 0)]
// #[case("?#..??##??.??? 1,5,2", 0)]
// #[case("?#???#???# 1,1,2", 0)]
// #[case("#???#??..?????.??.?. 5,1,1,1,2,1", 0)]
// #[case("??#??.?#??????. 2,2,1,1,1", 0)] // case 23 (med)
// #[case("??????.???..??#?? 1,2,1,4", 0)] // case 24 (extreme)
// #[case(".?#??#???#??#?????. 12,3", 0)]
// #[case("??#???#?.?##?.?? 6,3", 0)]
// #[case("?#?#?.??#???? 3,1,1,1", 0)]
fn test_run(#[case] input: String, #[case] expected: u64) {
    assert_eq!(expected, run(input))
}
