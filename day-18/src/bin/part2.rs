use glam::i64::I64Vec2;
use itertools::Itertools;
use nom::bytes::complete::{tag, take_while_m_n};
use nom::character::complete::{alphanumeric1, anychar, space1};
use nom::character::complete::{line_ending, space0, u32};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded, tuple};
use nom::{HexDisplay, IResult, Parser};
use rayon::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

fn read_input() -> String {
    fs::read_to_string("src/bin/input.txt").expect("Expected to read the file")
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn hex_color(input: &str) -> IResult<&str, &str> {
    delimited(tag("(#"), alphanumeric1, tag(")"))(input)
}

fn parse_line(input: &str) -> IResult<&str, (char, u32, &str)> {
    preceded(
        space0,
        tuple((anychar, preceded(space1, u32), preceded(space1, hex_color))),
    )(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<(char, u32, &str)>> {
    separated_list1(line_ending, parse_line)(input)
}

fn get_adjecent(point: I64Vec2) -> Vec<I64Vec2> {
    vec![
        point,
        point + I64Vec2 { x: -1, y: 0 },
        point + I64Vec2 { x: 1, y: 0 },
        point + I64Vec2 { x: 0, y: -1 },
        point + I64Vec2 { x: 0, y: 1 },
    ]
}

fn run(input: String) -> i64 {
    let (_, res) = parse_input(input.as_str()).expect("Expected to parse input");
    let mut pos = I64Vec2 { x: 1, y: 1 };
    let mut path = vec![pos];
    for (_, _, color) in res {
        let steps = i64::from_str_radix(color.chars().take(5).join("").as_str(), 16).unwrap();
        let dir = color.chars().nth(5).unwrap();
        match dir {
            '3' => {
                pos += I64Vec2 {
                    x: 0,
                    y: -(steps ),
                };
                path.push(pos)
            }
            '1' => {
                pos += I64Vec2 {
                    x: 0,
                    y: (steps ),
                };
                path.push(pos)
            }
            '2' => {
                pos += I64Vec2 {
                    x: -(steps ),
                    y: 0,
                };
                path.push(pos)
            }
            '0' => {
                pos += I64Vec2 {
                    x: (steps ),
                    y: 0,
                };
                path.push(pos)
            }
            _ => {}
        }
    }
    // dbg!(&path.iter());
    let mut area: i64 = 0;
    let mut delim: i64 = 0;
    for p in path.windows(2) {
        // dbg!(p, p[0].x as i64 * p[1].y as i64, p[0].y as i64 * p[1].x as i64);
        area += p[0].x * p[1].y;
        area -= p[0].y * p[1].x;
        delim += (p[0].x - p[1].x).abs() + (p[0].y - p[1].y).abs()
    }
    delim = (delim + 2) / 2;
    // dbg!(area, delim);
    area / 2 + delim
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
        let input: &'static str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        assert_eq!(run(input.to_string()), 952408144115);
    }
}
