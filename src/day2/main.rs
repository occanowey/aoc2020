use regex::Regex;

fn main() {
    let entry_pattern = Regex::new("(\\d+)\\-(\\d+) (\\w): (\\w+)").unwrap();

    let valid_passwords = include_str!("input")
        .lines()
        .filter_map(|line| {
            entry_pattern.captures(line).map(|parts| {
                (
                    parts[1].parse().unwrap(),
                    parts[2].parse().unwrap(),
                    parts[3].parse().unwrap(),
                    parts[4].to_owned(),
                )
            })
        })
        .filter(is_valid_entry)
        .count();

    println!("answer: {}", valid_passwords);
}

fn is_valid_entry((pos1, pos2, letter, password): &(usize, usize, char, String)) -> bool {
    let password_chars = password.chars().collect::<Vec<_>>();

    let pos1_has_letter = password_chars[*pos1 - 1] == *letter;
    let pos2_has_letter = password_chars[*pos2 - 1] == *letter;

    pos1_has_letter ^ pos2_has_letter
}