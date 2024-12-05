use std::collections::HashMap;
const INPUT: &str = include_str!("../../inputs/day1.txt");

fn main() {
    let mut vec: Vec<&str> = Vec::new();
    let mut duplicates = HashMap::<&str, i32>::new();

    INPUT
        .lines()
        .for_each(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            vec.push(parts[0]);
            if duplicates.contains_key(parts[1]) {
                duplicates.entry(parts[1]).and_modify(|val| *val += 1);
            } else {
                duplicates.insert(parts[1], 1);
            }
        });

    let result = vec.iter().fold(0, |acc, el| {
        if duplicates.contains_key(el) {
            let count = duplicates[el];
            acc + el.parse::<i32>().unwrap() * count
        } else {
            acc + 0
        }
    });

    println!("{result}");
}
