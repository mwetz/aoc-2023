use core::slice;
use glam::i32::IVec2;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1};
use nom::character::complete::{line_ending, space0};
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::IResult;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

#[derive(Eq, PartialEq, Clone, Debug, Hash, Copy)]
enum Directions {
    North,
    East,
    South,
    West,
}

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
struct Node {
    pos: IVec2,
    total: u32,
    last: VecDeque<Directions>,
}

fn read_input() -> String {
    fs::read_to_string("src/bin/input.txt").expect("Expected to read the file")
}

fn parse_line(input: &str) -> IResult<&str, Vec<&str>> {
    preceded(
        space0,
        many1(alt((
            tag("0"),
            tag("1"),
            tag("2"),
            tag("3"),
            tag("4"),
            tag("5"),
            tag("6"),
            tag("7"),
            tag("8"),
            tag("9"),
        ))),
    )(input)
}

fn get_input(input: &str) -> IResult<&str, Vec<Vec<&str>>> {
    separated_list1(line_ending, parse_line)(input)
}

fn get_grid(input: String) -> HashMap<IVec2, u8> {
    let grid = get_input(input.as_str())
        .expect("Expected to parse input")
        .1;
    let mut map: HashMap<IVec2, u8> = HashMap::new();
    grid.iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, cost)| {
                    map.insert(
                        IVec2::new(x as i32, y as i32),
                        cost.parse().expect("Number expected"),
                    )
                })
                .collect_vec()
        })
        .collect_vec();
    map
}

fn traverse_grid(
    start: Node,
    grid: &HashMap<IVec2, u8>,
    cost_exp: &HashMap<IVec2, u32>,
    max_x: u32,
    max_y: u32,
) -> Vec<Node> {
    let mut max_total: HashMap<(IVec2, VecDeque<Directions>), u32> = HashMap::new();
    let mut best_candidates = VecDeque::new();
    best_candidates.push_front(start);
    let mut searched_candidates = VecDeque::new();
    let mut complete_paths = Vec::new();
    while complete_paths.len() == 0 {
        let best_cand = best_candidates.pop_front().unwrap();
        let mut active_paths = Vec::new();
        searched_candidates.push_back(best_cand.clone());
        active_paths.push(best_cand);

        let mut next_paths = vec![];
        active_paths
            .iter()
            .map(|path| {
                if path.last.len() == 0
                    || (path.last.len() > 0
                        && path.last.len() < 3
                        && path.last.get(0).unwrap() != &Directions::South)
                    || (path.last.len() > 2
                        && !path
                            .last
                            .iter()
                            .take(3)
                            .all(|dir| dir == &Directions::North)
                        && path.last.get(0).unwrap() != &Directions::South)
                {
                    let next_pos = path.pos + IVec2 { x: 0, y: -1 };
                    let grid_value = grid.get(&next_pos);
                    let mut new_last = path.last.clone();
                    if new_last.len() > 2 {
                        new_last.remove(2);
                    }
                    new_last.push_front(Directions::North);
                    match grid_value {
                        Some(&value) => {
                            let new_total = path.total + value as u32;
                            // let new_total_mod = path.total_mod + grid_mod.get(&next_pos).unwrap();
                            let max_cur_pos = max_total.get(&(next_pos, new_last.clone()));
                            match max_cur_pos {
                                Some(&max_pos_value) => {
                                    if new_total < max_pos_value {
                                        max_total
                                            .insert((next_pos, new_last.clone()), new_total);
                                    }
                                }
                                None => {
                                    max_total.insert((next_pos, new_last.clone()), new_total);
                                }
                            }
                            next_paths.push(Node {
                                pos: next_pos,
                                total: new_total,
                                last: new_last,
                            })
                        }
                        None => (),
                    }
                }
                if path.last.len() == 0
                    || (path.last.len() > 0
                        && path.last.len() < 3
                        && path.last.get(0).unwrap() != &Directions::West)
                    || (path.last.len() > 2
                        && !path.last.iter().take(3).all(|dir| dir == &Directions::East)
                        && path.last.get(0).unwrap() != &Directions::West)
                {
                    let next_pos = path.pos + IVec2 { x: 1, y: 0 };
                    let grid_value = grid.get(&next_pos);
                    let mut new_last = path.last.clone();
                    if new_last.len() > 2 {
                        new_last.remove(2);
                    }
                    new_last.push_front(Directions::East);
                    match grid_value {
                        Some(&value) => {
                            let new_total = path.total + value as u32;
                            // let new_total_mod = path.total_mod + grid_mod.get(&next_pos).unwrap();
                            let max_cur_pos = max_total.get(&(next_pos, new_last.clone()));
                            match max_cur_pos {
                                Some(&max_pos_value) => {
                                    if new_total < max_pos_value {
                                        max_total
                                            .insert((next_pos, new_last.clone()), new_total);
                                    }
                                }
                                None => {
                                    max_total.insert((next_pos, new_last.clone()), new_total);
                                }
                            }
                            next_paths.push(Node {
                                pos: next_pos,
                                total: new_total,
                                last: new_last,
                            })
                        }
                        None => (),
                    }
                }
                if path.last.len() == 0
                    || (path.last.len() > 0
                        && path.last.len() < 3
                        && path.last.get(0).unwrap() != &Directions::North)
                    || (path.last.len() > 2
                        && !path
                            .last
                            .iter()
                            .take(3)
                            .all(|dir| dir == &Directions::South)
                        && path.last.get(0).unwrap() != &Directions::North)
                {
                    let next_pos = path.pos + IVec2 { x: 0, y: 1 };
                    let grid_value = grid.get(&next_pos);
                    let mut new_last = path.last.clone();
                    if new_last.len() > 2 {
                        new_last.remove(2);
                    }
                    new_last.push_front(Directions::South);
                    match grid_value {
                        Some(&value) => {
                            let new_total = path.total + value as u32;
                            // let new_total_mod = path.total_mod + grid_mod.get(&next_pos).unwrap();
                            let max_cur_pos = max_total.get(&(next_pos, new_last.clone()));
                            match max_cur_pos {
                                Some(&max_pos_value) => {
                                    if new_total < max_pos_value {
                                        max_total
                                            .insert((next_pos, new_last.clone()), new_total);
                                    }
                                }
                                None => {
                                    max_total.insert((next_pos, new_last.clone()), new_total);
                                }
                            }
                            next_paths.push(Node {
                                pos: next_pos,
                                total: new_total,
                                last: new_last,
                            })
                        }
                        None => (),
                    }
                }
                if path.last.len() == 0
                    || (path.last.len() > 0
                        && path.last.len() < 3
                        && path.last.get(0).unwrap() != &Directions::East)
                    || (path.last.len() > 2
                        && !path.last.iter().take(3).all(|dir| dir == &Directions::West)
                        && path.last.get(0).unwrap() != &Directions::East)
                {
                    let next_pos = path.pos + IVec2 { x: -1, y: 0 };
                    let grid_value = grid.get(&next_pos);
                    let mut new_last = path.last.clone();
                    if new_last.len() > 2 {
                        new_last.remove(2);
                    }
                    new_last.push_front(Directions::West);
                    match grid_value {
                        Some(&value) => {
                            let new_total = path.total + value as u32;
                            // let new_total_mod = path.total_mod + grid_mod.get(&next_pos).unwrap();
                            let max_cur_pos = max_total.get(&(next_pos, new_last.clone()));
                            match max_cur_pos {
                                Some(&max_pos_value) => {
                                    if new_total < max_pos_value {
                                        max_total
                                            .insert((next_pos, new_last.clone()), new_total);
                                    }
                                }
                                None => {
                                    max_total.insert((next_pos, new_last.clone()), new_total);
                                }
                            }
                            next_paths.push(Node {
                                pos: next_pos,
                                total: new_total,
                                last: new_last,
                            })
                        }
                        None => (),
                    }
                }
            })
            .collect_vec();

        next_paths
            .iter()
            .filter(|path| {
                path.pos
                    == (IVec2 {
                        x: max_x as i32,
                        y: max_y as i32,
                    })
            })
            .map(|path| complete_paths.push(path.clone()))
            .collect_vec();

        best_candidates = max_total
            .iter()
            .sorted_by(|((pos_a, _), val_a), ((pos_b, _), val_b)| {
                Ord::cmp(
                    &(*val_a + cost_exp.get(pos_a).unwrap()),
                    &(*val_b + cost_exp.get(pos_b).unwrap()),
                )
            })
            .map(|((pos, last), &val)| Node {
                pos: *pos,
                total: val,
                last: last.clone(),
            })
            .filter(|cand| !searched_candidates.contains(cand))
            .collect();
    }
    complete_paths
}

fn run(input: String) -> u32 {
    let gridmap = get_grid(input);
    let max_x = gridmap.keys().map(|grid| grid.x).max().unwrap() as u32;
    let max_y = gridmap.keys().map(|grid| grid.y).max().unwrap() as u32;
    let cost_exp = gridmap
        .clone()
        .into_iter()
        .map(|(pos, val)| (pos, (max_x - pos.x as u32) + (max_y - pos.y as u32)))
        .collect::<HashMap<IVec2, u32>>();
    let start = Node {
        pos: IVec2 { x: 0, y: 0 },
        total: 0,
        last: VecDeque::new(),
    };
    let paths = traverse_grid(start, &gridmap, &cost_exp, max_x, max_y);
    paths
        .iter()
        .map(|p| p.total)
        .min()
        .expect("At least one shortest path expected")
}

fn main() {
    let input: String = read_input();
    let sum = run(input);
    println!("{sum}")
    // 1031 Answer too low (high?)
    // 1035 Answer too high
    // 1271 Answer too high
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test() {
        let input: &'static str = r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        assert_eq!(run(input.to_string()), 102);
    }
}
