use std::fs;

fn read_input() -> String {
    let input: String = fs::read_to_string("src/bin/input.txt").expect("Expected to read the file");
    input
}

fn get_value(s: String) -> u32 {
    let mut number = String::new();
    for i in s.chars() {
        if i.is_numeric() {
            number.push(i);
            break;
        }
    }
    for i in s.chars().rev() {
        if i.is_numeric() {
            number.push(i);
            break;
        }
    }
    
    number.parse().expect("Expected parsable number")
}

fn get_sum(input: String) -> u32 {
    let mut sum = 0;
    for i in input.lines() {
        sum += get_value(i.to_string());
    }
    sum
}

fn main() {
    let input: String = read_input();
    println!("{}", { get_sum(input) });
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test() {
        let input: &'static str = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        assert_eq!(get_sum(input.to_string()), 142);
    }
}
