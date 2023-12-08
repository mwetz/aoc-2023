use regex::Regex;
use std::fs;

fn read_input() -> String {
    let input: String =
        fs::read_to_string("src/bin/input.txt").expect("Expected to read the file");
    input
}

fn to_digit(s: &str) -> &str {
    match s {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => s,
    }
}

fn get_value(s: String) -> u32 {
    let re_first = Regex::new(r"^.*?(\d|one|two|three|four|five|six|seven|eight|nine){1}").unwrap();
    let re_last = Regex::new(r".*(\d|one|two|three|four|five|six|seven|eight|nine).*?$").unwrap();

    let cap_first = re_first.captures(s.as_str()).ok_or("no match").unwrap();
    let first = cap_first.get(1).unwrap().as_str();

    let cap_last = re_last.captures(s.as_str()).ok_or("no match").unwrap();
    let last = cap_last.get(1).unwrap().as_str();

    let c = to_digit(first).to_owned() + to_digit(last);
    c.parse().unwrap()
}

fn get_sum(input: String) -> u32 {
    let mut sum = 0;
    for i in input.lines() {
        sum += get_value(i.to_string());
    }
    sum
}

fn main() {
    let input: String = read_input();
    println!("{}", { get_sum(input) });
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test() {
        let input: &'static str = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        assert_eq!(get_sum(input.to_string()), 281);
    }
}
