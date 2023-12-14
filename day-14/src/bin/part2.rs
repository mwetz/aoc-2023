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
    max_x: u32,
}

impl Grid {
    fn get_point(&self, x: usize, y: usize) -> Option<&Point> {
        self.points.iter().find(|p| p.x == x && p.y == y)
    }

    fn display_grid(&self) -> String {
        let mut display = String::from("");
        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
                let point = self.get_point(x as usize, y as usize);
                match point {
                    Some(tile) => match tile.tile {
                        Tile::Round => display.push('O'),
                        Tile::Square => display.push('#'),
                        Tile::Empty => display.push('.'),
                    },
                    None => display.push('.'),
                }
            }
            display.push('\n')
        }
        display
    }
}

fn get_grid(input: String) -> Grid {
    let mut gridpoints = Vec::new();
    let mut max_y = 0;
    let mut max_x = 0;
    for (y, l) in input.lines().enumerate() {
        max_y = y as u32;
        for (x, i) in l.chars().enumerate() {
            max_x = x as u32;
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
    Grid {
        points: gridpoints,
        max_y: max_y,
        max_x: max_x,
    }
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
    Grid {
        points: points_new,
        max_y: platform.max_y,
        max_x: platform.max_x,
    }
}

fn move_south(platform: Grid, point: Point) -> Grid {
    let path = platform
        .points
        .iter()
        .filter(|p| p.y > point.y && p.x == point.x)
        .map(|p| p.y)
        .min();
    let new_y = if path.is_some() {
        path.unwrap() - 1
    } else {
        platform.max_y as usize
    };
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
    Grid {
        points: points_new,
        max_y: platform.max_y,
        max_x: platform.max_x,
    }
}

fn move_west(platform: Grid, point: Point) -> Grid {
    let path = platform
            .points
            .iter()
            .filter(|p| p.y == point.y && p.x < point.x).collect_vec();
    let obstacle = path.into_iter().map(|p| p.x).max();
    let new_x = if obstacle.is_some() { obstacle.unwrap() + 1 } else { 0 };
    let mut points_new = platform
        .clone()
        .points
        .into_iter()
        .filter(|p| p.x != point.x || p.y != point.y)
        .collect_vec();
    points_new.push(Point {
        x: new_x,
        y: point.y,
        tile: Tile::Round,
    });
    Grid {
        points: points_new,
        max_y: platform.max_y,
        max_x: platform.max_x,
    }
}

fn move_east(platform: Grid, point: Point) -> Grid {
    let path = platform
        .points
        .iter()
        .filter(|p| p.y == point.y && p.x > point.x)
        .map(|p| p.x)
        .min();
    let new_x = if path.is_some() {
        path.unwrap() - 1
    } else {
        platform.max_x as usize
    };
    let mut points_new = platform
        .clone()
        .points
        .into_iter()
        .filter(|p| p.x != point.x || p.y != point.y)
        .collect_vec();
    points_new.push(Point {
        x: new_x,
        y: point.y,
        tile: Tile::Round,
    });
    Grid {
        points: points_new,
        max_y: platform.max_y,
        max_x: platform.max_x,
    }
}

fn run_cycle(platform: Grid) -> Grid {
    let mut platform_new = platform.clone();
    for p in platform_new
        .clone()
        .points
        .into_iter()
        .sorted_by_key(|p| (p.y as i32))
        .filter(|x| x.tile == Tile::Round)
    {
        platform_new = move_north(platform_new, p);
    }
    // println!("After tilting north");
    // println!("{}", platform_new.display_grid().as_str());
    for p in platform_new
            .clone()
            .points
            .into_iter()
            .sorted_by_key(|p| (p.x as i32))
            .filter(|x| x.tile == Tile::Round)
    {
        platform_new = move_west(platform_new, p);
    }
    // println!("After tilting west");
    // println!("{}", platform_new.display_grid().as_str());
    for p in platform_new
        .clone()
        .points
        .into_iter()
        .sorted_by_key(|p| -(p.y as i32))
        .filter(|x| x.tile == Tile::Round)
    {
        platform_new = move_south(platform_new, p);
    }
    // println!("After tilting south");
    // println!("{}", platform_new.display_grid().as_str());
    for p in platform_new
        .clone()
        .points
        .into_iter()
        .sorted_by_key(|p| -(p.x as i32))
        .filter(|x| x.tile == Tile::Round)
    {
        platform_new = move_east(platform_new, p);
    }
    // println!("After tilting east");
    // println!("{}", platform_new.display_grid().as_str());
    platform_new
}


fn run(input: String) -> u32 {
    let mut platform = get_grid(input);
    let mut load = 0;
    // println!("Initial platform");
    // println!("{}", platform.display_grid().as_str());
    let mut points_ref: Vec<Point> = Vec::new();
    let mut load_sequence: Vec<u32> = Vec::new();
    for i in 1..10_000 {
        if i % 10 == 0 {
            println!("Running cycle {}", i)
        }
        platform = run_cycle(platform);
        load = platform
            .points
            .iter()
            .filter(|x| x.tile == Tile::Round)
            .map(|x| (platform.max_y + 1) - x.y as u32)
            .sum::<u32>();
        if i >= 200 {
            if platform.points == points_ref {
                break;
            }
            load_sequence.push(load);
            if i == 200 {
                points_ref = platform.points.clone();
            }
        }
        // println!("Load {} after {} cycles", load, i + 1);
        // println!("{}", platform.display_grid().as_str());
    }
    let cyclepos = (1_000_000_000 - 200) % load_sequence.len();
    // dbg!(&load_sequence, &cyclepos);
    load_sequence[cyclepos]
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
        assert_eq!(run(input.to_string()), 64);
    }
}
