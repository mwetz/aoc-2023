use itertools::Itertools;
use regex::Regex;
use std::{collections::HashMap, fs};

fn read_input() -> String {
    let input: String = fs::read_to_string("src/bin/input.txt").expect("Expected to read the file");
    input
}

fn update_dst_vec(
    src: &Vec<i64>,
    dst: &Vec<i64>,
    src_start: &i64,
    dst_start: &i64,
    rng: &i64,
) -> Vec<i64> {
    let src_end: i64 = src_start + rng;
    let mut dst_new = dst.clone();
    for (i, j) in src.iter().enumerate() {
        if j >= src_start && j < &src_end {
            dst_new[i] = j - src_start + dst_start;
        }
    }
    dst_new
}

fn run(input: String) -> i64 {
    let re_seeds = Regex::new(r"seeds: (?<seeds>[\d\s]+)$").unwrap();
    let re_map = Regex::new(r"(?<src>\w+)-to-(?<dst>\w+)").unwrap();
    let re_values = Regex::new(r"^(?<dst_start>\d+) (?<src_start>\d+) (?<rng>\d+)").unwrap();
    let mut src = "";
    let mut dst = "";
    let mut prodmap: HashMap<&str, Vec<i64>> = HashMap::new();
    for l in input.lines() {
        if re_seeds.is_match(l) {
            let seeds_str = re_seeds
                .captures(l)
                .unwrap()
                .name("seeds")
                .unwrap()
                .as_str();
            // println!("Seeds str {}", seeds_str);
            let seeds: Vec<i64> = seeds_str
                .split(' ')
                .map(|x| x.parse().unwrap())
                .collect_vec();
            prodmap.insert("seed", seeds);
        }
        if re_map.is_match(l) {
            src = re_map.captures(l).unwrap().name("src").unwrap().as_str();
            dst = re_map.captures(l).unwrap().name("dst").unwrap().as_str();
            // println!("Mapping {} to {}", src, dst);
            prodmap.insert(dst, prodmap.get(src).unwrap().clone());
        }
        if re_values.is_match(l) {
            let src_start: i64 = re_values
                .captures(l)
                .unwrap()
                .name("src_start")
                .unwrap()
                .as_str()
                .parse()
                .unwrap();
            let dst_start: i64 = re_values
                .captures(l)
                .unwrap()
                .name("dst_start")
                .unwrap()
                .as_str()
                .parse()
                .unwrap();
            let rng: i64 = re_values
                .captures(l)
                .unwrap()
                .name("rng")
                .unwrap()
                .as_str()
                .parse()
                .unwrap();
            let new_dst = update_dst_vec(
                prodmap.get(src).unwrap(),
                prodmap.get(dst).unwrap(),
                &src_start,
                &dst_start,
                &rng,
            );
            prodmap.insert(dst, new_dst);
        }
    }
    let location = prodmap.get(dst).unwrap().clone();
    // println!("{dst}:");
    // for l in &location {
    //     println!("{l}")
    // }
    let min = *location.iter().min().unwrap();
    min
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
        let input: &'static str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(run(input.to_string()), 35);
    }
}
