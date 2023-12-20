use glam::i32::IVec2;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, space0};
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::IResult;
use pathfinding::prelude::astar;
use rayon::prelude::*;
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::iter::Successors;

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

fn get_grid(input: String) -> HashMap<IVec2, i32> {
    let grid = get_input(input.as_str())
        .expect("Expected to parse input")
        .1;
    let mut map: HashMap<IVec2, i32> = HashMap::new();
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
    last: VecDeque<Directions>,
}

fn move_into_dir(node: Node, dir: Directions) -> Node {
    return Node {
        pos: node.pos
            + match dir {
                Directions::North => IVec2 { x: 0, y: -1 },
                Directions::East => IVec2 { x: 1, y: 0 },
                Directions::South => IVec2 { x: 0, y: 1 },
                Directions::West => IVec2 { x: -1, y: 0 },
            },
        last: {
            let mut new_last = node.last.clone();
            new_last.remove(2);
            new_last.push_front(dir);
            new_last
        },
    };
}

impl Node {
    fn distance(&self, other: &Node) -> i32 {
        (self.pos.x.abs_diff(other.pos.x) + self.pos.y.abs_diff(other.pos.y)) as i32
    }

    fn successors(&self, gridcost: &HashMap<IVec2, i32>) -> Vec<(Node, i32)> {
        let mut sucessors = Vec::new();
        match self.last.get(0) {
            Some(last) => match last {
                Directions::North => {
                    let new = move_into_dir(self.clone(), Directions::East);
                    if let Some(val) = gridcost.get(&new.pos) {
                        sucessors.push((new.clone(), *val));
                    }
                    let new = move_into_dir(self.clone(), Directions::West);
                    if let Some(val) = gridcost.get(&new.pos) {
                        sucessors.push((new.clone(), *val));
                    }
                    if self.last != vec![Directions::North, Directions::North, Directions::North] {
                        let new = move_into_dir(self.clone(), Directions::North);
                        if let Some(val) = gridcost.get(&new.pos) {
                            sucessors.push((new.clone(), *val));
                        }
                    }
                }
                Directions::East => {
                    let new = move_into_dir(self.clone(), Directions::North);
                    if let Some(val) = gridcost.get(&new.pos) {
                        sucessors.push((new.clone(), *val));
                    }
                    let new = move_into_dir(self.clone(), Directions::South);
                    if let Some(val) = gridcost.get(&new.pos) {
                        sucessors.push((new.clone(), *val));
                    }
                    if self.last != vec![Directions::East, Directions::East, Directions::East] {
                        let new = move_into_dir(self.clone(), Directions::East);
                        if let Some(val) = gridcost.get(&new.pos) {
                            sucessors.push((new.clone(), *val));
                        }
                    }
                }
                Directions::South => {
                    let new = move_into_dir(self.clone(), Directions::East);
                    if let Some(val) = gridcost.get(&new.pos) {
                        sucessors.push((new.clone(), *val));
                    }
                    let new = move_into_dir(self.clone(), Directions::West);
                    if let Some(val) = gridcost.get(&new.pos) {
                        sucessors.push((new.clone(), *val));
                    }
                    if self.last != vec![Directions::South, Directions::South, Directions::South] {
                        let new = move_into_dir(self.clone(), Directions::South);
                        if let Some(val) = gridcost.get(&new.pos) {
                            sucessors.push((new.clone(), *val));
                        }
                    }
                }
                Directions::West => {
                    let new = move_into_dir(self.clone(), Directions::North);
                    if let Some(val) = gridcost.get(&new.pos) {
                        sucessors.push((new.clone(), *val));
                    }
                    let new = move_into_dir(self.clone(), Directions::South);
                    if let Some(val) = gridcost.get(&new.pos) {
                        sucessors.push((new.clone(), *val));
                    }
                    if self.last != vec![Directions::West, Directions::West, Directions::West] {
                        let new = move_into_dir(self.clone(), Directions::West);
                        if let Some(val) = gridcost.get(&new.pos) {
                            sucessors.push((new.clone(), *val));
                        }
                    }
                }
            },
            None => {
                let new = move_into_dir(self.clone(), Directions::North);
                if let Some(val) = gridcost.get(&new.pos) {
                    sucessors.push((new.clone(), *val));
                }
                let new = move_into_dir(self.clone(), Directions::East);
                if let Some(val) = gridcost.get(&new.pos) {
                    sucessors.push((new.clone(), *val));
                }
                let new = move_into_dir(self.clone(), Directions::South);
                if let Some(val) = gridcost.get(&new.pos) {
                    sucessors.push((new.clone(), *val));
                }
                let new = move_into_dir(self.clone(), Directions::West);
                if let Some(val) = gridcost.get(&new.pos) {
                    sucessors.push((new.clone(), *val));
                }
            }
        }
        sucessors
    }
}

fn run(input: String) -> i32 {
    let cost_map = get_grid(input);
    let max_x = cost_map.keys().map(|grid| grid.x).max().unwrap() as i32;
    let max_y = cost_map.keys().map(|grid| grid.y).max().unwrap() as i32;
    let start = Node {
        pos: IVec2 { x: 0, y: 0 },
        last: VecDeque::with_capacity(3),
    };
    let goal = Node {
        pos: IVec2 { x: max_x, y: max_y },
        last: VecDeque::with_capacity(3),
    };
    let result = astar(
        &start,
        |p| p.successors(&cost_map),
        |p| p.distance(&goal),
        |p| p.pos == goal.pos,
    );
    result.unwrap().1
}

fn main() {
    let input: String = read_input();
    let sum = run(input);
    println!("{sum}")
    // 1031 Answer too high
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
