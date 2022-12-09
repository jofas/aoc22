use std::fs;
use std::str::FromStr;
use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Pos(i32, i32);

impl Pos {
  fn add(&self, other: &Pos) -> Self {
    Self(self.0 + other.0, self.1 + other.1)
  }

  fn delta(&self, other: &Pos) -> Self {
    Self(self.0 - other.0, self.1 - other.1)
  }

  fn abs(&self) -> Self {
    Self(self.0.abs(), self.1.abs())
  }

  fn max(&self) -> i32 {
    self.0.max(self.1)
  }

  fn norm(&self) -> Self {
    let zero = if self.0 == 0 { 1 } else { self.0.abs() };
    let one = if self.1 == 0 { 1 } else { self.1.abs() };

    Self(self.0 / zero, self.1 / one)
  }
}

enum Move {
  Up(i32),
  Down(i32),
  Left(i32),
  Right(i32),
}

impl Move {
  fn pos(&self) -> Pos {
    match self {
      Self::Up(_) => Pos(1, 0),
      Self::Down(_) => Pos(-1, 0),
      Self::Left(_) => Pos(0, -1),
      Self::Right(_) => Pos(0, 1),
    }
  }

  fn amount(&self) -> i32 {
    match self {
      Self::Up(u) => *u,
      Self::Down(d) => *d,
      Self::Left(l) => *l,
      Self::Right(r) => *r,
    }
  }
}

impl FromStr for Move {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut s = s.split(' ');

    let cmd = s.next().ok_or(())?;
    let amnt: i32 = s.next().ok_or(())?.parse().map_err(|_| ())?;

    match cmd {
      "U" => Ok(Self::Up(amnt)),
      "D" => Ok(Self::Down(amnt)),
      "L" => Ok(Self::Left(amnt)),
      "R" => Ok(Self::Right(amnt)),
      _ => Err(()),
    }
  }
}

fn part_one() {
  let input = fs::read_to_string("input.txt").unwrap();

  let mut pos = HashSet::new();

  let mut knots = [Pos(0i32, 0i32); 2];

  pos.insert(knots[knots.len() - 1]);

  for line in input.lines() {
    let m = Move::from_str(line).unwrap();

    for _ in 0..m.amount() {
      knots[0] = knots[0].add(&m.pos());

      for i in 1..knots.len() {
        let delta = knots[i-1].delta(&knots[i]);

        if delta.abs().max() > 1 {
          knots[i] = knots[i].add(&delta.norm());
        }
      }

      pos.insert(knots[knots.len() - 1]);
    }
  }

  println!("{}", pos.len());
}

fn part_two() {
  let input = fs::read_to_string("input.txt").unwrap();

  let mut pos = HashSet::new();

  let mut knots = [Pos(0i32, 0i32); 10];

  pos.insert(knots[knots.len() - 1]);

  for line in input.lines() {
    let m = Move::from_str(line).unwrap();

    for _ in 0..m.amount() {
      knots[0] = knots[0].add(&m.pos());

      for i in 1..knots.len() {
        let delta = knots[i-1].delta(&knots[i]);

        if delta.abs().max() > 1 {
          knots[i] = knots[i].add(&delta.norm());
        }
      }

      pos.insert(knots[knots.len() - 1]);
    }
  }

  println!("{}", pos.len());
}

fn main() {
  part_one();
  part_two();
}
