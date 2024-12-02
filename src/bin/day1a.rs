const INPUT: &str = include_str!("../../inputs/day1.txt");

fn main() {
    let mut vec1: Vec<u32> = Vec::new();
    let mut vec2: Vec<u32> = Vec::new();

    INPUT
        .lines()
        .for_each(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let (Ok(left), Ok(right)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
                vec1.push(left);
                vec2.push(right);
            }
        });
    vec1.sort();
    vec2.sort();

    let result = vec1.iter().enumerate().fold(0, |acc, (i, el)| {
        acc + el.abs_diff(vec2[i])
    });

    println!("{result}");
}
