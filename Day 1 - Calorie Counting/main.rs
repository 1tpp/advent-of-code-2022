use std::fs;

fn main() {
  let file_path = "input.txt";

  println!("In file {}", file_path);

  let contents = fs::read_to_string(file_path)
      .expect("Should have been able to read the file");

  // split string with \n\n
  let lines: Vec<&str> = contents.split("\n\n").collect();

  // split each line with \n
  let lines: Vec<Vec<&str>> = lines.iter().map(|line| line.split("\n").collect()).collect();

  // convert string to int and skip empty lines
  let numbers: Vec<Vec<i32>> = lines.iter().map(|line| line.iter().filter(|&x| x != &"").map(|x| x.parse::<i32>().unwrap()).collect()).collect();
  
  // sum of each array
  let sum: Vec<i32> = numbers.iter().map(|line| line.iter().sum()).collect();

  // max value of sum
  let max = sum.iter().max().unwrap();

  println!("Max sum: {}", max);
}