use itertools::Itertools;
use std::collections::HashSet;
use std::{fs, iter, vec};

fn read_input() -> String {
    let input: String = fs::read_to_string("src/bin/input.txt").expect("Expected to read the file");
    input
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Pipe {
    Start,
    WE,
    NS,
    NE,
    SE,
    SW,
    NW,
    Empty,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
    pipe: Pipe,
}

impl Point {
    fn get_connected<'a>(
        &'a self,
        from: &Point,
        grid: &'a Grid,
    ) -> (Option<&Point>, Vec<Option<&Point>>) {
        match self.pipe {
            Pipe::Start => {
                let connected = grid.get_point(self.x, self.y + 1);
                if connected.is_some() {
                    match connected.unwrap().pipe {
                        Pipe::NS | Pipe::NE | Pipe::NW => return (connected, Vec::new()),
                        _ => {}
                    }
                }
                let connected = grid.get_point(self.x + 1, self.y);
                if let Some(..) = connected {
                    match connected.unwrap().pipe {
                        Pipe::NW | Pipe::SW | Pipe::WE => return (connected, Vec::new()),
                        _ => {}
                    }
                    let connected = grid.get_point(self.x, self.y - 1);
                    if connected.is_some() {
                        match connected.unwrap().pipe {
                            Pipe::SE | Pipe::SW | Pipe::NS => return (connected, Vec::new()),
                            _ => {}
                        }
                    }
                    let connected = grid.get_point(self.x - 1, self.y);
                    if connected.is_some() {
                        match connected.unwrap().pipe {
                            Pipe::WE | Pipe::NE | Pipe::SE => return (connected, Vec::new()),
                            _ => {}
                        }
                    }
                }
                return (None, Vec::new());
            }
            Pipe::WE => {
                if self.x > from.x {
                    return (
                        grid.get_point(self.x + 1, self.y),
                        vec![grid.get_point(self.x, self.y + 1)],
                    );
                } else {
                    return (
                        grid.get_point(self.x - 1, self.y),
                        vec![grid.get_point(self.x, self.y - 1)],
                    );
                }
            }
            Pipe::NS => {
                if self.y > from.y {
                    return (
                        grid.get_point(self.x, self.y + 1),
                        vec![grid.get_point(self.x - 1, self.y)],
                    );
                } else {
                    return (
                        grid.get_point(self.x, self.y - 1),
                        vec![grid.get_point(self.x + 1, self.y)],
                    );
                }
            }
            Pipe::NE => {
                if self.y > from.y {
                    return (
                        grid.get_point(self.x + 1, self.y),
                        vec![
                            grid.get_point(self.x - 1, self.y),
                            grid.get_point(self.x, self.y + 1),
                        ],
                    );
                } else {
                    return (grid.get_point(self.x, self.y - 1), Vec::new());
                }
            }
            Pipe::SE => {
                if self.y < from.y {
                    return (grid.get_point(self.x + 1, self.y), Vec::new());
                } else {
                    return (
                        grid.get_point(self.x, self.y + 1),
                        vec![
                            grid.get_point(self.x - 1, self.y),
                            grid.get_point(self.x, self.y - 1),
                        ],
                    );
                }
            }
            Pipe::SW => {
                if self.y < from.y {
                    return (
                        grid.get_point(self.x - 1, self.y),
                        vec![
                            grid.get_point(self.x + 1, self.y),
                            grid.get_point(self.x, self.y - 1),
                        ],
                    );
                } else {
                    return (grid.get_point(self.x, self.y + 1), Vec::new());
                }
            }
            Pipe::NW => {
                if self.y > from.y {
                    return (grid.get_point(self.x - 1, self.y), Vec::new());
                } else {
                    return (
                        grid.get_point(self.x, self.y - 1),
                        vec![
                            grid.get_point(self.x + 1, self.y),
                            grid.get_point(self.x, self.y + 1),
                        ],
                    );
                }
            }
            _ => (None, Vec::new()),
        }
    }

    fn get_adjacent<'a>(self: &'a Point, grid: &'a Grid) -> Vec<&Point> {
        vec![
            grid.get_point(self.x, self.y),
            grid.get_point(self.x + 1, self.y),
            grid.get_point(self.x - 1, self.y),
            grid.get_point(self.x, self.y + 1),
            grid.get_point(self.x, self.y - 1),
        ]
        .iter()
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect_vec()
    }
}

#[derive(Debug)]
struct Grid {
    points: Vec<Point>,
}

impl Grid {
    fn get_point(&self, x: i32, y: i32) -> Option<&Point> {
        self.points.iter().find(|p| p.x == x && p.y == y)
    }
}

fn get_grid(input: String) -> (Grid) {
    let mut gridpoints = Vec::new();
    for (y, l) in input.lines().enumerate() {
        for (x, i) in l.chars().enumerate() {
            // Get grid information
            gridpoints.push(Point {
                x: x as i32,
                y: y as i32,
                pipe: match i {
                    'S' => Pipe::Start,
                    '|' => Pipe::NS,
                    '-' => Pipe::WE,
                    'L' => Pipe::NE,
                    'J' => Pipe::NW,
                    '7' => Pipe::SW,
                    'F' => Pipe::SE,
                    _ => Pipe::Empty,
                },
            })
        }
    }
    Grid { points: gridpoints }
}

fn traverse_pipe(grid: &Grid) -> (Vec<&Point>, Vec<&Point>) {
    let mut steps: Vec<&Point> = Vec::new();
    let start = grid
        .points
        .iter()
        .filter(|&x| match x.pipe {
            Pipe::Start => true,
            _ => false,
        })
        .collect_vec();
    let mut from = start[0];
    let mut to = start[0];
    let mut right_all: Vec<&Point> = Vec::new();
    for i in 0..1_000_000 {
        let (next, right) = to.get_connected(from, grid);
        println!("{:?}", next.unwrap().pipe);
        steps.push(next.unwrap());
        right_all.extend(right.iter().filter(|x| x.is_some()).map(|x| x.unwrap()));
        from = to;
        to = next.expect("Expected next to be a point");
        if let Pipe::Start = to.pipe {
            println!("Finished loop!");
            return (steps, right_all);
        }
    }
    (Vec::new(), Vec::new())
}

fn run(input: String) -> u32 {
    let grid = get_grid(input);
    let (path, inner) = traverse_pipe(&grid);
    let mut inner_extend = inner;
    let mut count_last = 0;
    for i in 0..100 {
        let inner_new = inner_extend
            .into_iter()
            .unique()
            .filter(|x| !&path.contains(x))
            .collect_vec();
        let count_new = inner_new.clone().into_iter().count() as u32;
        inner_extend = inner_new.into_iter().map(|x| x.get_adjacent(&grid)).flatten().collect_vec();
        if count_last == count_new {
            return count_new;
        }
        count_last = count_new;
    }
    0
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
        let input: &'static str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!(run(input.to_string()), 8);
    }
}
