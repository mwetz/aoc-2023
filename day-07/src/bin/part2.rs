use itertools::{enumerate, Itertools};
use nom::{
    bytes::complete::{is_not, tag, take_until},
    character::complete::{digit1, newline, space0, space1},
    combinator::map_res,
    multi::{many0, separated_list0, separated_list1},
    sequence::{delimited, pair, terminated, tuple},
    IResult,
};
use regex::Regex;
use std::{fs, vec};

fn read_input() -> String {
    let input: String = fs::read_to_string("src/bin/input.txt").expect("Expected input.txt");
    return input;
}

#[derive(Debug)]
struct Hand {
    // Original chars A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2
    hand: Vec<char>,
    bid: u32,
}

impl Hand {
    fn get_group(&self) -> u8 {
        let mut freq = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let mut joker = 0;
        for c in self.hand.clone() {
            match c {
                'J' => joker += 1,
                _ => {
                    for (i, b) in vec!['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2']
                        .into_iter()
                        .enumerate()
                    {
                        if c == b {
                            freq[i] += 1;
                        }
                    }
                }
            }
        }

        let mut freq_s = freq.into_iter().sorted().rev().collect_vec();
        freq_s[0] += joker;
        match freq_s[0] {
            5 => return 10,
            4 => return 9,
            3 => match freq_s[1] {
                2 => return 8,
                _ => return 7,
            },
            2 => match freq_s[1] {
                2 => return 6,
                _ => return 5,
            },
            _ => return 4,
        }
    }

    fn get_pos(&self, i: usize) -> u8 {
        match self.hand[i] {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 1,
            'T' => 10,
            _ => String::from(self.hand[i]).parse().unwrap(),
        }
    }
}

fn parse_numbers(input: &str) -> IResult<&str, u32> {
    return map_res(digit1, |s: &str| s.parse::<u32>())(input);
}

fn parse_list(input: &str) -> IResult<&str, Vec<u32>> {
    return separated_list1(space1, parse_numbers)(input);
}

fn parse_line(input: &str) -> (&str, &str, u32) {
    return tuple((is_not(" "), space1, parse_numbers))(input)
        .expect("hand - bid pair expected")
        .1;
}

fn parse_input(input: &str) -> Vec<Hand> {
    return input
        .lines()
        .map(|x| parse_line(x))
        .into_iter()
        .map(|(hand, x, bid)| Hand {
            hand: hand.chars().collect_vec(),
            bid: bid,
        })
        .collect_vec();
}

fn run(input: String) -> u32 {
    let hand = parse_input(input.as_str());

    let hand_s: Vec<Hand> = hand
        .into_iter()
        .sorted_by(|a, b| a.get_pos(4).cmp(&b.get_pos(4)))
        .sorted_by(|a, b| a.get_pos(3).cmp(&b.get_pos(3)))
        .sorted_by(|a, b| a.get_pos(2).cmp(&b.get_pos(2)))
        .sorted_by(|a, b| a.get_pos(1).cmp(&b.get_pos(1)))
        .sorted_by(|a, b| a.get_pos(0).cmp(&b.get_pos(0)))
        .sorted_by(|a, b| a.get_group().cmp(&b.get_group()))
        .collect_vec();
    return hand_s
        .into_iter()
        .enumerate()
        .map(|(rank, hand)| (1 + rank as u32) * hand.bid)
        .sum();
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
        let input: &'static str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(run(input.to_string()), 5905);
    }
}
