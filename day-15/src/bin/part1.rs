use itertools::Itertools;
use nom::bytes::complete::is_not;
use nom::{bytes::complete::tag, multi::separated_list1, IResult, Parser};
use std::collections::HashSet;
use std::{fs, iter, vec};

fn read_input() -> String {
    fs::read_to_string("src/bin/input.txt").expect("Expected to read the file")
}

fn parse_input(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(tag(","), is_not(","))(input)
}

fn run(input: String) -> u32 {
    let steps = parse_input(input.as_str()).expect("Expected valid input").1;
    // dbg!(&steps);
    steps
        .into_iter()
        .map(|x| {
            let mut current = 0;
            for i in x.chars() {
                current += i as u32;
                current = current * 17;
                current = current % 256;
            }
            current
        })
        .sum()
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
        let input: &'static str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(run(input.to_string()), 1320);
    }
}
