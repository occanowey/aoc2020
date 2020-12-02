fn main() {
    let input = include_str!("input")
        .lines()
        .filter_map(|n| n.parse::<u32>().ok())
        .collect::<Vec<_>>();

    let (x, y, z) = find_entries(&input, 2020);
    println!("answer: {}", (x * y * z));
}

fn find_entries(input: &[u32], target: u32) -> (u32, u32, u32) {
    for (i, x) in input[..input.len() - 1].iter().enumerate() {
        for y in &input[i..] {
            for z in &input[i..] {
                if x + y + z == target {
                    return (*x, *y, *z);
                }
            }
        }
    }

    panic!("no matches found");
}
