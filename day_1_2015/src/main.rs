use std::fs;

fn part_one(directions: &String) -> i32 {
    let mut floor = 0;
    for step in directions.trim().chars() {
        match step {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
    }
    floor
}

fn part_two(directions: &String) -> (char, i32) {
    let mut floor = 0;
    let mut basement_entry_step = (' ', 0);
    for step in directions.trim().chars() {
        basement_entry_step.1 += 1;
        match step {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
        if floor <= -1 {
            basement_entry_step.0 = step;
            break;
        }
    }
    basement_entry_step
}

fn main() {
    let directions = match fs::read_to_string("directions.txt") {
        Ok(steps) => steps,
        Err(kind) => String::from(format!("{}", kind)),
    };

    println!("Head to floor {}", part_one(&directions));

    let basement_entry_step = part_two(&directions);
    println!(
        "The basement will be entered on step {}.\nCharacter {} will cause you to enter the basement.",
        basement_entry_step.1, basement_entry_step.0
    );

    println!(
        "If the directions analyzed are not yours, please check the 'directions.txt' file in root."
    );
}
