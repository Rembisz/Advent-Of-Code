use std::{io, fs};
use std::io::prelude::*;

enum Action {
    Toggle,
    On,
    Off
}

fn main() {
    let mut x_value = String::new();
    let mut y_value = String::new();

    println!("How many columns are in your grid?");

    'x_val: loop {
        x_value.clear();
        io::stdin().read_line(&mut x_value).unwrap(); // Take column input and check 
        for character in x_value.trim().chars() {
            match character.to_digit(10) {
                Some(_) => (),
                None => {
                    println!("Invalid, try again.");
                    continue 'x_val;
                }
            }  
        }
        break;
    }

    println!("How many rows are in your grid?");

    'y_val: loop {
        y_value.clear();
        io::stdin().read_line(&mut y_value).unwrap(); // Take row input and check 
        for character in y_value.trim().chars() {
            match character.to_digit(10) {
                Some(_) => (),
                None => {
                    println!("Invalid, try again.");
                    continue 'y_val;
                }
            }
        }
        break;
    }   
    
    let grid_size_x = x_value.trim().parse::<i32>().unwrap(); // Store inputs as integer values
    let grid_size_y = y_value.trim().parse::<i32>().unwrap();
    println!("\nX: {}\nY: {}\n", grid_size_x, grid_size_y);

    let instructions = match fs::read_to_string("src/instructions.txt") {
        Ok(contents) => contents, // Reads instructions and assigns them to a string using read_to_string
        Err(e) => panic!("Invalid input file. (Error: {})", e), // Error handling
    };

    let axis = vec![0; grid_size_x as usize];
    let mut plane = vec![axis; grid_size_y as usize]; // Create the 2d plane of lights (bools)

    let action_list = parse_actions(&instructions); // Create a list of each action performed in order using parse_actions

    let mut line_number = 0;
    for line in instructions.lines() {
        let coordinates = parse_coordinates(&line); 
        let x1 = coordinates[0];
        let y1 = coordinates[1];// For each line, pull the coordinates using parse_coordinates
        let x2 = coordinates[2];
        let y2 = coordinates[3];

        let mut count = 0;
        for coord in coordinates {
            if coord <= grid_size_x && count % 2 == 0 { // Match coordinates to grid size entered in order to avoid errors
                count += 1;
                continue;
            } else if coord <= grid_size_y && count % 2 != 0 {
                count += 1;
                continue;
            } else if count > 3 {
                break;
            } else {
                println!("Grid size entered does not match instruction file!\nPlease update 'src/instructions.txt'");
                return;
            }
        }

        perform_action(false, x1 as usize, y1 as usize, x2 as usize, y2 as usize, &mut plane, &action_list[line_number]);
        // For each line, perform the respective action (from action_list) on the plane using perform_action

        line_number += 1; // Count each line number so as not to lose coordinate sync with action_list
    }

    let on = count_print_results(plane);
    println!("\n\nThere are {} lights; {} of which are on, and {} off.\n", grid_size_x * grid_size_y, on, grid_size_x * grid_size_y - on); // Count and print results
    println!("Next up, variation two.\n");

    pause(); // Pause program before variation two

    let axis_2 = vec![0; grid_size_x as usize];
    let mut plane_2 = vec![axis_2; grid_size_y as usize]; // Create the 2d plane of lights (bools)
    let mut line_number = 0; // Reset our line number
    for line in instructions.lines() {
        let coordinates = parse_coordinates(&line); 
        let x1 = coordinates[0];
        let y1 = coordinates[1];// For each line, pull the coordinates using parse_coordinates
        let x2 = coordinates[2];
        let y2 = coordinates[3];

        perform_action(true, x1 as usize, y1 as usize, x2 as usize, y2 as usize, &mut plane_2, &action_list[line_number]);
        // For each line, perform the respective action (from action_list) on the plane using perform_action

        line_number += 1; // Count each line number so as not to lose coordinate sync with action_list
    }

    let brightness = count_print_results_2(plane_2);
    println!("\n\nTotal brightness is {}.", brightness);
}

fn count_print_results_2(plane: Vec<Vec<i32>>) -> i64{
    let mut total: i64 = 0;
    for column in 0..plane.len() {
        println!("");
        for row in 0..plane[0].len() { // Count every light's brightness and return that value
            print!(" {} ", plane[column][row]); // Print row indices without skipping lines
            total += plane[column][row] as i64;
        }
    }
    total
}

fn count_print_results(plane: Vec<Vec<i32>>) -> i32 {
    let mut on = 0;
    for column in 0..plane.len() {
        println!("");
        for row in 0..plane[0].len() {
            if plane[column][row] > 0 { // Count every light that is on and return that value
                print!(" 1 "); // Print row indices that are on without skipping lines
                on += 1;
            } else {
                print!(" 0 "); // Same but off instead
            }
        }
    }
    on
}

fn parse_coordinates(line: &str) -> Vec<i32> {
    let mut coordinates = vec![];
    let unfiltered_coordinates: Vec<&str> = line.trim().split(' ').collect();
    let mut filtered_coordinates: Vec<&str> = vec![];
    'str_loop: for str in unfiltered_coordinates {
        for char in str.chars() {
            match char.to_digit(10) {
                Some(_) => {
                    filtered_coordinates.push(str);
                    continue 'str_loop;
                }
                None => ()
            }
        }
    }
    // Convert the line to a vector of strings and filter out useless ones

    for coordinate_pair in filtered_coordinates {
        let pair_vec: Vec<&str> = coordinate_pair.split(',').collect();
        let x = pair_vec[0];
        let y = pair_vec[1]; // Split each leftover string over the comma and retain the numbers in order as coordinates (x1 then y1 then loop for 2)
        coordinates.push(x.to_string().parse::<i32>().unwrap());
        coordinates.push(y.to_string().parse::<i32>().unwrap());
    }
    coordinates
}

fn parse_actions(instructions: &String) -> Vec<Action> {  
    let mut actions: Vec<Action> = vec![];
    for line in instructions.lines() {
        match line.find("toggle") {
            Some(_) => actions.push(Action::Toggle),
            None => match line.find("on") {
                Some(_) => actions.push(Action::On), // Match each respective word found in the line to an instance of the Action enum and push to vector (actions)
                None => match line.find("off") {
                    Some(_) => actions.push(Action::Off),
                    None => panic!("Incorrect instruction format!"),
                }
            }
        }
    }
    actions
}

fn perform_action(variation: bool, x1: usize, y1: usize, x2: usize, y2: usize, plane: &mut Vec<Vec<i32>>, action: &Action) {
    match action {
        Action::Toggle => {
            for column in x1..x2 + 1 { // Toggle every index in the given coordinates [(x1,y1), (x2,y2)]
                for row in y1..y2 + 1 {
                    if variation {
                        toggle_2(&mut plane[column][row]);
                    } else {
                        toggle(&mut plane[column][row]);
                    }
                }
            }
        }
        Action::On => {
            for column in x1..x2 + 1 { // Make every index in the given coordinates true [(x1,y1), (x2,y2)]
                for row in y1..y2 + 1 {
                    if variation {
                        activate_2(&mut plane[column][row]);
                    } else {
                        activate(&mut plane[column][row]);
                    }
                }
            }
        }
        Action::Off => {
            for column in x1..x2 + 1 { // Make every index in the given coordinates false [(x1,y1), (x2,y2)]
                for row in y1..y2 + 1 {
                    if variation {
                        deactivate_2(&mut plane[column][row]);
                    } else {
                        deactivate(&mut plane[column][row]);
                    }
                }
            }
        }
    }
}

fn toggle(switch: &mut i32) {
    if *switch == 0 {
        activate(switch);
    } else {
        deactivate(switch);
    }
}

fn activate(switch: &mut i32) {
    *switch = 1;
}

fn deactivate(switch: &mut i32) {
    *switch = 0;
}

fn toggle_2(switch: &mut i32) {
    *switch += 2;
}

fn activate_2(switch: &mut i32) {
    *switch += 1;
}

fn deactivate_2(switch: &mut i32) {
    if *switch > 0 {
        *switch -= 1;
    }
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(stdout, "Press any key to continue...").unwrap();// Cursor will stay at the end of the line
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap(); // Read a single byte and ignore
}

