use std::collections::HashMap;
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
                    let line_splits: Vec<&str> = line.split(",").collect();
                    let mut safe = true;
                    line_splits
                        .iter()
                        .enumerate()
                        .for_each(|(i, line_split)| {
                            if i < line_splits.len() - 1 && rules.get(line_split) != None {
                                println!("rules.get(line_split): {:?}", rules.get(line_split));
                                for comp_idx in line_splits.len()-2..0 {
                                    println!("comp_idx {comp_idx}");
                                    println!("line_splits[comp_idx]: {}", line_splits[comp_idx]);
                                    if rules[line_split].contains(&line_splits[comp_idx]) { safe = false; }
                                }
                            }
                        });
                    if safe {
                        result += line_splits[line_splits.len() / 2].parse::<i32>().unwrap();
                    }
                }
        });

    println!("{result}");
}
