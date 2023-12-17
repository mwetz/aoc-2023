use itertools::Itertools;
use nom::{
    bytes::complete::{is_a, is_not, tag},
    character::complete::{alphanumeric1, digit1, i64, newline, space0, space1, u8},
    error::Error,
    multi::{count, many0, separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult, Parser, Slice,
};
use rayon::prelude::*;
use rstest::rstest;
use std::{
    cmp,
    collections::{HashMap, HashSet, VecDeque},
    fs,
    ops::Add,
    vec,
};

fn read_input() -> String {
    fs::read_to_string("src/bin/input.txt").expect("Expected input.txt")
}

fn parse_line(input: &str) -> IResult<&str, (&str, Vec<u8>)> {
    preceded(
        space0,
        separated_pair(is_a("#.?"), space1, separated_list1(tag(","), u8)),
    )(input)
}

#[derive(Debug, Clone)]
struct Sequence<'a> {
    prev: char,
    rem: &'a str,
    patterns: VecDeque<u8>,
    count: u64,
}

fn get_input(input: &str) -> Vec<(&str, Vec<u8>)> {
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
                    if seq.patterns.is_empty() {
                        // Found correct sequence, increment combinations
                        combinations += seq.count;
                    }
                } else {
                    // Remaining sequence needs to be longer than requirement from pattern
                    let start = seq
                        .rem
                        .chars()
                        .next()
                        .expect("Expected char in next position");
                    let req = seq.patterns.iter().map(|&x| 1 + x as i8).sum::<i8>() - 1;
                    if req <= seq.rem.len() as i8 {
                        match start {
                            '.' => {
                                // Move string up 1 char, add to next
                                next_seq.push(Sequence {
                                    prev: '.',
                                    rem: seq.rem.strip_prefix('.').expect("Dot expected"),
                                    patterns: seq.patterns.clone(),
                                    count: seq.count,
                                })
                            }
                            _ => {
                                if start == '?' {
                                    // Match ? to ., move string up 1 char, repeat
                                    next_seq.push(Sequence {
                                        prev: '.',
                                        rem: seq.rem.strip_prefix('?').expect("Dot expected"),
                                        patterns: seq.patterns.clone(),
                                        count: seq.count,
                                    })
                                }
                                let next_dot = seq.rem.find('.');
                                let limit: usize = match next_dot {
                                    Some(value) => std::cmp::min(seq.rem.len(), value),
                                    None => seq.rem.len(),
                                };
                                if !seq.patterns.is_empty()
                                    && seq.patterns[0] as usize <= limit
                                    && seq.prev != '#'
                                {
                                    // Sequence can fit, repeat with remaining sequences
                                    let mut patterns_new = seq.patterns.clone();
                                    patterns_new.remove(0);
                                    next_seq.push(Sequence {
                                        prev: '#',
                                        rem: &seq.rem[seq.patterns[0] as usize..],
                                        patterns: patterns_new,
                                        count: seq.count,
                                    })
                                }
                            }
                        }
                    }
                }
            })
            .collect_vec();

        // Find unique combinations, sum up their counts
        let unique_seq = next_seq
            .iter()
            .map(|seq| (seq.prev, seq.rem, seq.patterns.clone()))
            .collect::<HashSet<(char, &str, VecDeque<u8>)>>();

        let mut red_seq = Vec::new();
        for useq in unique_seq {
            let total_count = next_seq
                .iter()
                .filter(|seq| seq.prev == useq.0 && seq.rem == useq.1 && seq.patterns == useq.2)
                .map(|seq| seq.count)
                .sum::<u64>();
            red_seq.push(Sequence {
                prev: useq.0,
                rem: useq.1,
                patterns: useq.2,
                count: total_count,
            })
        }
        active_seq = red_seq;
    }
    combinations
}

fn run(input: String) -> u64 {
    let seq_input = get_input(input.as_str())
        .into_iter()
        .map(|(seq, patterns)| {
            (
                [seq; 5].join("?"),
                vec![patterns; 5].into_iter().flatten().collect_vec(),
            )
        })
        .collect_vec();

    // Unfold for part 2
    let seq_inst = seq_input
        .iter()
        .map(|(seq, patterns)| Sequence {
            prev: '.',
            rem: seq.as_str(),
            patterns: VecDeque::from(patterns.clone()),
            count: 1,
        })
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
// #[case("?????????? 1,4,1", 3268760)] // case 12 (med)
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
// #[case("??????.???..??#?? 1,2,1,4", 1)] // case 24 (extreme)
// #[case(".?#??#???#??#?????. 12,3", 0)]
// #[case("??#???#?.?##?.?? 6,3", 0)]
// #[case("?#?#?.??#???? 3,1,1,1", 0)]
fn test_run(#[case] input: String, #[case] expected: u64) {
    assert_eq!(expected, run(input))
}
