const INPUT: &str = include_str!("../../inputs/day2.txt");

// Scenarios
// 1. The two levels are equal -> safe, compare the next levels
// 2. One level is higher than the other
//      a. difference is greater than 3 -> unsafe, goto next line
//      b. the levels had been previously descending -> unsafe, goto next line
//      c. none of the above -> safe, compare the next levels
// 3. One level is lower than the other
//      a. difference is greater than 3 -> unsafe, goto next line
//      b. the levels had been previously ascending -> unsafe, goto next line
//      c. none of the above -> safe, compare the next levels
// 4. EOL -> acc + 1

fn main() {
    let result = INPUT
        .lines()
        .fold(0, |acc, line| {
            let mut safe = 1;
            let mut asc = false;
            let mut desc = false;
            let parts: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            for i in 0..parts.len() - 1 {
                // 1.
                if parts[i] == parts[i+1] { safe = 0; break; }
                if parts[i].abs_diff(parts[i+1]) > 3 {
                    safe = 0;
                    break;
                    // 2.
                } 
                if parts[i] < parts[i+1] {
                    if desc == true { safe = 0; break; }
                    else { asc = true; }
                } 
                if parts[i] > parts[i+1] {
                    if asc == true { safe = 0; break; }
                    else { desc = true; }
                }
            }
            acc + safe
        });

    println!("{result}");
}
