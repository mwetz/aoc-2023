use itertools::Itertools;
use ndarray::{s, Array2, ArrayBase, Slice};
use nom::{
    bytes::complete::{is_a, is_not, tag, take_until, take_while1},
    character::complete::{
        alphanumeric1, anychar, char, digit1, i64, line_ending, multispace0, newline, space0,
        space1,
    },
    multi::{many1, separated_list1},
    sequence::{pair, preceded, terminated},
    IResult, Parser,
};
use std::{collections::HashSet, ops::Sub};
use std::{fs, iter, vec};

fn read_input() -> String {
    fs::read_to_string("src/bin/input.txt").expect("Expected input.txt")
}

fn parse_line(input: &str) -> IResult<&str, &str> {
    preceded(space0, is_a("#."))(input)
}

fn parse_block(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(line_ending, parse_line)(input)
}

fn get_input(input: &str) -> IResult<&str, Vec<Vec<&str>>> {
    separated_list1(pair(newline, newline), parse_block)(input)
}

fn build_matrix(nested: Vec<Vec<i32>>) -> Array2<i32> {
    let inner_shape = nested[0].len();
    let shape = (nested.len(), inner_shape);
    let flat: Vec<i32> = nested.iter().flatten().cloned().collect();
    Array2::from_shape_vec(shape, flat).expect("Expected to build array")
}

fn find_mirror_line(array: Array2<i32>) -> i32 {
    let mut mirrors = Vec::new();
    for i in 0..array.shape()[0] - 1 {
        let r_check = std::cmp::min(i, (array.shape()[0] - 2) - i);
        let s_original = array.slice(s![i - r_check..=i, ..]);
        let s_mirror = array.slice(s![i+1..=i+1+r_check;-1, ..]);
        let diff = ndarray::Zip::from(&s_original).and(&s_mirror).map_collect(|&a, &b| a != b);
        let diff_sum = diff.iter().map(|&x| if x { return 1} else { return 0}).sum::<u32>();
        if diff_sum == 1 {
            mirrors.push((i as i32 + 1) * 100);
        }
    }
    for i in 0..array.shape()[1] - 1 {
        let r_check = std::cmp::min(i, (array.shape()[1] - 2) - i);
        let s_original = array.slice(s![.., i - r_check..=i]);
        let s_mirror = array.slice(s![..,i+1..=i+1+r_check;-1]);
        let diff = ndarray::Zip::from(&s_original).and(&s_mirror).map_collect(|&a, &b| a != b);
        let diff_sum = diff.iter().map(|&x| if x { return 1} else { return 0}).sum::<u32>();
        if diff_sum == 1 {
            mirrors.push(i as i32 + 1);
        }
    }
    // dbg!(&mirrors);
    mirrors.iter().sum()
}

fn run(input: String) -> i32 {
    let (_, fields) = get_input(input.as_str()).expect("Expected to parse file");
    let fields_num = fields
        .iter()
        .map(|x| {
            x.iter()
                .map(|&y| {
                    y.chars()
                        .into_iter()
                        .map(|z| match z {
                            '#' => 1,
                            _ => 0,
                        })
                        .collect_vec()
                })
                .collect_vec()
        })
        .collect_vec();
    let fields_mat = fields_num
        .iter()
        .cloned()
        .map(|x| build_matrix(x))
        .collect_vec();
    let mirror_lines = fields_mat
        .iter()
        .cloned()
        .map(|x| find_mirror_line(x))
        .collect_vec();
    // dbg!(&mirror_lines);
    mirror_lines.into_iter().sum()
    // 0
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
        let input: &'static str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(run(input.to_string()), 400);
    }
}
