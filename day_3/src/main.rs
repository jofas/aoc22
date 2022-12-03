use std::fs;
use std::collections::HashSet;

fn priority(c: char) -> u32 {
  if c.is_ascii_uppercase() {
    u32::from(c) - 38
  } else if c.is_ascii_lowercase() {
    u32::from(c) - 96
  } else {
    unreachable!()
  }
}

fn part_one() {
  let rucksacks = fs::read_to_string("input.txt").unwrap();

  let mut sum = 0;

  for line in rucksacks.lines() {
    let c1 = line[0..line.len() / 2].chars();
    let c2 = line[line.len() / 2 .. line.len()].chars();

    let c1: HashSet<char> = HashSet::from_iter(c1);
    let c2: HashSet<char> = HashSet::from_iter(c2);

    let intersection: HashSet<&char> = c1.intersection(&c2).collect();

    assert_eq!(intersection.len(), 1);

    sum += priority(**intersection.iter().next().unwrap());

  }

  println!("{}", sum);
}

fn part_two() {
  let rucksacks = fs::read_to_string("input.txt").unwrap();

  let mut sum = 0;

  let lines: Vec<&str> = rucksacks.lines().collect();

  for group in lines.chunks_exact(3) {
    if let [r1, r2, r3] = group {
      let r1: HashSet<char> = HashSet::from_iter(r1.chars());
      let r2: HashSet<char> = HashSet::from_iter(r2.chars());
      let r3: HashSet<char> = HashSet::from_iter(r3.chars());

      let intersection: HashSet<char> = r1.intersection(&r2)
        .map(|x| *x)
        .collect();

      let intersection: HashSet<char> = intersection.intersection(&r3)
        .map(|x| *x)
        .collect();

      sum += priority(intersection.into_iter().next().unwrap());
    } else {
      unreachable!();
    }
  }

  println!("{}", sum);
}

fn main() {
  part_one();
  part_two();
}

#[cfg(test)]
mod tests {
  use super::priority;

  #[test]
  fn ascii_value() {
    assert_eq!(u32::from('A'), 65);
    assert_eq!(u32::from('Z'), 90);

    assert_eq!(u32::from('a'), 97);
    assert_eq!(u32::from('z'), 122);
  }

  #[test]
  fn test_priority() {
    assert_eq!(priority('A'), 27);
    assert_eq!(priority('Z'), 52);

    assert_eq!(priority('a'), 1);
    assert_eq!(priority('z'), 26);
  }
}
