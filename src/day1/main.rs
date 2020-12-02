fn main() {
    let input = include_str!("input")
        .lines()
        .filter_map(|n| n.parse::<u32>().ok())
        .collect::<Vec<_>>();

    let (x, y) = find_entries(&input, 2020);
    println!("answer: {}", (x * y));
}

fn find_entries(input: &[u32], target: u32) -> (u32, u32) {
    for (i, x) in input[..input.len() - 1].iter().enumerate() {
        for y in &input[i..] {
            if x + y == target {
                return (*x, *y);
            }
        }
    }

    panic!("no matches found");
}
