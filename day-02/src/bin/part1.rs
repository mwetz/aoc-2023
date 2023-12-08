use regex::Regex;
use std::fs;

fn read_input() -> String {
    let input: String = fs::read_to_string("src/bin/input.txt").expect("Expected to read the file");
    input
}

struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn is_valid(&self, other: Bag) -> bool {
        return self.rounds.iter().map(|x| x.is_valid(other)).all(|x| x);
    }
}

struct Round {
    blue: u32,
    green: u32,
    red: u32,
}

impl Round {
    fn is_valid(&self, other: Bag) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }
}

#[derive(Copy, Clone)]
struct Bag {
    blue: u32,
    green: u32,
    red: u32,
}

fn parse_input(input: String) -> Vec<Game> {
    let re_id = Regex::new(r"Game (\d+)").unwrap();
    let re_red = Regex::new(r"(?<red>\d+) red").unwrap();
    let re_green = Regex::new(r"(?<green>\d+) green").unwrap();
    let re_blue = Regex::new(r"(?<blue>\d+) blue").unwrap();
    let mut games = Vec::new();
    for l in input.lines() {
        let mut l_iter = l.split(':');
        let cap_id = re_id
            .captures(l_iter.next().unwrap())
            .ok_or("no match")
            .unwrap();
        let id: u32 = cap_id.get(1).unwrap().as_str().parse().unwrap();
        let mut rounds = Vec::new();
        for i in l_iter.next().unwrap().split(';') {
            let cap_red = re_red.captures(i);
            let mut red: u32 = 0;
            if cap_red.is_some() {
                red = cap_red
                    .unwrap()
                    .name("red")
                    .map_or("0", |m| m.as_str())
                    .parse()
                    .unwrap();
            }
            let cap_green = re_green.captures(i);
            let mut green: u32 = 0;
            if cap_green.is_some() {
                green = cap_green
                    .unwrap()
                    .name("green")
                    .map_or("0", |m| m.as_str())
                    .parse()
                    .unwrap();
            }
            let cap_blue = re_blue.captures(i);
            let mut blue: u32 = 0;
            if cap_blue.is_some() {
                blue = cap_blue
                    .unwrap()
                    .name("blue")
                    .map_or("0", |m| m.as_str())
                    .parse()
                    .unwrap();
            }
            rounds.push(Round {
                blue,
                green,
                red,
            })
        }
        games.push(Game {
            id,
            rounds,
        })
    }
    games
}

fn run(input: String) -> u32 {
    let games = parse_input(input);
    let bag = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };
    let sum: u32 = games.iter().map(|x| x.id * x.is_valid(bag) as u32).sum();
    sum
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
        let input: &'static str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(run(input.to_string()), 8);
    }
}
