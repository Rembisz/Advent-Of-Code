use std::fs;

pub struct Present {
    length: u32,
    width: u32,
    height: u32,
}

impl Default for Present {
    fn default() -> Present {
        Present {
            length: 0,
            width: 0,
            height: 0,
        }
    }
}

fn ribbon(present: &Present) -> u32 {
    let mut smallest_side = 0;
    let mut smaller_side = 0;
    let mut smaller_found = false;

    if present.length <= present.width && present.length <= present.height {
        smallest_side += present.length;
    } else if present.width <= present.length && present.width <= present.height {
        smallest_side += present.width;
    } else {
        smallest_side += present.height;
    }

    if smallest_side == present.length && !smaller_found {
        if present.width <= present.height {
            smaller_side += present.width;
            smaller_found = true;
        } else {
            smaller_side += present.height;
            smaller_found = true;
        }
    }
    if smallest_side == present.width && !smaller_found {
        if present.length <= present.height {
            smaller_side += present.length;
            smaller_found = true;
        } else {
            smaller_side += present.height;
            smaller_found = true;
        }
    }
    if smallest_side == present.height && !smaller_found {
        if present.length <= present.width {
            smaller_side += present.length;
        } else {
            smaller_side += present.width;
        }
    }

    let ribbon_wrap = smaller_side + smaller_side + smallest_side + smallest_side;
    let ribbon_bow = present.length * present.width * present.height;
    return ribbon_wrap + ribbon_bow;
}

fn main() {
    let presents = match fs::read_to_string("presents.txt") {
        Ok(dimensions) => dimensions,
        Err(kind) => String::from(format!("{}", kind)),
    };
    let mut present = Present::default();
    let mut total_count = 0;
    let mut total_paper: u32 = 0;
    let mut total_ribbon = 0;
    for dimensions in presents.split_whitespace() {
        let mut dimension_count: i8 = 1;
        let mut dimension_vec = Vec::new();
        let mut index_count = 0;

        for char in dimensions.chars() {
            dimension_vec.push(char);
        }
        dimension_vec.push('|');

        let mut two_digit = false;
        let mut not_number;
        for &index in &dimension_vec {
            match index.to_digit(10) {
                Some(num) => {
                    not_number = false;
                    if two_digit {
                        two_digit = false;
                    } else {
                        match dimension_count {
                            1 => {
                                match &dimension_vec[&index_count + 1].to_string().parse::<u32>() {
                                    Ok(num2) => {
                                        present.length += num * 10;
                                        present.length += num2;
                                        two_digit = true;
                                    }
                                    Err(_) => present.length += num,
                                }
                            }
                            2 => {
                                match &dimension_vec[&index_count + 1].to_string().parse::<u32>() {
                                    Ok(num2) => {
                                        present.width += num * 10;
                                        present.width += num2;
                                        two_digit = true;
                                    }
                                    Err(_) => present.width += num,
                                }
                            }
                            3 => {
                                match &dimension_vec[&index_count + 1].to_string().parse::<u32>() {
                                    Ok(num2) => {
                                        present.height += num * 10;
                                        present.height += num2;
                                        two_digit = true;
                                    }
                                    Err(_) => present.height += num,
                                }
                            }
                            _ => println!("number filter error"),
                        };
                    }
                }
                None => not_number = true,
            }

            if dimension_count % 3 == 0 && !two_digit && !not_number {
                let side_a = present.length * present.width;
                let side_b = present.width * present.height;
                let side_c = present.height * present.length;

                let mut small_side = 0;
                if side_a <= side_b && side_a <= side_c {
                    small_side += side_a;
                } else if side_b <= side_a && side_b <= side_c {
                    small_side += side_b;
                } else {
                    small_side += side_c;
                }

                let mut present_paper = 0;
                present_paper += ((2 * side_a) + (2 * side_b) + (2 * side_c)) + small_side;

                total_paper += present_paper;

                total_ribbon += ribbon(&present);

                present = Present::default();
                total_count += 1;
            }
            if !two_digit && !not_number {
                dimension_count += 1;
            }
            index_count += 1;
        }
    }
    println!("The required amount of paper is: {}ft²", total_paper);
    println!("The required amount of ribbon is: {}ft", total_ribbon);
    println!("Count: {}", total_count);
}
