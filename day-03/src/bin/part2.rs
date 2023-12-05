use std::fs;
use itertools::Itertools;

fn read_input() -> String {
    let input: String = fs::read_to_string("src/bin/input.txt").expect("Expected to read the file");
    return input;
}

enum Element {
    Dot,
    Digit,
    Symbol
}

struct Point {
    id: u32,
    x: i32,
    y: i32,
    ele: Element
}

struct Grid {
    points: Vec<Point>
}

impl Grid {
    fn get_number_id(&self, x:i32, y:i32) -> u32 {
        let point = self.points.iter().filter(|p| p.x == x && p.y == y).next();
        return match point {
            None => 0,
            _ => match point.unwrap().ele {
                Element::Digit => point.unwrap().id,
                _ => 0
            }
        }
    }
}

struct Number {
    id: u32,
    value: u32
}

struct Gear {
    y: i32,
    x: i32,
}

impl Gear {
    fn get_neighbours(&self) -> Vec<(i32, i32)> {
        let mut neighbours = Vec::new();
        for y in self.y - 1 ..= self.y + 1 {
            for x in self.x - 1 ..= self.x + 1 {
                if y != self.y || x < self.x || x > self.x {
                    neighbours.push((x, y));
                }
            } 
        }
        return neighbours
    }

    fn get_gearratio(&self, grid:&Grid, numbers:&Vec<Number>) -> u32 {
        let neighbours = self.get_neighbours();
        let number_ids: Vec<u32> = neighbours.iter().map(|n|grid.get_number_id(n.0, n.1)).filter(|x| x > &0).unique().collect_vec();
        if number_ids.len() == 2 {
            return numbers.iter().filter(|x| number_ids.contains(&x.id)).map(|x| x.value).product();
        } else {
            return 0;
        }
    }
}

fn parse_input(input: String) -> (Grid, Vec<Number>, Vec<Gear>) {
    let mut numbers = Vec::new();
    let mut gridpoints = Vec::new();
    let mut gears: Vec<Gear> = Vec::new();
    let mut id = 1; 
    for (y, l) in input.lines().enumerate() {
        let mut number: Vec<char> = Vec::new();
        for (x, i) in l.chars().enumerate() {
            // Get grid information
            match i {
                '.' => gridpoints.push(Point{id: 0, x: x as i32, y: y as i32, ele: Element::Dot}),
                '0'..='9' => gridpoints.push(Point{id: id, x: x as i32, y: y as i32, ele: Element::Digit}),
                _ => gridpoints.push(Point{id: 0, x: x as i32, y: y as i32, ele: Element::Symbol}),
            }
            // Get number information
            match i {
                '0'..='9' => number.push(i),
                _ => {
                    if number.len() > 0 {
                        numbers.push(Number{id: id, value: String::from_iter(number).parse().unwrap()});
                        id += 1;
                        number = Vec::new();
                    }
                }
            }
            // Get gear information
            match i {
                '*' => gears.push(Gear{y: y as i32, x: x as i32}),
                _ => {}
            }
        }
        if number.len() > 0 {
            numbers.push(Number{id: id, value: String::from_iter(number).parse().unwrap()});
            id += 1;
        }
    }
    return (Grid{points: gridpoints}, numbers, gears)
}

fn run(input: String) -> u32 {
    let (grid, numbers, gears) = parse_input(input);
    let sum:u32 = gears.iter().map(|n| n.get_gearratio(&grid, &numbers)).sum();
    return sum
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
        assert_eq!(run(input.to_string()), 467835);
    }
}