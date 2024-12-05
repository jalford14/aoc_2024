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
            let mut value = 1;
            let mut l = 0;
            let mut skip_idx = 1000;
            let mut safe = true;
            let mut asc = false;
            let mut desc = false;
            let parts: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            // 5 1 2 3 4 not work
            while l < parts.len() - 1 {
                let mut r = l + 1;
                if skip_idx == l { l += 1; continue; }
                if skip_idx == r { r += 1; }
                if l >= parts.len() || r >= parts.len() { break; }
                println!("{} {}", parts[l], parts[r]);

                match is_safe(parts[l], parts[r], asc, desc) {
                    "true" => { l += 1 },
                    "false" => {
                        if safe { 
                    //     if safe { skip_idx = r; safe = false; }
                    //     else { value = 0; break; }
                            println!("bullshit: {} {}", parts[l], parts[r+1]);
                            println!("bullshit: {} {}", parts[l+1], parts[r+1]);
                            if is_safe(parts[l], parts[r+1], asc, desc) != "false" {
                                 skip_idx = r; safe = false; 
                            } else if is_safe(parts[l+1], parts[r+1], asc, desc) != "false" {
                                 skip_idx = l; safe = false; 
                            } else {
                                value = 0;
                                break;
                            }
                        }
                        else { value = 0; break; }
                    },
                    "asc" => {
                        asc = true;
                        l += 1;
                    },
                    "desc" => {
                        desc = true;
                        l += 1;
                    },
                    _ => l += 1
                }
            }
            if value == 1 { println!("safe!"); }
            acc + value
        });

    println!("{result}");
}

fn is_safe(l: i32, r: i32, asc: bool, desc: bool) -> &'static str {
    if l == r { return "false"; }
    else if l.abs_diff(r) > 3 { return "false"; } 
    else if l < r {
        if desc == true { return "false"; }
        else { return "asc"; }
    } 
    else if l > r {
        if asc == true { return "false"; }
        else { return "desc"; }
    }
    else { return "true"; }
}

// Here is the solution I stole from @nated0g. I'm a cheater!
// Ultimately, rather than attempt to write out each permutation,
// this solution generates all of them by removing a number from
// the line until the permutation is valid. Nice.
//
// fn main() {
//     let result = part2(INPUT);
//     println!(k{result}");
// }
// 
// fn safe1(nums: &[i32]) -> u32 {
//     let ascending = nums[0] < nums[1];
// 
//     let mut prev = nums[0];
//     for &n in &nums[1..] {
//         let diff = (prev - n).abs();
// 
//         let no_diff = diff < 1;
//         let too_big_gap = diff > 3;
//         let wrong_direction = (ascending && prev > n) || (!ascending && prev < n);
// 
//         if no_diff || too_big_gap || wrong_direction {
//             return 0;
//         }
//         prev = n;
//     }
//     1
// }
// 
// fn safe2(nums: &[i32]) -> u32 {
//     if safe1(nums) == 1 {
//         return 1;
//     }
// 
//     for skip_idx in 0..nums.len() {
//         let subset: Vec<i32> = nums
//             .iter()
//             .enumerate()
//             .filter(|(i, _)| *i != skip_idx)
//             .map(|(_, &n)| n)
//             .collect();
// 
//         if safe1(&subset) == 1 {
//             return 1;
//         }
//     }
//     0
// }
// 
// pub fn part2(input: &str) -> String {
//     input
//         .lines()
//         .into_iter()
//         .map(|line| {
//             let nums: Vec<i32> = line
//                 .split_whitespace()
//                 .map(|n| n.parse::<i32>().unwrap())
//                 .collect();
//             safe2(&nums)
//         })
//         .sum::<u32>()
//         .to_string()
// }
