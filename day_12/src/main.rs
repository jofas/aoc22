use std::fs;
use std::collections::BinaryHeap;
use std::cmp::{Reverse, Ordering};

#[derive(PartialEq, Eq, Ord, Debug)]
struct PQE {
  p: u32,
  i: usize,
  j: usize,
}

impl PQE {
  fn new(p: u32, i: usize, j: usize) -> Self {
    Self { p, i, j }
  }
}

impl PartialOrd for PQE {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(Reverse(self.p).cmp(&Reverse(other.p)))
  }
}

fn elevation(c: char) -> u32 {
  if c.is_ascii_lowercase() {
    u32::from(c) - 96
  } else {
    unreachable!()
  }
}

fn update_priority(
  q: &mut BinaryHeap<PQE>,
  i: usize,
  j: usize,
  i_: usize,
  j_: usize,
  grid: &[[u32; 80]; 41],
  dist: &mut [[u32; 80]; 41],
  prev: &mut [[(usize, usize); 80]; 41],
) {
  let alt = if grid[i_][j_] <= grid[i][j] + 1 {
    dist[i][j].checked_add(1).unwrap_or(u32::MAX)
  } else {
    u32::MAX
  };

  if alt < dist[i_][j_] {
    dist[i_][j_] = alt;
    prev[i_][j_] = (i, j);
    q.push(PQE::new(alt, i_, j_));
  }
}

fn dist(
  start_i: usize,
  start_j: usize,
  end_i: usize,
  end_j: usize,
  grid: &[[u32; 80]; 41],
) -> u32 {
  let mut dist = [[u32::MAX; 80]; 41];
  dist[start_i][start_j] = 0;

  let mut prev = [[(0, 0); 80]; 41];

  let mut q = BinaryHeap::new();

  for (i, row) in dist.iter().enumerate() {
    for (j, d) in row.iter().enumerate() {
      q.push(PQE::new(*d, i, j));
    }
  }

  while q.len() > 0 {
    let u = q.pop().unwrap();

    // up
    if let Some(i_) = u.i.checked_sub(1) {
      update_priority(&mut q, u.i, u.j, i_, u.j, &grid, &mut dist, &mut prev);
    };

    // down
    if u.i + 1 < 41 {
      update_priority(&mut q, u.i, u.j, u.i + 1, u.j, &grid, &mut dist, &mut prev);
    };

    // left
    if let Some(j_) = u.j.checked_sub(1) {
      update_priority(&mut q, u.i, u.j, u.i, j_, &grid, &mut dist, &mut prev);
    };

    // right
    if u.j + 1 < 80 {
      update_priority(&mut q, u.i, u.j, u.i, u.j + 1, &grid, &mut dist, &mut prev);
    };
  }

  dist[end_i][end_j]
}

fn part_one() {
  let input = fs::read_to_string("input.txt").unwrap();

  let mut grid = [[0; 80]; 41];

  let mut start = (0, 0);
  let mut end = (0, 0);

  for (i, line) in input.lines().enumerate() {
    for (j, c) in line.chars().enumerate() {
      grid[i][j] = match c {
        'S' => {
          start = (i, j);
          elevation('a')
        }
        'E' => {
          end = (i, j);
          elevation('z')
        }
        _ => elevation(c),
      };
    }
  }

  println!("{}", dist(start.0, start.1, end.0, end.1, &grid));
}

fn part_two() {
  let input = fs::read_to_string("input.txt").unwrap();

  let mut grid = [[0; 80]; 41];

  let mut starts = Vec::new();
  let mut end = (0, 0);

  for (i, line) in input.lines().enumerate() {
    for (j, c) in line.chars().enumerate() {
      grid[i][j] = match c {
        'S' | 'a' => {
          starts.push((i, j));
          elevation('a')
        }
        'E' => {
          end = (i, j);
          elevation('z')
        }
        _ => elevation(c),
      };
    }
  }

  let mut min_d = u32::MAX;

  for start in starts {
    min_d = min_d.min(dist(start.0, start.1, end.0, end.1, &grid));
  }

  println!("{}", min_d);
}

fn main() {
  part_one();
  part_two();
}
