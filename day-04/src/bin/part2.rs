use itertools::Itertools;
use std::fs;

fn read_input() -> String {
    let input: String = fs::read_to_string("src/bin/input.txt").expect("Expected to read the file");
    input
}

fn get_matches(line: &str) -> u32 {
    let l_card = line.split(':').collect_vec();
    let mut l_iter = l_card.get(1).unwrap().split('|');
    let winning = l_iter
        .next()
        .unwrap()
        .split(' ')
        .filter(|x| !x.is_empty())
        .collect_vec();
    let check = l_iter
        .next()
        .unwrap()
        .split(' ')
        .filter(|x| !x.is_empty())
        .collect_vec();
    let matches = check.iter().map(|x| winning.contains(x) as u32).sum();
    matches
}

fn run(input: String) -> u32 {
    let mut cards = 0;
    let mut card_mult: Vec<u32> = vec![1; input.lines().collect_vec().len()];
    for (i, l) in input.lines().enumerate() {
        let wins = get_matches(l);
        cards += card_mult[i];
        // println!("{}: {} cards won {} times", i, card_mult[i], wins);
        for j in 0..wins {
            card_mult[1 + i + j as usize] += card_mult[i]
        }
    }
    cards
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
        let input: &'static str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(run(input.to_string()), 30);
    }
}
