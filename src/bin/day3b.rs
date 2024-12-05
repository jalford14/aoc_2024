const INPUT: &str = include_str!("../../inputs/day3.txt");

fn main() {
    let mut result = 0;
    let mut enabled = true;
    
    INPUT
        .lines()
        .for_each(|line| {
                for (i, char) in line.chars().enumerate() {
                    if char == 'm' && enabled {
                        let valid_pattern: Vec<char> = Vec::from(['u', 'l', '(', ',', ')']);
                        let mut pattern_pos = 0;
                        let mut checking_num = false;
                        let mut num1: Vec<u32> = Vec::new();
                        let mut num2: Vec<u32> = Vec::new();
                        let mut first_num = true;
                        let mut second_num = false;

                        for mul_char in line[i+1..].chars() {
                            if pattern_pos > 4 { break; }
                            if !checking_num && mul_char != valid_pattern[pattern_pos] { break; }
                            if !checking_num && mul_char == valid_pattern[pattern_pos] {
                                if mul_char == '(' { checking_num = true; }
                                pattern_pos += 1;
                            }
                            else if checking_num {
                                if mul_char == ')' { 
                                    let mut res1 = 0;
                                    let mut res2 = 0;
                                    for &digit in num1.iter() { res1 = res1 * 10 + digit; }
                                    for &digit in num2.iter() { res2 = res2 * 10 + digit; }
                                    result += res1 * res2;
                                    break;
                                }
                                if mul_char == ',' { 
                                    first_num = false;
                                    second_num = true;
                                    continue;
                                }
                                match mul_char.to_digit(10) {
                                    Some(num) => {
                                        if first_num { num1.push(num); }
                                        if second_num { num2.push(num); }
                                    }
                                    None => { break; }
                                }
                            }
                        }
                    }
                    else if char == 'd' {
                        if &line[i..i+4] == "do()" { enabled = true; }
                        if &line[i..i+7] == "don't()" { enabled = false; }
                    }
                }
            });
    println!("{result}");
}
