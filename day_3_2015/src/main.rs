use colored::Colorize;
use std::fs;

struct Position {
    x: isize,
    y: isize,
}

fn print_2d(array: &Vec<Vec<u32>>, size: isize) -> () {
    let mut row: usize = 0;
    let mut count = 0;
    while count != size * size {
        for col in array {
            if count % size == 0 && count != 0 {
                println!(";");
                if row as isize == size - 1 {
                    ();
                } else {
                    row += 1;
                }
            } else {
                if col[row] == 0 {
                    print!("{}", "_  ".blue());
                } else {
                    if col[row] > 9 {
                        print!("{} ", col[row].to_string().red());
                    } else if col[row] > 1 {
                        print!("{}  ", col[row].to_string().yellow());
                    } else {
                        print!("{}  ", col[row].to_string().green());
                    }
                }
            }
            count += 1;
        }
    }
}

fn size(directions: &String) -> usize {
    let mut list = Vec::new();
    let mut x_value: isize = 0;
    let mut y_value: isize = 0;

    for char in directions.trim().chars() {
        match char {
            '^' => {
                y_value += 1;
                list.push(y_value.abs());
            }
            'v' => {
                y_value -= 1;
                list.push(y_value.abs());
            }
            '>' => {
                x_value += 1;
                list.push(x_value.abs());
            }
            '<' => {
                x_value -= 1;
                list.push(x_value.abs());
            }
            _ => (),
        };
    }
    let max_opt = list.iter().max();
    let max = match max_opt {
        Some(max) => *max as usize + 2,
        None => 0,
    };

    if max % 2 == 0 {
        max
    } else {
        max + 1
    }
}

fn navigate(raw_size: usize, directions: &String) -> i32 {
    println!(
        "Array initializing with size {raw_size} * 2 = {}",
        raw_size * 2
    );
    let size = raw_size as isize * 2;
    let mut neighborhood: Vec<Vec<u32>> =
        vec![vec![Default::default(); raw_size * 2]; raw_size * 2];
    let mut location = Position { x: 0, y: 0 };
    neighborhood[(size / 2) as usize][(size / 2) as usize] = 1;
    let mut runs = 0;

    for (char_index, char) in directions.trim().chars().enumerate() {
        match char {
            '^' => {
                location.y += 1;
            }
            'v' => {
                location.y -= 1;
            }
            '>' => {
                location.x += 1;
            }
            '<' => {
                location.x -= 1;
            }
            _ => continue,
        };
        let col_index = match usize::try_from(location.x + (size / 2)) {
            Ok(i) => i,
            Err(e) => {
                eprintln!(
                    "Failed to convert col_index = {} to usize with error {e}. run : {}",
                    location.x + (size / 2),
                    runs
                );
                break;
            }
        };
        let col = match neighborhood.get_mut(col_index) {
            Some(col) => col,
            None => {
                eprintln!("At character {char_index} attempted to index into columns with index of {col_index} and went out of bounds.");
                break;
            }
        };
        let row_index = match usize::try_from(location.y + (size / 2)) {
            Ok(i) => i,
            Err(e) => {
                eprintln!(
                    "Failed to convert row_index = {} to usize with error {e}. run : {}",
                    location.x + (size / 2),
                    runs
                );
                break;
            }
        };
        match col.get_mut(row_index) {
            Some(row) => *row += 1,
            None => {
                eprintln!("At character {char_index} attempted to index into column at {col_index} with index of {row_index} and went out of bounds.");
                break;
            }
        }
        runs += 1;
    }

    print_2d(&neighborhood, size);

    let mut repeats = 0;
    for vector in neighborhood.iter() {
        for num in vector.iter() {
            match num {
                0 | 1 => (),
                _ => repeats += 1,
            };
        }
    }
    repeats
}

fn main() {
    let directions = match fs::read_to_string("directions.txt") {
        Ok(steps) => steps,
        Err(e) => String::from(format!("{}", e)),
    };

    let repeats = navigate(size(&directions), &directions);

    println!("Repeats: {}", repeats);
}
