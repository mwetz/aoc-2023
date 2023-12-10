use itertools::Itertools;
use std::fs;

fn read_input() -> String {
    let input: String = fs::read_to_string("src/bin/input.txt").expect("Expected to read the file");
    input
}

#[derive(Debug)]
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

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    pipe: Pipe,
}

impl Point {
    fn get_connected<'a>(&'a self, from: &Point, grid: &'a Grid) -> Option<&Point> {
        match self.pipe {
            Pipe::Start => {
                let connected = grid.get_point(self.x, self.y + -1);
                if connected.is_some() {
                    return connected;
                }
                let connected = grid.get_point(self.x - 1 , self.y);
                if connected.is_some() {
                    return connected;
                }
                let connected = grid.get_point(self.x, self.y + 1);
                if connected.is_some() {
                    return connected;
                }
                let connected = grid.get_point(self.x + 1, self.y);
                if connected.is_some() {
                    return connected;
                }
                return None;
            }
            Pipe::WE => {
                if self.x > from.x {
                    return grid.get_point(self.x + 1, self.y);
                } else {
                    return grid.get_point(self.x - 1, self.y);
                }
            }
            Pipe::NS => {
                if self.y > from.y {
                    return grid.get_point(self.x, self.y + 1);
                } else {
                    return grid.get_point(self.x, self.y - 1);
                }
            }
            Pipe::NE => {
                if self.y > from.y {
                    return grid.get_point(self.x + 1, self.y);
                } else {
                    return grid.get_point(self.x, self.y - 1);
                }
            }
            Pipe::SE => {
                if self.y < from.y {
                    return grid.get_point(self.x + 1, self.y);
                } else {
                    return grid.get_point(self.x, self.y + 1);
                }
            }
            Pipe::SW => {
                if self.y < from.y {
                    return grid.get_point(self.x - 1, self.y);
                } else {
                    return grid.get_point(self.x, self.y + 1);
                }
            }
            Pipe::NW => {
                if self.y > from.y {
                    return grid.get_point(self.x - 1, self.y);
                } else {
                    return grid.get_point(self.x, self.y - 1);
                }
            }
            _ => None,
        }
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

fn traverse_pipe(grid: &Grid) -> u32 {
    let mut steps = 0;
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
    for i in 0..100_000_000 {
        steps += 1;
        let next = to.get_connected(from, grid).unwrap();
        from = to;
        to = next;
        match to.pipe {
            Pipe::Start => return steps / 2,
            _ => {}
        }
    }
    0
}

fn run(input: String) -> u32 {
    let grid = get_grid(input);
    traverse_pipe(&grid)
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
        let input: &'static str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!(run(input.to_string()), 8);
    }
}
