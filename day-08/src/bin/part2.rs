use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, newline, space0},
    multi::separated_list1,
    sequence::{delimited, pair, preceded, separated_pair},
    IResult,
};
use num::integer::lcm;
use std::{collections::HashMap, fs};

fn read_input() -> String {
    fs::read_to_string("src/bin/input.txt").expect("Expected input.txt")
}

fn parse_directions(input: &str) -> IResult<&str, &str> {
    preceded(space0, alphanumeric1)(input)
}

fn parse_node(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
    preceded(
        space0,
        separated_pair(
            alphanumeric1,
            tag(" = "),
            delimited(
                tag("("),
                separated_list1(tag(", "), alphanumeric1),
                tag(")"),
            ),
        ),
    )(input)
}

fn parse_block(input: &str) -> IResult<&str, Vec<(&str, Vec<&str>)>> {
    separated_list1(newline, parse_node)(input)
}

fn parse_input(input: &str) -> (&str, HashMap<&str, Vec<&str>>) {
    let (_, (directions, nodesvec)) =
        separated_pair(parse_directions, pair(newline, newline), parse_block)(input)
            .expect("Expected to parse file");
    let map = nodesvec.into_iter().collect::<HashMap<&str, Vec<&str>>>();
    (directions, map)
}

fn find_sequence(start: &str, directions: &str, map: &HashMap<&str, Vec<&str>>) -> u64 {
    let mut nodeid = start;
    let mut nextnodes = &map[nodeid];
    let mut steps = 0;
    let mut steps_to_first = 0;
    let mut steps_to_next = 0;
    for d in directions.repeat(100000).chars() {
        steps += 1;
        nodeid = match d {
            'L' => nextnodes[0],
            'R' => nextnodes[1],
            _ => "",
        };
        nextnodes = &map[nodeid];
        if nodeid.ends_with('Z') {
            if steps_to_first > 0 {
                steps_to_next = steps;
            }
            if steps_to_first == 0 {
                steps_to_first = steps;
            }
            if steps_to_first > 0 && steps_to_next > 0 {
                return steps_to_next;
            }
            steps = 0;
        }
    }
    0
}

fn run(input: String) -> u64 {
    let (directions, map) = parse_input(input.as_str());
    let nodeid = map.keys().filter(|x| x.ends_with('A')).collect_vec();
    let steps = nodeid
        .iter()
        .map(|x| find_sequence(x, directions, &map))
        .collect_vec();
    let mut steps_full = 1;
    for i in steps.iter() {
        steps_full = lcm(steps_full, *i)
    }
    steps_full
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
        let input: &'static str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(run(input.to_string()), 6);
    }
}
