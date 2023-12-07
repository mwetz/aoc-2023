use std::fs;

fn read_input() -> String {
    let input: String = fs::read_to_string("src/bin/input.txt").expect("Expected to read the file");
    return input;
}

enum Element {
    Dot,
    Digit,
    Symbol,
}

struct Point {
    x: i32,
    y: i32,
    ele: Element,
}

struct Grid {
    points: Vec<Point>,
}

impl Grid {
    fn is_point_symbol(&self, x: i32, y: i32) -> bool {
        let point = self.points.iter().filter(|p| p.x == x && p.y == y).next();
        return match point {
            None => false,
            _ => match point.unwrap().ele {
                Element::Symbol => true,
                _ => false,
            },
        };
    }
}

struct Number {
    y: i32,
    xmin: i32,
    xmax: i32,
    value: u32,
}

impl Number {
    fn get_neighbours(&self) -> Vec<(i32, i32)> {
        let mut neighbours = Vec::new();
        for y in self.y - 1..=self.y + 1 {
            for x in self.xmin - 1..=self.xmax + 1 {
                if y != self.y || x < self.xmin || x > self.xmax {
                    neighbours.push((x, y));
                }
            }
        }
        return neighbours;
    }
    fn has_symbol(&self, grid: &Grid) -> bool {
        let neighbours = self.get_neighbours();
        neighbours
            .iter()
            .map(|n| grid.is_point_symbol(n.0, n.1))
            .any(|n| n)
    }
}

fn parse_input(input: String) -> (Grid, Vec<Number>) {
    let mut numbers = Vec::new();
    let mut gridpoints = Vec::new();
    for (y, l) in input.lines().enumerate() {
        let maxline: u32 = l.len() as u32;
        let mut number: Vec<char> = Vec::new();
        for (x, i) in l.chars().enumerate() {
            // Get grid information
            match i {
                '.' => gridpoints.push(Point {
                    x: x as i32,
                    y: y as i32,
                    ele: Element::Dot,
                }),
                '0'..='9' => gridpoints.push(Point {
                    x: x as i32,
                    y: y as i32,
                    ele: Element::Digit,
                }),
                _ => gridpoints.push(Point {
                    x: x as i32,
                    y: y as i32,
                    ele: Element::Symbol,
                }),
            }
            // Get number information
            match i {
                '0'..='9' => number.push(i),
                _ => {
                    if number.len() > 0 {
                        numbers.push(Number {
                            y: y as i32,
                            xmin: x as i32 - number.len() as i32,
                            xmax: x as i32 - 1 as i32,
                            value: String::from_iter(number).parse().unwrap(),
                        });
                        number = Vec::new();
                    }
                }
            }
        }
        if number.len() > 0 {
            numbers.push(Number {
                y: y as i32,
                xmin: maxline as i32 - number.len() as i32,
                xmax: maxline as i32 - 1 as i32,
                value: String::from_iter(number).parse().unwrap(),
            });
        }
    }
    return (Grid { points: gridpoints }, numbers);
}

fn run(input: String) -> u32 {
    let (grid, numbers) = parse_input(input);
    // for n in numbers {
    //     println!("{}, {}, {}, {}, {}", n.value, n.y, n.xmin, n.xmax, n.has_symbol(&grid))
    // }
    let sum: u32 = numbers
        .iter()
        .map(|n| n.value * n.has_symbol(&grid) as u32)
        .sum();
    return sum;
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
        let input: &'static str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(run(input.to_string()), 4361);
    }
}
