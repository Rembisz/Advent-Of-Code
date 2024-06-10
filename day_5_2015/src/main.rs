use std::fs;

fn main() {
    let file_contents = match fs::read_to_string("src/strings.txt") {
        Ok(contents) => contents, // Reads file contents and assigns it to a string. Handles bad file error.
        Err(e) => panic!("Invalid input file. (Error: {})", e),
    };

    let mut naughty_strings: Vec<&str> = vec![]; // Creates vectors to sort nice and naughty strings respectively.
    let mut nice_strings: Vec<&str> = vec![];

    for line in file_contents.lines() {
        // Iterates over each line as an instance
        if line.contains("ab") || line.contains("cd") || line.contains("pq") || line.contains("xy")
        {
            naughty_strings.push(line); // Sorts lines that have ab, cd, pq, or xy
            continue;
        }

        let mut double_check: bool = false;
        for i in 1..(line.len()) {
            let current_char = line.chars().nth(i).unwrap(); // Breaks line into chars and assigns char at index "i" to variable
            let last_char = line.chars().nth(i - 1).unwrap(); // index "i - 1" for equation

            if current_char == last_char {
                double_check = true; // Sorts lines that have double letters by equating indexes
                break;
            }
        }

        if !double_check {
            naughty_strings.push(line); // Sorts lines without doubles
            continue;
        }

        let mut vowel_count = 0;
        for char in line.chars() {
            if is_vowel(&char) {
                vowel_count += 1; // Counts the vowels in line
            }
        }
        if vowel_count >= 3 {
            nice_strings.push(line);
            continue;
        } else {
            naughty_strings.push(line); // Sorts lines by vowel count into respective vector
        }
    }
    println!(
        "There are {} Nice Strings and {} Naughty Strings for this file under ruleset 1.",
        nice_strings.len(),
        naughty_strings.len() // Prints the length of each sort vector
    );

    // PART 2 //

    let mut naughty_strings: Vec<&str> = vec![]; // Shadows sort vectors
    let mut nice_strings: Vec<&str> = vec![];

    for line in file_contents.lines() {
        // Iterates over each line as an instance
        let mut pair = false;
        let mut triple_couple = false;
        'pair_loop: for i in 1..(line.len()) {
            // Ensures every pair is checked across the line
            let mut line_vec: Vec<char> = line.chars().collect(); // line as vector

            let second_char = line.chars().nth(i).unwrap();
            let first_char = line.chars().nth(i - 1).unwrap();
            let mut letter_pair = String::new();
            letter_pair.push(first_char);
            letter_pair.push(second_char);
            // Creates a letter pair to equate across the line

            line_vec[i] = '_';
            line_vec[i - 1] = '_'; // Removes letter pair from line as vector to avoid overlapping

            let new_line: String = line_vec.into_iter().collect(); // Creates a new line from vector without the pair

            for i in 1..(new_line.len()) {
                let second_check_char = new_line.chars().nth(i).unwrap();
                let first_check_char = new_line.chars().nth(i - 1).unwrap();
                let mut check_pair = String::new();
                check_pair.push(first_check_char);
                check_pair.push(second_check_char);
                // Creates a check pair of letters across the new line to equate with original pair

                if letter_pair == check_pair {
                    pair = true;
                    continue 'pair_loop; // Establishes pairs by equating check and letter
                }
            }
        }

        for i in 2..(line.len()) {
            let last_char = line.chars().nth(i).unwrap();
            let first_char = line.chars().nth(i - 2).unwrap();
            // Iterates over 3 indexs for every step in line and saves first and last char

            if first_char == last_char {
                triple_couple = true; // Establishes triple-couples by equating the first and last char
                continue;
            }
        }

        if pair && triple_couple {
            nice_strings.push(line);
        } else {
            naughty_strings.push(line); // Sorts lines into respective vector
        }
    }

    println!(
        "There are {} Nice Strings and {} Naughty Strings for this file under ruleset 2.",
        nice_strings.len(),
        naughty_strings.len() // Prints the length of each sort vector
    );
}

fn is_vowel(char: &char) -> bool {
    matches!(*char, 'a' | 'e' | 'i' | 'o' | 'u')
}
