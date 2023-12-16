use glam::i32::IVec2;
use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete::char;
use nom::character::complete::{line_ending, space0};
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::IResult;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fs;

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

#[derive(Debug, Eq, PartialEq, Clone, Hash, Copy)]
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

impl Direction {
    fn step(&self, position: &IVec2) -> IVec2 {
        match self {
            Direction::North => *position + IVec2::new(0, -1),
            Direction::East => *position + IVec2::new(1, 0),
            Direction::South => *position + IVec2::new(0, 1),
            Direction::West => *position + IVec2::new(-1, 0),
        }
    }
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

fn trace_beam(map: &HashMap<IVec2, Tile>, starting_beam: Beam) -> HashSet<Beam> {
    let mut active_beams = Vec::new();
    let mut path: HashSet<Beam> = HashSet::new();
    active_beams.push(starting_beam);
    while active_beams.len() > 0 {
        let beams_next = active_beams
            .iter()
            .map(|beam| Beam {
                pos: beam.dir.step(&beam.pos),
                dir: beam.dir,
            })
            .filter(|beam| !path.contains(beam))
            .collect_vec();

        let mut new_beams = vec![];
        beams_next
            .into_iter()
            .map(|beam| {
                // Check new tile
                match map.get(&beam.pos) {
                    Some(tile) => match tile {
                        Tile::MirrorNE => match beam.dir {
                            Direction::North => new_beams.push(Beam {
                                pos: beam.pos,
                                dir: Direction::East,
                            }),
                            Direction::East => new_beams.push(Beam {
                                pos: beam.pos,
                                dir: Direction::North,
                            }),
                            Direction::South => new_beams.push(Beam {
                                pos: beam.pos,
                                dir: Direction::West,
                            }),
                            Direction::West => new_beams.push(Beam {
                                pos: beam.pos,
                                dir: Direction::South,
                            }),
                        },
                        Tile::MirrorNW => match beam.dir {
                            Direction::North => new_beams.push(Beam {
                                pos: beam.pos,
                                dir: Direction::West,
                            }),
                            Direction::East => new_beams.push(Beam {
                                pos: beam.pos,
                                dir: Direction::South,
                            }),
                            Direction::South => new_beams.push(Beam {
                                pos: beam.pos,
                                dir: Direction::East,
                            }),
                            Direction::West => new_beams.push(Beam {
                                pos: beam.pos,
                                dir: Direction::North,
                            }),
                        },
                        Tile::SplitterNS => match beam.dir {
                            Direction::North | Direction::South => new_beams.push(Beam {
                                pos: beam.pos,
                                dir: beam.dir,
                            }),
                            Direction::East | Direction::West => {
                                new_beams.push(Beam {
                                    pos: beam.pos,
                                    dir: Direction::North,
                                });
                                new_beams.push(Beam {
                                    pos: beam.pos,
                                    dir: Direction::South,
                                });
                            }
                        },
                        Tile::SplitterWE => match beam.dir {
                            Direction::East | Direction::West => new_beams.push(Beam {
                                pos: beam.pos,
                                dir: beam.dir,
                            }),
                            Direction::North | Direction::South => {
                                new_beams.push(Beam {
                                    pos: beam.pos,
                                    dir: Direction::East,
                                });
                                new_beams.push(Beam {
                                    pos: beam.pos,
                                    dir: Direction::West,
                                });
                            }
                        },
                        Tile::Empty => new_beams.push(Beam {
                            pos: beam.pos,
                            dir: beam.dir,
                        }),
                    },
                    // None means we are out of bounds
                    None => (),
                }
            })
            .collect_vec();
        path.extend(new_beams.clone());
        active_beams = new_beams;
    }
    path
}

fn run(input: String) -> usize {
    let gridmap = get_grid(input);
    let starting_beams = vec![Beam {
        pos: IVec2 { x: -1, y: 0 },
        dir: Direction::East,
    }];
    starting_beams
        .into_iter()
        .map(|beam| {
            trace_beam(&gridmap, beam)
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
        assert_eq!(run(input.to_string()), 46);
    }
}
