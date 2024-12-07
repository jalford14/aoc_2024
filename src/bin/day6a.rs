use std::{thread, time};

const INPUT: &str = include_str!("../../inputs/day6.txt");
const GRID_SIZE: usize = 10;
#[derive(Debug)]
enum Direction { Up, Down, Left, Right }

fn main() {
    let time = time::Duration::from_millis(300);
    let mut map: Vec<char> = INPUT.chars().filter(|&c| c != '\n').collect();
    let mut result = 0;
    let mut guard_pos: usize = INPUT.find("^").unwrap() - 1;
    let mut direction = Direction::Up;

    loop {
            // this isn't working because you're checking the position
            // ahead rather than where the guard actually is. so you're 
            // looking ahead and then not updating the result because
            // there's an 'X' ahead
            println!("*******************");
            match direction {
                Direction::Up => {
                    match map.get(guard_pos - GRID_SIZE) {
                        Some('#') => { direction = Direction::Right; }
                        Some('.') => { map[guard_pos] = 'X'; result += 1; guard_pos -= GRID_SIZE; }
                        Some('X') => { guard_pos -= GRID_SIZE; }
                        _ => { if map[guard_pos] == '.' { result += 1} break; }
                    }
                    print_map(map.clone());
                }
                Direction::Down => {
                    match map.get(guard_pos + GRID_SIZE) {
                        Some('#') => { direction = Direction::Left; }
                        Some('.') => { map[guard_pos] = 'X'; result += 1; guard_pos += GRID_SIZE; }
                        Some('X') => { guard_pos += GRID_SIZE; }
                        _ => { if map[guard_pos] == '.' { result += 1} break; }
                    }
                    print_map(map.clone());
                }
                Direction::Left => {
                    match map.get(guard_pos - 1) {
                        Some('#') => { direction = Direction::Up; }
                        Some('.') => { map[guard_pos] = 'X'; result += 1; guard_pos -= 1; }
                        Some('X') => { guard_pos -= 1; }
                        _ => { if map[guard_pos] == '.' { result += 1} break; }
                    }
                    print_map(map.clone());
                }
                Direction::Right => {
                    match map.get(guard_pos + 1) {
                        Some('#') => { direction = Direction::Down; }
                        Some('.') => { map[guard_pos] = 'X'; result += 1; guard_pos += 1; }
                        Some('X') => { guard_pos += 1; }
                        _ => { if map[guard_pos] == '.' { result += 1} break; }
                    }
                    print_map(map.clone());
                }
            }
            thread::sleep(time);
    }
    println!("{result}");
}

fn print_map(map: Vec<char>) {
    for w in map.chunks(10) {
        println!("{:?}", w);
    }
}
