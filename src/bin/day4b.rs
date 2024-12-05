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
        if letter == 'A' {
            // M M
            //  A
            // S S
            if puzzle[i-1 - ROW_SIZE] == 'M' && puzzle[i+1 + ROW_SIZE] == 'S' && puzzle[i+1 - ROW_SIZE] == 'M' && puzzle[i-1 + ROW_SIZE] == 'S' { result += 1; }
            // S M
            //  A
            // S M
            else if puzzle[i-1 - ROW_SIZE] == 'S' && puzzle[i+1 + ROW_SIZE] == 'M' && puzzle[i+1 - ROW_SIZE] == 'M' && puzzle[i-1 + ROW_SIZE] == 'S' { result += 1; }
            // S S
            //  A
            // M M
            else if puzzle[i-1 - ROW_SIZE] == 'S' && puzzle[i+1 + ROW_SIZE] == 'M' && puzzle[i+1 - ROW_SIZE] == 'S' && puzzle[i-1 + ROW_SIZE] == 'M' { result += 1; }
            // M S
            //  A
            // M S
            else if puzzle[i-1 - ROW_SIZE] == 'M' && puzzle[i+1 + ROW_SIZE] == 'S' && puzzle[i+1 - ROW_SIZE] == 'S' && puzzle[i-1 + ROW_SIZE] == 'M' { result += 1; }
        }
    }
    println!("{result}");
}
