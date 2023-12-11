use itertools::Itertools;
use std::collections::HashSet;
use std::{fs, iter, vec};

fn read_input() -> String {
    let input: String = fs::read_to_string("src/bin/input.txt").expect("Expected to read the file");
    input
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Tile {
    Galaxy,
    Empty,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
    tile: Tile,
}

impl Point {}

#[derive(Debug)]
struct Grid {
    points: Vec<Point>,
    max_x: usize,
    max_y: usize,
}

impl Grid {
    fn get_point(&self, x: usize, y: usize) -> Option<&Point> {
        self.points.iter().find(|p| p.x == x && p.y == y)
    }
}

fn get_grid(input: String) -> Grid {
    let mut gridpoints = Vec::new();
    let mut max_y = 0;
    let mut max_x = 0;
    for (y, l) in input.lines().enumerate() {
        max_y = y;
        for (x, i) in l.chars().enumerate() {
            max_x = x;
            gridpoints.push(Point {
                x: x,
                y: y,
                tile: match i {
                    '#' => Tile::Galaxy,
                    _ => Tile::Empty,
                },
            })
        }
    }
    Grid {
        points: gridpoints,
        max_x: max_x,
        max_y: max_y,
    }
}

fn extend_galaxy(grid: &Grid) -> Grid {
    // Extend in x direction
    let mut gridpoints_x: Vec<Point> = Vec::new();
    let mut offset_x: usize = 0;
    for x in 0..=grid.max_x {
        let x_slice = grid.points.iter().filter(|&p| p.x == x).collect_vec();
        if x_slice.clone().into_iter().all(|p| p.tile == Tile::Empty) {
            offset_x += 1
        }
        gridpoints_x.extend(x_slice.clone().into_iter().map(|p| Point {
            x: p.x + offset_x,
            y: p.y,
            tile: p.tile,
        }));
    }
    let grid_x = Grid {
        points: gridpoints_x,
        max_x: grid.max_x + offset_x,
        max_y: grid.max_y,
    };

    // Extend in y direction
    let mut gridpoints_y: Vec<Point> = Vec::new();
    let mut offset_y: usize = 0;
    for y in 0..=grid_x.max_y {
        let y_slice = grid_x.points.iter().filter(|&p| p.y == y).collect_vec();
        if y_slice.clone().into_iter().all(|p| p.tile == Tile::Empty) {
            offset_y += 1
        }
        gridpoints_y.extend(y_slice.clone().into_iter().map(|p| Point {
            x: p.x,
            y: p.y + offset_y,
            tile: p.tile,
        }));
    }
    Grid {
        points: gridpoints_y,
        max_x: grid_x.max_x,
        max_y: grid_x.max_y + offset_y,
    }
}

fn run(input: String) -> usize {
    let galaxy = get_grid(input);
    let ext_galaxy = extend_galaxy(&galaxy);
    let galaxies = ext_galaxy
        .points
        .into_iter()
        .filter(|p| p.tile == Tile::Galaxy)
        .collect_vec();
    galaxies
        .clone()
        .into_iter()
        .map(|a| {
            galaxies
                .clone()
                .into_iter()
                .map(move |b| a.x.abs_diff(b.x) + a.y.abs_diff(b.y)).sum::<usize>()
        })
        .sum::<usize>() / 2
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
        let input: &'static str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(run(input.to_string()), 374);
    }
}
