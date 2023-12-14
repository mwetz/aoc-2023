use itertools::Itertools;
use std::collections::HashSet;
use std::{fs, iter, vec};

fn read_input() -> String {
    let input: String = fs::read_to_string("src/bin/input.txt").expect("Expected to read the file");
    input
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Tile {
    Round,
    Square,
    Empty,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Point {
    x: usize,
    y: usize,
    tile: Tile,
}

#[derive(Debug, Clone)]
struct Grid {
    points: Vec<Point>,
    max_y: u32,
}

impl Grid {
    fn get_point(&self, x: usize, y: usize) -> Option<&Point> {
        self.points.iter().find(|p| p.x == x && p.y == y)
    }
}

fn get_grid(input: String) -> Grid {
    let mut gridpoints = Vec::new();
    let mut max_y = 0;
    for (y, l) in input.lines().enumerate() {
        max_y = y as u32;
        for (x, i) in l.chars().enumerate() {
            match i {
                '.' => {}
                _ => gridpoints.push(Point {
                    x: x,
                    y: y,
                    tile: match i {
                        '#' => Tile::Square,
                        'O' => Tile::Round,
                        _ => Tile::Empty,
                    },
                }),
            }
        }
    }
    Grid { points: gridpoints, max_y: max_y }
}

fn move_north(platform: Grid, point: Point) -> Grid {
    let path = platform
        .points
        .iter()
        .filter(|p| p.y < point.y && p.x == point.x)
        .map(|p| p.y)
        .max();
    let new_y = if path.is_some() { path.unwrap() + 1 } else { 0 };
    let mut points_new = platform
        .clone()
        .points
        .into_iter()
        .filter(|p| p.x != point.x || p.y != point.y)
        .collect_vec();
    points_new.push(Point {
        x: point.x,
        y: new_y,
        tile: Tile::Round,
    });
    Grid { points: points_new, max_y: platform.max_y }
}

fn run(input: String) -> u32 {
    let mut platform = get_grid(input);
    for p in platform
        .clone()
        .points
        .into_iter()
        .sorted_by_key(|p| (p.y as i32))
        .filter(|x| x.tile == Tile::Round)
    {
        platform = move_north(platform, p);
    }
    platform.points.iter().filter(|x| x.tile == Tile::Round).map(|x| platform.max_y + 1 - x.y as u32).sum::<u32>()
    
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
        let input: &'static str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!(run(input.to_string()), 136);
    }
}
