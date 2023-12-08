
use nom::{
    bytes::complete::{tag},
    character::complete::{alphanumeric1, newline, space0},
    multi::{separated_list1},
    sequence::{delimited, pair, preceded, separated_pair},
    IResult,
};

use std::{collections::HashMap, fs};

fn read_input() -> String {
    let input: String = fs::read_to_string("src/bin/input.txt").expect("Expected input.txt");
    input
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
    let map = nodesvec.into_iter().collect::<HashMap<_, _>>();
    (directions, map)
}

fn run(input: String) -> u32 {
    let (directions, map) = parse_input(input.as_str());
    let mut nodeid = "AAA";
    let mut nextnodes = &map[nodeid];
    let mut steps = 0;
    for d in directions.repeat(1000).chars() {
        match nodeid {
            "ZZZ" => return steps,
            _ => {
                steps += 1;
                nodeid = match d {
                    'L' => nextnodes[0],
                    'R' => nextnodes[1],
                    _ => "",
                };
                nextnodes = &map[nodeid]
            }
        }
    }
    0
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
        let input: &'static str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(run(input.to_string()), 6);
    }
}
