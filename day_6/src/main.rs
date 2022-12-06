use std::fs;
use std::collections::HashSet;

fn part_one() {
  let stream = fs::read_to_string("input.txt").unwrap();

  for i in 4..stream.len() {
    let s: HashSet<char> = HashSet::from_iter(stream[i-4..i].chars());

    if s.len() == 4 {
      println!("{}", i);
      return
    }
  }
}

fn part_two() {
  let stream = fs::read_to_string("input.txt").unwrap();

  for i in 14..stream.len() {
    let s: HashSet<char> = HashSet::from_iter(stream[i-14..i].chars());

    if s.len() == 14 {
      println!("{}", i);
      return
    }
  }
}

fn main() {
    part_one();
    part_two();
}
