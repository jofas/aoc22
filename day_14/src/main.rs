use std::fs;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
  Rock,
  Air,
  Sand,
}

impl Cell {
  fn is_air(&self) -> bool {
    self == &Self::Air
  }
}

impl fmt::Display for Cell {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Rock => write!(f, "{}", '#'),
      Self::Air => write!(f, "{}", '.'),
      Self::Sand => write!(f, "{}", 'o'),
    }
  }
}

#[allow(dead_code)]
fn display_matrix(v: &[Vec<Cell>]) {
  for row in v {
    for cell in row {
      print!("{}", cell);
    }
    println!();
  }
}

fn part_one() {
  let input = fs::read_to_string("input.txt").unwrap();

  let lower = 0;
  let mut upper = 0;
  let mut left = usize::MAX;
  let mut right = 0;

  let mut paths: Vec<Vec<(usize, usize)>> = Vec::new();

  for line in input.lines() {
    let line = line.trim();

    paths.push(Vec::new());

    for point in line.split(" -> ") {
      let mut point = point.split(',');

      let x = point.next().unwrap().parse().unwrap();
      let y = point.next().unwrap().parse().unwrap();

      left = left.min(x);
      right = right.max(x);

      upper = upper.max(y);

      let i = paths.len() - 1;
      paths[i].push((x, y));
    }
  }

  let mut matrix = vec![vec![Cell::Air; right - left + 1]; upper - lower + 1];

  for path in paths {
    for i in 1..path.len() {
      let (x1, y1) = path[i - 1];
      let (x2, y2) = path[i];

      let (x1, x2) = (x1.min(x2), x1.max(x2));
      let (y1, y2) = (y1.min(y2), y1.max(y2));

      assert!(x1 == x2 || y1 == y2);

      for x in x1..=x2 {
        for y in y1..=y2 {
          matrix[y - lower][x - left] = Cell::Rock;
        }
      }
    }
  }

  let mut rested_sand = 0;

  'outer: loop {
    let mut y = 0 - lower;
    let mut x = 500 - left;

    loop {
      if y + 1 >= matrix.len() {
        break 'outer;
      }

      if matrix[y + 1][x].is_air() {
        y += 1;
        continue;
      }

      if x.checked_sub(1).is_none() {
        break 'outer;
      }

      if matrix[y + 1][x - 1].is_air() {
        y += 1;
        x -= 1;
        continue;
      }

      if x + 1 >= matrix[0].len() {
        break 'outer;
      }

      if matrix[y + 1][x + 1].is_air() {
        y += 1;
        x += 1;
        continue;
      }

      matrix[y][x] = Cell::Sand;
      rested_sand += 1;
      break;
    }
  }

  println!("{}", rested_sand);
}

fn part_two() {
  let input = fs::read_to_string("input.txt").unwrap();

  const MOVE_RIGHT: usize = 500;

  let lower = 0;
  let mut upper = 0;
  let mut left = usize::MAX;
  let mut right = 0;

  let mut paths: Vec<Vec<(usize, usize)>> = Vec::new();

  for line in input.lines() {
    let line = line.trim();

    paths.push(Vec::new());

    for point in line.split(" -> ") {
      let mut point = point.split(',');

      let x = point.next().unwrap().parse().unwrap();
      let y = point.next().unwrap().parse().unwrap();

      left = left.min(x);
      right = right.max(x);

      upper = upper.max(y);

      let i = paths.len() - 1;
      paths[i].push((x, y));
    }
  }

  let mut matrix = vec![vec![Cell::Air; 2048]; upper + 3];

  for path in paths {
    for i in 1..path.len() {
      let (x1, y1) = path[i - 1];
      let (x2, y2) = path[i];

      let (x1, x2) = (x1.min(x2), x1.max(x2));
      let (y1, y2) = (y1.min(y2), y1.max(y2));

      assert!(x1 == x2 || y1 == y2);

      for x in x1..=x2 {
        for y in y1..=y2 {
          matrix[y - lower][x - left + MOVE_RIGHT] = Cell::Rock;
        }
      }
    }
  }

  for cell in &mut matrix[upper - lower + 2] {
    *cell = Cell::Rock;
  }

  let mut rested_sand = 0;

  loop {
    let mut y = 0 - lower;
    let mut x = 500 - left + MOVE_RIGHT;

    if matrix[y][x] == Cell::Sand {
      break;
    }

    loop {
      if matrix[y + 1][x].is_air() {
        y += 1;
        continue;
      }

      if matrix[y + 1][x - 1].is_air() {
        y += 1;
        x -= 1;
        continue;
      }

      if matrix[y + 1][x + 1].is_air() {
        y += 1;
        x += 1;
        continue;
      }

      matrix[y][x] = Cell::Sand;
      rested_sand += 1;
      break;
    }
  }

  println!("{}", rested_sand);
}

fn main() {
  part_one();
  part_two();
}
