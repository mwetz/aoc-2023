use itertools::Itertools;





use std::fs;

fn read_input() -> String {
    let input: String = fs::read_to_string("src/bin/input.txt").expect("Expected to read the file");
    input
}

fn run(input: String) -> u64 {
    // let mut parse_time = pair(tag("Time:"), separated_list0(tag(" "), u32));
    // let mut parse_dist = pair(tag("Distance:"), separated_list0(tag(" "), u32));
    let mut li = input.lines();
    let time: u64 = li
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .split(' ')
        .filter(|x| !x.is_empty())
        .join("")
        .parse()
        .unwrap();
    let dist: u64 = li
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .split(' ')
        .filter(|x| !x.is_empty())
        .join("")
        .parse()
        .unwrap();

    let mut races = 0;
    for k in 0..time {
        let reach = k * (time - k);
        if reach > dist {
            races += 1;
        }
    }
    races
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
        let input: &'static str = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(run(input.to_string()), 71503);
    }
}
