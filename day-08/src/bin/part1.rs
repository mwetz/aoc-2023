use itertools::{enumerate, Itertools};
use nom::{
    bytes::complete::{is_not, tag, take_until},
    character::complete::{alphanumeric1, digit1, newline, space0, space1},
    combinator::map_res,
    multi::{many0, many1, separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult,
};
use regex::Regex;
use std::{collections::HashMap, fs, vec};

fn read_input() -> String {
    let input: String = fs::read_to_string("src/bin/input.txt").expect("Expected input.txt");
    return input;
}

// pub struct Arena {
//     nodes: Vec<Node>,
// }

// pub struct NodeId {
//     id: String,
// }

// pub struct Node {
//     left: Option<NodeId>,
//     right: Option<NodeId>,
// }
// impl Arena {
//     pub fn new_node(&mut self, id, left, right) -> NodeId {
//         // Get the next free index
//         let next_index = self.nodes.len();

//         // Push the node into the arena
//         self.nodes.push(Node {
//             parent: None,
//             first_child: None,
//             last_child: None,
//             previous_sibling: None,
//             next_sibling: None,
//             data: data,
//         });

//         // Return the node identifier
//         NodeId { index: next_index }

// }
// }

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
    return (directions, map);
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
    return 0;
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
