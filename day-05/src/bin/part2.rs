use itertools::Itertools;
use regex::Regex;
use std::{collections::HashMap, fs};

fn read_input() -> String {
    let input: String = fs::read_to_string("src/bin/input.txt").expect("Expected to read the file");
    return input;
}

#[derive(Copy, Clone)]
struct StartRange {
    start: i64,
    range: i64,
}

fn update_dst_vec(
    src: &Vec<StartRange>,
    dst: &Vec<StartRange>,
    src_start: i64,
    dst_start: i64,
    rng: i64,
) -> (Vec<StartRange>, Vec<StartRange>) {
    // mappings are always increasing
    let mut src_new = Vec::new();
    let mut dst_new = Vec::new();
    for (j, k) in src.iter().zip(dst.iter()) {
        if j.start < src_start && src_start + rng < j.start + j.range {
            // Map fully contained => generate 3 parts
            let src_before = StartRange {
                start: j.start,
                range: src_start - j.start,
            };
            let dst_before = StartRange {
                start: k.start,
                range: src_start - j.start,
            };
            let src_inside = StartRange {
                start: src_start,
                range: rng,
            };
            let dst_inside = StartRange {
                start: src_start - src_start + dst_start,
                range: rng,
            };
            let src_after = StartRange {
                start: j.start + (src_start - j.start) + rng,
                range: j.start + j.range - (src_start + rng),
            };
            let dst_after = StartRange {
                start: k.start + (src_start - j.start) + rng,
                range: j.start + j.range - (src_start + rng),
            };
            src_new.extend(vec![src_before, src_inside, src_after]);
            dst_new.extend(vec![dst_before, dst_inside, dst_after]);
        } else if j.start >= src_start && j.start + j.range <= src_start + rng {
            // Interval fully contained => generate 1 part
            let src_inside = StartRange {
                start: j.start,
                range: j.range,
            };
            let dst_inside = StartRange {
                start: j.start - src_start + dst_start,
                range: j.range,
            };
            src_new.extend(vec![src_inside]);
            dst_new.extend(vec![dst_inside]);
            // assert_eq!(.iter().map(|x| x.range).sum(), src_new.iter().map(|x| x.range).sum())
        } else if j.start >= src_start
            && j.start < src_start + rng
            && j.start + j.range > src_start + rng
        {
            // Only start in map => generate 2 parts
            let src_inside = StartRange {
                start: j.start,
                range: (src_start + rng) - j.start,
            };
            let dst_inside = StartRange {
                start: j.start - src_start + dst_start,
                range: (src_start + rng) - j.start,
            };
            let src_after = StartRange {
                start: j.start + (src_start - j.start) + rng,
                range: j.start + j.range - (src_start + rng),
            };
            let dst_after = StartRange {
                start: k.start + (src_start - j.start) + rng,
                range: j.start + j.range - (src_start + rng),
            };
            src_new.extend(vec![src_inside, src_after]);
            dst_new.extend(vec![dst_inside, dst_after]);
        } else if j.start < src_start
            && src_start < j.start + j.range
            && j.start + j.range <= src_start + rng
        {
            // Only end in map => generate 2 parts
            let src_before = StartRange {
                start: j.start,
                range: src_start - j.start,
            };
            let dst_before = StartRange {
                start: k.start,
                range: src_start - j.start,
            };
            let src_inside = StartRange {
                start: src_start,
                range: j.start + j.range - src_start,
            };
            let dst_inside = StartRange {
                start: src_start - src_start + dst_start,
                range: j.start + j.range - src_start,
            };
            src_new.extend(vec![src_before, src_inside]);
            dst_new.extend(vec![dst_before, dst_inside]);
        } else {
            // Nothing in map => return original part
            let src = StartRange {
                start: j.start,
                range: j.range,
            };
            let dst = StartRange {
                start: k.start,
                range: j.range,
            };
            src_new.extend(vec![src]);
            dst_new.extend(vec![dst]);
        }
    }
    return (src_new, dst_new);
}

fn run(input: String) -> i64 {
    let re_seeds = Regex::new(r"seeds: (?<seeds>[\d\s]+)$").unwrap();
    let re_map = Regex::new(r"(?<src>\w+)-to-(?<dst>\w+)").unwrap();
    let re_values = Regex::new(r"^(?<dst_start>\d+) (?<src_start>\d+) (?<rng>\d+)").unwrap();
    let mut src = "";
    let mut dst = "";
    let mut prodmap: HashMap<&str, Vec<StartRange>> = HashMap::new();
    for l in input.lines() {
        if re_seeds.is_match(l) {
            let mut seed_rngs: Vec<StartRange> = Vec::new();
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
            for c in &seeds.into_iter().chunks(2) {
                let (i, j) = c.collect_tuple().unwrap();
                seed_rngs.push(StartRange { start: i, range: j })
            }
            prodmap.insert("seed", seed_rngs);
        }
        if re_map.is_match(l) {
            src = re_map.captures(l).unwrap().name("src").unwrap().as_str();
            dst = re_map.captures(l).unwrap().name("dst").unwrap().as_str();
            prodmap.insert(dst, prodmap.get(src).clone().unwrap().clone());
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
            let (new_src, new_dst) = update_dst_vec(
                prodmap.get(src).unwrap(),
                prodmap.get(dst).unwrap(),
                src_start,
                dst_start,
                rng,
            );
            prodmap.insert(src, new_src);
            prodmap.insert(dst, new_dst);
        }
    }
    let location = prodmap
        .get(dst)
        .unwrap()
        .iter()
        .map(|x| x.start)
        .collect_vec();
    let min = location.iter().min().unwrap().clone();
    return min;
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
        assert_eq!(run(input.to_string()), 46);
    }
}
