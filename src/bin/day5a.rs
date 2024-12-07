use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../../inputs/day5.txt");

fn main() {
    let mut rules: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut result = 0;

    INPUT
        .lines()
        .for_each(|line| {
                if line.contains("|") {
                    let rules_to_insert: Vec<&str> = line.split("|").collect();
                    if rules.get(rules_to_insert[0]) != None {
                        rules.entry(rules_to_insert[0]).and_modify(|val| val.push(rules_to_insert[1]));
                    } else {
                        rules.insert(rules_to_insert[0], Vec::new());
                        rules.entry(rules_to_insert[0]).and_modify(|val| val.push(rules_to_insert[1]));
                    }
                } else if line.contains(",") {
                    let mut page_numbers: Vec<&str> = line.split(",").collect();
                    let mut safe = true;
                    page_numbers.reverse();
                    page_numbers
                        .iter()
                        .enumerate()
                        .for_each(|(i, page_number)| {
                            if !safe { return; }
                            if i < page_numbers.len() - 1 && rules.get(page_number) != None {
                                let rules_set: HashSet<&str> = rules[page_number].iter().rev().cloned().collect();
                                let page_rules_to_check: HashSet<&str> = page_numbers[i+1..].iter().rev().cloned().collect();
                                if !rules_set.intersection(&page_rules_to_check).collect::<Vec<_>>().is_empty() { 
                                    safe = false;
                                }
                            }
                        });
                    if safe {
                        result += page_numbers[page_numbers.len() / 2].parse::<i32>().unwrap();
                    }
                }
        });

    println!("{result}");
}
