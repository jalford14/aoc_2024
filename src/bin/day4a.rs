const INPUT: &str = include_str!("../../inputs/day4.txt");
const ROW_SIZE: usize = 148;

fn main() {
    let mut result = 0;
    let mut puzzle: Vec<char> = Vec::new();
    
    INPUT
        .lines()
        .for_each(|line| {
                for char in line.chars() {
                    puzzle.push(char);
                }
            });

    for (i, &letter) in puzzle.iter().enumerate() {
        if letter == 'X' {
            if puzzle[i] == 'X' && puzzle[i+1] == 'M' && puzzle[i+2] == 'A' && puzzle[i+3] == 'S' { result += 1; }
            if puzzle[i] == 'X' && puzzle[i-1] == 'M' && puzzle[i-2] == 'A' && puzzle[i-3] == 'S' { result += 1; }
            if [puzzle[i], puzzle[i + ROW_SIZE], puzzle[i + 2*ROW_SIZE], puzzle[i + 3*ROW_SIZE]].iter().collect::<String>() == "XMAS" { result += 1; }
            if [puzzle[i], puzzle[i - ROW_SIZE], puzzle[i - 2*ROW_SIZE], puzzle[i - 3*ROW_SIZE]].iter().collect::<String>()  == "XMAS" { result += 1; }
            if [puzzle[i], puzzle[i+1 - ROW_SIZE], puzzle[i+2 - 2*ROW_SIZE], puzzle[i+3 - 3*ROW_SIZE]].iter().collect::<String>() == "XMAS" { result += 1; }
            if [puzzle[i], puzzle[i-1 - ROW_SIZE], puzzle[i-2 - 2*ROW_SIZE], puzzle[i-3 - 3*ROW_SIZE]].iter().collect::<String>() == "XMAS" { result += 1; }
            if [puzzle[i], puzzle[i+1 + ROW_SIZE], puzzle[i+2 + 2*ROW_SIZE], puzzle[i+3 + 3*ROW_SIZE]].iter().collect::<String>() == "XMAS" { result += 1; }
            if [puzzle[i], puzzle[i-1 + ROW_SIZE], puzzle[i-2 + 2*ROW_SIZE], puzzle[i-3 + 3*ROW_SIZE]].iter().collect::<String>() == "XMAS" { result += 1; }
        }
    }
    println!("{result}");
}
