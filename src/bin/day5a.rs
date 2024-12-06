use std::collections::{HashMap, HashSet};
use std::{thread, time};

const INPUT: &str = include_str!("../../inputs/day5.txt");

fn main() {
    let time = time::Duration::from_millis(1000);
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
                    let page_numbers: Vec<&str> = line.split(",").collect();
                    let mut safe = true;
                    page_numbers
                        .iter()
                        .enumerate()
                        .for_each(|(i, page_number)| {
                            thread::sleep(time);
                            if i < page_numbers.len() - 1 && rules.get(page_number) != None {
                                println!("rules.get(page_number): {:?}", rules.get(page_number));
                                for pgno_idx in (0..page_numbers.len()-2).rev() {
                                    let rules_set: HashSet<&str> = HashSet::from(rules[page_number]);
                                    let page_rules_to_check: HashSet<&str> = HashSet::from(page_numbers[0..pgno_idx]);
                                    println!("pgno_idx {pgno_idx}");
                                    println!("page_numbers[pgno_idx]: {}", page_numbers[pgno_idx]);
                                    if rules_set.intersection(&page_rules_to_check).collect() == [] { safe = false; }
                                }
                            }
                        });
                    if safe {
                        println!("safe! {:?}", page_numbers);
                        result += page_numbers[page_numbers.len() / 2].parse::<i32>().unwrap();
                    }
                }
        });

    println!("{result}");
}
