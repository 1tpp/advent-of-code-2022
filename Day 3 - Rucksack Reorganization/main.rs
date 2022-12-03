fn main() {
    let input_data = include_bytes!("./input.txt");

    // Part 1
    println!(
        "{:?}",
        input_data
            .split(|byte| *byte == b'\n')
            .map(|line| line.split_at(line.len() / 2))
            .map(|(a, b)| 
                b.iter().filter(|byte| a.contains(byte))
                .map(|byte| if *byte >= b'a' {
                    (byte - b'a') as i16 + 1
                } else {
                    (byte - b'A') as i16 + 27
                })
                .next()
                .unwrap()
            ).sum::<i16>(),
    );

    // Part 2
    println!(
        "{:?}",
        input_data
            .split(|byte| *byte == b'\n')
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|line| line[0].iter().filter(|byte| line[1].contains(byte) && line[2].contains(byte)).next().unwrap())
            .map(|byte| if *byte >= b'a' {
                (byte - b'a') as i16 + 1
            } else {
                (byte - b'A') as i16 + 27
            }).sum::<i16>(),
    );
}