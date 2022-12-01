use std::fs;
use std::cmp::Reverse;

fn main() {
  let calories = fs::read_to_string("input.txt").unwrap();

  let mut cs = Vec::new();
  let mut current_calories = 0;

  for line in calories.lines() {
    if line == "" {
      cs.push(Reverse(current_calories));
      current_calories = 0;
    } else {
      current_calories += line.parse::<u64>().unwrap();
    }
  }

  cs.sort();

  println!("{:?}", cs[0..3].iter().map(|x| x.0).sum::<u64>());
}
