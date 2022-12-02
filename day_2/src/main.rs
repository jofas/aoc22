use std::fs;

#[derive(Debug)]
enum Move {
  Rock,
  Paper,
  Scissors,
}

impl Move {
  pub fn new(s: &str) -> Self {
    match s {
      "A" | "X" => Move::Rock,
      "B" | "Y" => Move::Paper,
      "C" | "Z" => Move::Scissors,
      _ => unreachable!(),
    }
  }

  pub fn score(&self) -> u32 {
    match self {
      Self::Rock => 1,
      Self::Paper => 2,
      Self::Scissors => 3
    }
  }

  pub fn result(&self, other: &Move) -> u32 {
    match (self, other) {
      (Move::Rock, Move::Rock) => 3,
      (Move::Rock, Move::Paper) => 6,
      (Move::Rock, Move::Scissors) => 0,

      (Move::Paper, Move::Rock) => 0,
      (Move::Paper, Move::Paper) => 3,
      (Move::Paper, Move::Scissors) => 6,

      (Move::Scissors, Move::Rock) => 6,
      (Move::Scissors, Move::Paper) => 0,
      (Move::Scissors, Move::Scissors) => 3,
    }
  }
}

#[derive(Debug)]
enum Strategy {
  Lose,
  Draw,
  Win,
}

impl Strategy {
  pub fn new(s: &str) -> Self {
    match s {
      "X" => Self::Lose,
      "Y" => Self::Draw,
      "Z" => Self::Win,
      _ => unreachable!(),
    }
  }

  pub fn into_move(&self, m: &Move) -> Move {
    match (self, m) {
      (Self::Lose, Move::Rock) => Move::Scissors,
      (Self::Lose, Move::Paper) => Move::Rock,
      (Self::Lose, Move::Scissors) => Move::Paper,

      (Self::Draw, Move::Rock) => Move::Rock,
      (Self::Draw, Move::Paper) => Move::Paper,
      (Self::Draw, Move::Scissors) => Move::Scissors,

      (Self::Win, Move::Rock) => Move::Paper,
      (Self::Win, Move::Paper) => Move::Scissors,
      (Self::Win, Move::Scissors) => Move::Rock,
    }
  }
}

fn part_one() {
  let moves = fs::read_to_string("input.txt").unwrap();

  let mut score = 0;

  for line in moves.lines() {
    let l: [Move; 2] = line.split(" ").map(Move::new).collect::<Vec<Move>>().try_into().unwrap();

    score += l[1].score() + l[0].result(&l[1]);
  }

  println!("{}", score);
}

fn part_two() {
  let moves = fs::read_to_string("input.txt").unwrap();

  let mut score = 0;

  for line in moves.lines() {
    let line = line.split(" ").collect::<Vec<&str>>();

    let (m, s) = (Move::new(line[0]), Strategy::new(line[1]));

    let s = s.into_move(&m);

    score += s.score() + m.result(&s);
  }

  println!("{}", score);
}

fn main() {
  part_one();
  part_two();
}
