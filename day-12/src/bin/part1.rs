use itertools::Itertools;
use nom::{
    bytes::complete::{is_a, tag},
    character::complete::{i32, newline, space0, space1},
    multi::{many0, separated_list0, separated_list1},
    sequence::{preceded, separated_pair},
    IResult,
};
use rayon::prelude::*;
use rstest::rstest;
use std::{collections::VecDeque, fs, vec};

fn read_input() -> String {
    fs::read_to_string("src/bin/input.txt").expect("Expected input.txt")
}

#[derive(Debug, Clone)]
struct Sequence<'a> {
    prev: char,
    rem: &'a str,
    patterns: VecDeque<i32>,
}

fn get_input(input: &str) -> Vec<(&str, Vec<i32>)> {
    separated_list1(newline, parse_line)(input)
        .expect("Expected to parse file")
        .1
}

fn parse_line(input: &str) -> IResult<&str, (&str, Vec<i32>)> {
    preceded(
        space0,
        separated_pair(is_a("#.?"), space1, separated_list0(tag(","), i32)),
    )(input)
}

fn parse_gaps(input: &str) -> IResult<&str, Vec<&str>> {
    preceded(many0(is_a(".")), separated_list0(is_a("#?"), is_a(".")))(input)
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
                        combinations += 1;
                    }
                } else {
                    // Remaining sequence needs to be longer than requirement from pattern
                    let start = seq
                        .rem
                        .chars()
                        .next()
                        .expect("Expected char in next position");
                    let req = seq.patterns.iter().map(|x| x + 1).sum::<i32>() - 1;
                    let gaps = seq.rem.split(".").map(|gap| gap.len()).collect_vec();
                    let gaps_max = gaps.iter().max().unwrap();
                    let gaps_count = gaps.iter().filter(|gap| gap == &gaps_max).count();
                    let pattern_max = match seq.patterns.iter().max(){
                        Some(value) => value,
                        None => &0,
                    };
                    let pattern_count = seq.patterns.iter().filter(|pat| pat == &pattern_max).count();
                    dbg!(&gaps, &gaps_max, &gaps_count, &pattern_max, &pattern_count);
                    if req <= seq.rem.len() as i32 {
                        match start {
                            '.' => {
                                // Move string up 1 char, add to next
                                next_seq.push(Sequence {
                                    prev: '.',
                                    rem: seq.rem.strip_prefix('.').expect("Dot expected"),
                                    patterns: seq.patterns.clone(),
                                })
                            }
                            _ => {
                                if start == '?' {
                                    // Match ? to ., move string up 1 char, repeat
                                    next_seq.push(Sequence {
                                        prev: '.',
                                        rem: seq.rem.strip_prefix('?').expect("Dot expected"),
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
                                    && seq.prev != '#'
                                {
                                    // Sequence can fit, repeat with remaining sequences
                                    let mut patterns_new = seq.patterns.clone();
                                    patterns_new.remove(0);
                                    next_seq.push(Sequence {
                                        prev: '#',
                                        rem: &seq.rem[seq.patterns[0] as usize..],
                                        patterns: patterns_new,
                                    })
                                }
                            }
                        }
                    }
                }
            })
            .collect_vec();
        dbg!(&next_seq);
        active_seq = next_seq;
    }
    combinations
}

fn run(input: String) -> u64 {
    let seq_input = get_input(input.as_str())
        .into_iter()
        .map(|(seq, patterns)| (seq.to_string(), patterns))
        .collect_vec();
    let seq_inst = seq_input
        .iter()
        .map(|(seq, patterns)| Sequence {
            prev: '.',
            rem: seq.as_str(),
            patterns: VecDeque::from(patterns.clone()),
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
// #[case("???.### 1,1,3", 1)]
// #[case(".??..??...?##. 1,1,3", 4)]
// #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
// #[case("????.#...#... 4,1,1", 1)]
// #[case("????.######..#####. 1,6,5", 4)]
// #[case("?###???????? 3,2,1", 10)] // case 6
// #[case("?????????????#. 5,1,4", 10)]
// #[case(".??.?#??##???. 1,6", 42725)]
// #[case("?#???.#??#?????.? 3,1,3,1,1", 1259712)] // case 9 (med)
// #[case("??#????#??#???.?? 4,7,1", 3498125)] // case 10 (med)  15 sec / 4 sec release
// #[case("??#?.?#?#???#?#?? 1,11", 0)]
// #[case("?????????? 1,4,1", 0)] // case 12 (med)
// #[case("?#?#????..?????? 6,4", 6)]
// #[case("?#?#?.#?#???????..? 5,1,1,3,2,1", 0)]
#[case("?.#??#??.??????.? 6,1,1,1", 0)] // case 15 (extreme)
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
