use itertools::Itertools;
use nom::bytes::complete::is_not;
use nom::character::complete::alpha1;
use nom::{bytes::complete::tag, multi::separated_list1, IResult};
use std::collections::HashMap;
use std::{fs, iter, vec};

fn read_input() -> String {
    fs::read_to_string("src/bin/input.txt").expect("Expected to read the file")
}

fn parse_input(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(tag(","), is_not(","))(input)
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Lens {
    label: String,
    focal: u32,
}

fn get_hash(input: &str) -> u32 {
    let mut current = 0;
    for i in input.chars().filter(|x| x.is_alphabetic()) {
        current += i as u32;
        current *= 17;
        current %= 256;
    }
    current
}

fn run(input: String) -> u32 {
    let mut boxes: HashMap<u32, Vec<Lens>> = HashMap::new();
    let steps = parse_input(input.as_str()).expect("Expected valid input").1;
    // dbg!(&steps);
    steps
        .into_iter()
        .map(|x| {
            let box_id = get_hash(x);
            // dbg!(&box_id);
            let box_content = boxes.get(&box_id);
            if box_content.is_some() {
                let mut box_content = box_content.unwrap().clone();
                let content_labels = box_content
                    .iter()
                    .map(|lens| lens.label.clone())
                    .collect_vec();

                if x.chars().contains(&'=') {
                    let (label, focal) = x
                        .split('=')
                        .collect_tuple()
                        .expect("Expected exactly two values");

                    if content_labels.contains(&label.to_owned()) {
                        let pos = content_labels
                            .iter()
                            .position(|c_label| c_label == label)
                            .unwrap();
                        box_content[pos] = Lens {
                            label: label.to_owned(),
                            focal: focal.parse().expect("Number expected"),
                        }
                    } else {
                        box_content.push(Lens {
                            label: label.to_owned(),
                            focal: focal.parse().expect("Number expected"),
                        });
                    }
                    boxes.insert(box_id, box_content);
                }
                if x.chars().contains(&'-') {
                    let label = x.strip_suffix('-').expect("Expected '-' at the end");
                    let mut box_content = boxes[&box_id].clone();
                    if box_content
                        .iter()
                        .map(|lens| lens.label.clone())
                        .collect_vec()
                        .contains(&label.to_owned())
                    {
                        let pos = box_content
                            .iter()
                            .map(|lens| lens.label.clone())
                            .position(|label_old| label_old == label)
                            .unwrap();
                        box_content.remove(pos);
                        boxes.insert(box_id, box_content);
                    }
                }
            } else {
                if x.chars().contains(&'=') {
                    let (label, focal) = x
                        .split('=')
                        .collect_tuple()
                        .expect("Expected exactly two values");

                    boxes.insert(
                        box_id,
                        vec![Lens {
                            label: label.to_owned(),
                            focal: focal.parse().expect("Number expected"),
                        }],
                    );
                }
            }
        })
        .collect_vec();
    // dbg!(&boxes);
    boxes
        .into_iter()
        .map(|(box_id, content)| {
            (box_id + 1)
                * content
                    .into_iter()
                    .enumerate()
                    .map(|(pos, lens)| (1 + pos as u32) * lens.focal)
                    .sum::<u32>()
        })
        .sum::<u32>()
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
        let input: &'static str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(run(input.to_string()), 145);
    }
}
