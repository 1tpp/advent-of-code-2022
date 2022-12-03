fn main() {
    let input_data = include_bytes!("./input.txt");
    // Part 1
    println!(
        "{}",
            input_data
            .split(|byte| *byte == b'\n')
            .map(|line| ((line[0] - b'A') as i16, (line[2] - b'X') as i16,))
            .map(|(a, b)| 1 + b + 3 * ((1 + b - a).rem_euclid(3)))
            .sum::<i16>(),
    );

    // Part 2
    println!(
        "{}",
            input_data
            .split(|byte| *byte == b'\n')
            .map(|line| ((line[0] - b'A') as i16, (line[2] - b'X') as i16,))
            .map(|(a, b)| 1 + b * 3 + (2 + a + b) % 3)
            .sum::<i16>(),
    );
}
