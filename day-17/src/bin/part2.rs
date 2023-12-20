use glam::i32::IVec2;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, space0};
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::IResult;
use pathfinding::grid;
use pathfinding::prelude::astar;
use rayon::prelude::*;
use std::collections::{HashMap, VecDeque};
use std::fs;

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

fn move_one_into_dir(
    node: Node,
    dir: Directions,
    gridcost: &HashMap<IVec2, i32>,
) -> Option<(Node, i32)> {
    let node = Node {
        pos: node.pos
            + match dir {
                Directions::North => IVec2 { x: 0, y: -1 },
                Directions::East => IVec2 { x: 1, y: 0 },
                Directions::South => IVec2 { x: 0, y: 1 },
                Directions::West => IVec2 { x: -1, y: 0 },
            },
        last: {
            let mut new_last = node.last.clone();
            new_last.remove(10);
            new_last.push_front(dir);
            new_last
        },
    };
    if let Some(val) = gridcost.get(&node.pos) {
        Some((node, *val))
    } else {
        None
    }
}

fn move_four_into_dir(
    node: Node,
    dir: Directions,
    gridcost: &HashMap<IVec2, i32>,
) -> Option<(Node, i32)> {
    let mut node = node;
    let mut val = 0;
    for _i in 0..4 {
        if let Some(new_pos) = move_one_into_dir(node, dir, gridcost) {
            node = new_pos.0;
            val += new_pos.1;
        } else {
            return None;
        }
    }
    return Some((node, val));
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
                    if self.last.len() > 9
                        && self.last.iter().take(10).collect_vec() == vec![&Directions::North; 10]
                    {
                        if let Some(node_val) =
                            move_four_into_dir(self.clone(), Directions::East, gridcost)
                        {
                            sucessors.push(node_val)
                        }
                        if let Some(node_val) =
                            move_four_into_dir(self.clone(), Directions::West, gridcost)
                        {
                            sucessors.push(node_val)
                        }
                    } else {
                        if let Some(node_val) =
                            move_one_into_dir(self.clone(), Directions::North, gridcost)
                        {
                            sucessors.push(node_val)
                        }
                        if let Some(node_val) =
                            move_four_into_dir(self.clone(), Directions::East, gridcost)
                        {
                            sucessors.push(node_val)
                        }
                        if let Some(node_val) =
                            move_four_into_dir(self.clone(), Directions::West, gridcost)
                        {
                            sucessors.push(node_val)
                        }
                    }
                }
                Directions::East => {
                    if self.last.len() > 9
                        && self.last.iter().take(10).collect_vec() == vec![&Directions::East; 10]
                    {
                        if let Some(node_val) =
                            move_four_into_dir(self.clone(), Directions::North, gridcost)
                        {
                            sucessors.push(node_val)
                        }
                        if let Some(node_val) =
                            move_four_into_dir(self.clone(), Directions::South, gridcost)
                        {
                            sucessors.push(node_val)
                        }
                    } else {
                        if let Some(node_val) =
                            move_one_into_dir(self.clone(), Directions::East, gridcost)
                        {
                            sucessors.push(node_val)
                        }
                        if let Some(node_val) =
                            move_four_into_dir(self.clone(), Directions::North, gridcost)
                        {
                            sucessors.push(node_val)
                        }
                        if let Some(node_val) =
                            move_four_into_dir(self.clone(), Directions::South, gridcost)
                        {
                            sucessors.push(node_val)
                        }
                    }
                }
                Directions::South => {
                    if self.last.len() > 9
                        && self.last.iter().take(10).collect_vec() == vec![&Directions::South; 10]
                    {
                        if let Some(node_val) =
                            move_four_into_dir(self.clone(), Directions::East, gridcost)
                        {
                            sucessors.push(node_val)
                        }
                        if let Some(node_val) =
                            move_four_into_dir(self.clone(), Directions::West, gridcost)
                        {
                            sucessors.push(node_val)
                        }
                    } else {
                        if let Some(node_val) =
                            move_one_into_dir(self.clone(), Directions::South, gridcost)
                        {
                            sucessors.push(node_val)
                        }
                        if let Some(node_val) =
                            move_four_into_dir(self.clone(), Directions::East, gridcost)
                        {
                            sucessors.push(node_val)
                        }
                        if let Some(node_val) =
                            move_four_into_dir(self.clone(), Directions::West, gridcost)
                        {
                            sucessors.push(node_val)
                        }
                    }
                }
                Directions::West => {
                    if self.last.len() > 9
                        && self.last.iter().take(10).collect_vec() == vec![&Directions::West; 10]
                    {
                        if let Some(node_val) =
                            move_four_into_dir(self.clone(), Directions::North, gridcost)
                        {
                            sucessors.push(node_val)
                        }
                        if let Some(node_val) =
                            move_four_into_dir(self.clone(), Directions::South, gridcost)
                        {
                            sucessors.push(node_val)
                        }
                    } else {
                        if let Some(node_val) =
                            move_one_into_dir(self.clone(), Directions::West, gridcost)
                        {
                            sucessors.push(node_val)
                        }
                        if let Some(node_val) =
                            move_four_into_dir(self.clone(), Directions::North, gridcost)
                        {
                            sucessors.push(node_val)
                        }
                        if let Some(node_val) =
                            move_four_into_dir(self.clone(), Directions::South, gridcost)
                        {
                            sucessors.push(node_val)
                        }
                    }
                }
            },
            None => {
                if let Some(node_val) =
                    move_four_into_dir(self.clone(), Directions::North, gridcost)
                {
                    sucessors.push(node_val)
                }
                if let Some(node_val) = move_four_into_dir(self.clone(), Directions::East, gridcost)
                {
                    sucessors.push(node_val)
                }
                if let Some(node_val) =
                    move_four_into_dir(self.clone(), Directions::South, gridcost)
                {
                    sucessors.push(node_val)
                }
                if let Some(node_val) = move_four_into_dir(self.clone(), Directions::West, gridcost)
                {
                    sucessors.push(node_val)
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
    let cost_min = cost_map
        .clone()
        .into_iter()
        .map(|(pos, _)| (pos, (max_x - pos.x) + (max_y - pos.y)))
        .collect::<HashMap<IVec2, i32>>();
    let start = Node {
        pos: IVec2 { x: 0, y: 0 },
        last: VecDeque::with_capacity(10),
    };
    let goal = Node {
        pos: IVec2 { x: max_x, y: max_y },
        last: VecDeque::with_capacity(10),
    };
    let result = astar(
        &start,
        |p| p.successors(&cost_map),
        |p| p.distance(&goal),
        |p| p.pos == goal.pos,
    );
    dbg!(&result);
    result.unwrap().1
}

fn main() {
    let input: String = read_input();
    let sum = run(input);
    println!("{sum}")
    // 1168 Answer too low
    // 1439 Answer too high
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
        assert_eq!(run(input.to_string()), 94);
    }
}
