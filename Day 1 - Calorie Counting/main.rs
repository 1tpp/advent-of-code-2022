fn main() {
  let input_text = include_str!("./input.txt");

  // part 1
  println!("{}",input_text.split("\n\n").map(|e| e.lines().map(|c| c.parse::<u32>().unwrap()).sum::<u32>()).max().unwrap(),);
  
  // part 2
  let mut calories = input_text.split("\n\n").map(|e| e.lines().map(|c| c.parse::<u32>().unwrap()).sum()).collect::<Vec<u32>>();
  calories.sort_unstable();
  println!("{}", calories.into_iter().rev().take(3).sum::<u32>());
}