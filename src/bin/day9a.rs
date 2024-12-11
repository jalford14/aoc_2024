const INPUT: &str = include_str!("../../inputs/day9.txt");

fn main() {
    let mut block: Vec<String> = Vec::new();
    INPUT
        .chars()
        .enumerate()
        .for_each(|(i, char)| {
            if char == '\n' { return; }
            if i % 2 == 0 {
                let file_size: usize = char.to_digit(10).unwrap() as usize;
                let value = (i/2).to_string();
                let mut block_to_append: Vec<String> = vec![value; file_size];
                block.append(&mut block_to_append);
            } else {
                let free_space = char.to_digit(10).unwrap() as usize;
                let mut block_to_append: Vec<String> = vec![".".to_string(); free_space as usize];
                block.append(&mut block_to_append);
            }
        });

    let mut i = 0;
    let mut tail = block.len() - 1;
    let mut acc = 0;

    while i <= tail {
        if block[i] == "." {
            if block[tail] == "." { tail -= 1; continue; }
            let value = block[tail].parse::<usize>().unwrap();
            acc += (value as usize) * i;
            tail -= 1;
        } else {
            let value = block[i].parse::<usize>().unwrap();
            acc += (value as usize) * i;
        }
        i += 1;
    }
    println!("result: {acc}");
}
