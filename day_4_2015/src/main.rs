use md5;

fn main() {
    let input = include_str!("../input.txt");
    println!(
        "(Part 1) Smallest number: {:?}",
        find_md5_suffix(input, "00000")
    );
    println!(
        "(Part 2) Smallest number: {:?}",
        find_md5_suffix(input, "000000")
    );
}

fn find_md5_suffix(input_base: &str, start_pattern: &str) -> u32 {
    let mut i = 0;
    loop {
        i += 1;
        let hash = md5::compute(input_base.to_string() + (i.to_string().as_str()));
        let hash_literal = format!("{:x}", hash);
        let removed: String = hash_literal.chars().take(start_pattern.len()).collect();
        if removed == start_pattern {
            break;
        }
    }
    i
}
