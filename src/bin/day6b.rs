const INPUT: &str = include_str!("../../inputs/day6.txt");
const GRID_SIZE: usize = 130;
#[derive(Debug)]
enum Direction { Up, Down, Left, Right }

fn main() {
    // let mut map: Vec<char> = INPUT.chars().filter(|&c| c != '\n').collect();
    // let mut result = 0;
    // let mut guard_pos: usize = map.iter().position(|&r| r == '^').unwrap();
    // let mut direction = Direction::Up;

    // loop {
    //         match direction {
    //             Direction::Up => {
    //                 if GRID_SIZE > guard_pos { break; }
    //                 match map.get(guard_pos - GRID_SIZE) {
    //                     Some('#') => { direction = Direction::Right; }
    //                     Some('.') => { if map[guard_pos] == '.' || map[guard_pos] == '^' { map[guard_pos] = 'X'; } guard_pos -= GRID_SIZE; }
    //                     Some('X') => { if map[guard_pos] == '.' { map[guard_pos] = 'X'; result += 1; } guard_pos -= GRID_SIZE; }
    //                     _ => { break; }
    //                 }
    //             }
    //             Direction::Down => {
    //                 match map.get(guard_pos + GRID_SIZE) {
    //                     Some('#') => { direction = Direction::Left; }
    //                     Some('.') => { if map[guard_pos] == '.' { map[guard_pos] = 'X'; } guard_pos += GRID_SIZE; }
    //                     Some('X') => { if map[guard_pos] == '.' { map[guard_pos] = 'X'; result += 1; } guard_pos += GRID_SIZE; }
    //                     _ => { break; }
    //                 }
    //             }
    //             Direction::Left => {
    //                 match map.get(guard_pos - 1) {
    //                     Some('#') => { direction = Direction::Up; }
    //                     Some('.') => { if map[guard_pos] == '.' { map[guard_pos] = 'X'; } guard_pos -= 1; }
    //                     Some('X') => { if map[guard_pos] == '.' { map[guard_pos] = 'X'; result += 1; } guard_pos -= 1; }
    //                     _ => { break; }
    //                 }
    //             }
    //             Direction::Right => {
    //                 match map.get(guard_pos + 1) {
    //                     Some('#') => { direction = Direction::Down; }
    //                     Some('.') => { if map[guard_pos] == '.' { map[guard_pos] = 'X'; } guard_pos += 1; }
    //                     Some('X') => { if map[guard_pos] == '.' { map[guard_pos] = 'X'; result += 1; } guard_pos += 1; }
    //                     _ => { break; }
    //                 }
    //             }
    //         }
    // }
    // println!("{result}");
}

// fn print_map(map: Vec<char>) {
//     for w in map.chunks(GRID_SIZE) {
//         println!("{:?}", w);
//     }
// }
