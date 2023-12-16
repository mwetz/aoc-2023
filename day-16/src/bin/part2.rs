use glam::i32::IVec2;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::{is_a, is_not};
use nom::character::complete::char;
use nom::character::complete::{line_ending, space0};
use nom::multi::many1;
use nom::sequence::{pair, preceded};
use nom::{bytes::complete::tag, multi::separated_list1, IResult, Parser};
use std::collections::{HashMap, HashSet};
use std::{fs, iter, vec};
use rayon::prelude::*;

fn read_input() -> String {
    fs::read_to_string("src/bin/input.txt").expect("Expected to read the file")
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Tile {
    MirrorNE,
    MirrorNW,
    SplitterNS,
    SplitterWE,
    Empty,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Point {
    x: usize,
    y: usize,
    tile: Tile,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Beam {
    pos: IVec2,
    dir: Direction,
}

fn parse_line(input: &str) -> IResult<&str, Vec<char>> {
    preceded(
        space0,
        many1(alt((
            char('\\'),
            char('/'),
            char('|'),
            char('-'),
            char('.'),
        ))),
    )(input)
}

fn get_input(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    separated_list1(line_ending, parse_line)(input)
}

fn get_grid(input: String) -> HashMap<IVec2, Tile> {
    let grid = get_input(input.as_str())
        .expect("Expected to parse input")
        .1;
    let mut map: HashMap<IVec2, Tile> = HashMap::new();
    grid.iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, tile)| {
                    map.insert(
                        IVec2::new(x as i32, y as i32),
                        match tile {
                            '/' => Tile::MirrorNE,
                            '\\' => Tile::MirrorNW,
                            '-' => Tile::SplitterWE,
                            '|' => Tile::SplitterNS,
                            _ => Tile::Empty,
                        },
                    )
                })
                .collect_vec()
        })
        .collect_vec();
    map
}

fn trace_beam(
    map: &HashMap<IVec2, Tile>,
    mut beam: Beam,
    mut path: HashSet<Beam>,
) -> HashSet<Beam> {
    for i in 0.. {
        // Make a move
        match beam.dir {
            Direction::North => beam.pos += IVec2::new(0, -1),
            Direction::East => beam.pos += IVec2::new(1, 0),
            Direction::South => beam.pos += IVec2::new(0, 1),
            Direction::West => beam.pos += IVec2::new(-1, 0),
        }

        // If We have been here before abort, else store beam
        if path.contains(&beam) {
            // println!("Hit previous position at {}", &beam.pos);
            return path;
        }

        // Check new tile
        match map.get(&beam.pos) {
            Some(tile) => {
                path.insert(beam.clone());
                match tile {
                    Tile::MirrorNE => match beam.dir {
                        Direction::North => beam.dir = Direction::East,
                        Direction::East => beam.dir = Direction::North,
                        Direction::South => beam.dir = Direction::West,
                        Direction::West => beam.dir = Direction::South,
                    },
                    Tile::MirrorNW => match beam.dir {
                        Direction::North => beam.dir = Direction::West,
                        Direction::East => beam.dir = Direction::South,
                        Direction::South => beam.dir = Direction::East,
                        Direction::West => beam.dir = Direction::North,
                    },
                    Tile::SplitterNS => match beam.dir {
                        Direction::North | Direction::South => (),
                        Direction::East | Direction::West => {
                            // println!("Sending ray north at {}", &beam.pos);
                            path.extend(trace_beam(
                                map,
                                Beam {
                                    pos: beam.pos,
                                    dir: Direction::North,
                                },
                                path.clone(),
                            ));
                            // println!("Sending ray south at {}", &beam.pos);
                            path.extend(trace_beam(
                                map,
                                Beam {
                                    pos: beam.pos,
                                    dir: Direction::South,
                                },
                                path.clone(),
                            ));
                            return path;
                        }
                    },
                    Tile::SplitterWE => match beam.dir {
                        Direction::East | Direction::West => (),
                        Direction::North | Direction::South => {
                            // println!("Sending ray west at {}", &beam.pos);
                            path.extend(trace_beam(
                                map,
                                Beam {
                                    pos: beam.pos,
                                    dir: Direction::West,
                                },
                                path.clone(),
                            ));
                            // println!("Sending ray east at {}", &beam.pos);
                            path.extend(trace_beam(
                                map,
                                Beam {
                                    pos: beam.pos,
                                    dir: Direction::East,
                                },
                                path.clone(),
                            ));
                            return path;
                        }
                    },
                    Tile::Empty => (),
                }
            }
            // None means we are out of bounds
            None => return path,
        }
    }
    path
}

fn run(input: String) -> usize {
    let gridmap = get_grid(input);
    let max_x = gridmap
        .iter()
        .map(|(pos, _)| pos.x)
        .max()
        .expect("Grid needs a max value in direction x");
    let max_y = gridmap
        .iter()
        .map(|(pos, _)| pos.y)
        .max()
        .expect("Grid needs a max value in direction y");
    let starting_beams_x = gridmap
        .iter()
        .filter(|(pos, _)| pos.x == 0 || pos.x == max_x)
        .map(|(pos, _)| match pos.x {
            0 => Beam {
                pos: IVec2::new(-1, pos.y),
                dir: Direction::East,
            },
            _ => Beam {
                pos: IVec2::new(max_x + 1, pos.y),
                dir: Direction::West,
            },
        })
        .collect_vec();
    let starting_beams_y = gridmap
        .iter()
        .filter(|(pos, _)| pos.y == 0 || pos.y == max_y)
        .map(|(pos, _)| match pos.y {
            0 => Beam {
                pos: IVec2::new(pos.x, -1),
                dir: Direction::South,
            },
            _ => Beam {
                pos: IVec2::new(pos.x, max_y + 1),
                dir: Direction::North,
            },
        })
        .collect_vec();
    let starting_beams = [starting_beams_x, starting_beams_y].concat();
    starting_beams
        .into_par_iter()
        .map(|beam| {
            trace_beam(&gridmap, beam, HashSet::new())
                .into_iter()
                .map(|beam| beam.pos)
                .collect::<HashSet<IVec2>>()
                .into_iter()
                .count()
        })
        .max()
        .expect("Maximum expected")
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
        let input: &'static str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        assert_eq!(run(input.to_string()), 51);
    }
}
