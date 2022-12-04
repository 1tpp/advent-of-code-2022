fn main() {
  let input_text = include_str!("./input.txt");

  let text_splited = input_text
    .lines()
    .map(|line| {
      let (left, right) = line.split_once(',').unwrap();
      let ((left_start, left_end), (right_start, right_end)) = (left.split_once('-').unwrap(), right.split_once('-').unwrap());
      
      (left_start.parse::<u8>().unwrap(), left_end.parse::<u8>().unwrap(), right_start.parse::<u8>().unwrap(), right_end.parse::<u8>().unwrap())
    });

  // Part 1
  println!("{:?}", text_splited.clone().filter(|(left_start, left_end, right_start, right_end)| 
      (left_start >= right_start && left_end <= right_end) || 
      (left_start <= right_start && left_end >= right_end))
    .count()  
  );

  // Part 2
  println!("{:?}", text_splited.clone().filter(|(left_start, left_end, right_start, right_end)| 
      (left_start >= right_start && left_start <= right_end) || 
      (left_end >= right_start && left_end <= right_end) ||
      (left_start <= right_start && left_end >= right_start) ||
      (left_start <= right_end && left_end >= right_end))
    .count()  
  );
}