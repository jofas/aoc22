use std::fs;
use std::ops::RangeInclusive;

fn parse_range(s: &str) -> RangeInclusive<u32> {
  match s.split("-").collect::<Vec<&str>>()[..] {
    [start, end] => RangeInclusive::new(start.parse().unwrap(), end.parse().unwrap()),
    _ => unreachable!(),
  }
}

fn contains(rhs: &RangeInclusive<u32>, lhs: &RangeInclusive<u32>) -> bool {
  rhs.contains(&lhs.start()) && rhs.contains(&lhs.end())
}

fn overlaps(rhs: &RangeInclusive<u32>, lhs: &RangeInclusive<u32>) -> bool {
  rhs.contains(&lhs.start()) || rhs.contains(&lhs.end())
}

fn part_one() {
  let pairs = fs::read_to_string("input.txt").unwrap();

  let mut sum = 0;

  for line in pairs.lines() {
    let (s1, s2) = match line.split(",").collect::<Vec<&str>>()[..] {
      [s1, s2] => (s1, s2),
      _ => unreachable!()
    };

    let r1 = parse_range(s1);
    let r2 = parse_range(s2);

    if contains(&r1, &r2) || contains(&r2, &r1) {
      sum += 1;
    }
  }

  println!("{}", sum);
}

fn part_two() {
  let pairs = fs::read_to_string("input.txt").unwrap();

  let mut sum = 0;

  for line in pairs.lines() {
    let (s1, s2) = match line.split(",").collect::<Vec<&str>>()[..] {
      [s1, s2] => (s1, s2),
      _ => unreachable!()
    };

    let r1 = parse_range(s1);
    let r2 = parse_range(s2);

    if overlaps(&r1, &r2) || overlaps(&r2, &r1) {
      sum += 1;
    }
  }

  println!("{}", sum);
}

fn main() {
  part_one();
  part_two();
}
