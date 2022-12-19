use std::fs;
use std::ops::RangeInclusive;

use rayon::prelude::*;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser);

use parser::InputParser;

lazy_static::lazy_static! {
  static ref PARSER: InputParser = InputParser::new();
}

fn union(a: &mut RangeInclusive<i32>, b: &RangeInclusive<i32>) -> bool {
  // fully contained ABBA
  if a.contains(b.start()) && a.contains(b.end()) {
    return true;
  }

  // fully contained BAAB
  if b.contains(a.start()) && b.contains(a.end()) {
    *a = b.clone();
    return true;
  }

  // partially contained ABAB
  if a.contains(b.start()) && !a.contains(b.end()) {
    *a = *a.start() ..= *b.end();
    return true;
  }

  // partially contained BABA
  if a.contains(b.end()) && !a.contains(b.start()) {
    *a = *b.start() ..= *a.end();
    return true;
  }

  false
}

fn union_vec(v: &[RangeInclusive<i32>]) -> Vec<RangeInclusive<i32>> {
  let mut res: Vec<RangeInclusive<i32>> = vec![];

  for range in v {
    let mut range_contained = false;

    for covered in &mut res {
      if union(covered, &range) {
        range_contained = true;
        break;
      }
    }

    if !range_contained {
      res.push(range.clone());
    }
  }

  res
}

fn row_coverage(d: &[(i32, i32, i32, i32)], line: i32) -> Vec<RangeInclusive<i32>> {
  let dm: Vec<i32> = d
    .iter()
    .map(|x| (x.0 - x.2).abs() + (x.1 - x.3).abs())
    .collect();

  let dy: Vec<i32> = d
    .iter()
    .map(|x| (x.1 - line).abs())
    .collect();

  let dmdy: Vec<RangeInclusive<i32>> = dm
    .iter()
    .zip(dy.iter())
    .enumerate()
    .filter_map(|(i, (dm, dy))| {
      if (dm - dy) > 0 {
        Some(d[i].0 - dm + dy ..= d[i].0 + dm - dy)
      } else {
        None
      }
    })
    .collect();

  let mut cf = union_vec(&dmdy);

  loop {
    let v = union_vec(&cf);

    if v == cf {
      break;
    } else {
      cf = v;
    }
  }

  cf
}

fn part_one() {
  let input = fs::read_to_string("input.txt").unwrap();
  let lines = PARSER.parse(&input).unwrap();

  const LINE: i32 = 2_000_000;

  let cf = row_coverage(&lines, LINE);

  let mut scf = 0;

  for c in &cf {
    let s = c.end() - c.start();

    scf += s;
  }

  println!("{}", scf);
}

fn part_two() {
  let input = fs::read_to_string("input.txt").unwrap();
  let lines = PARSER.parse(&input).unwrap();

  (0..=4_000_000).into_par_iter().for_each(|row| {
    let mut cf = row_coverage(&lines, row);

    for r in &mut cf {
      *r = *r.start().max(&0) ..= *r.end().min(&4_000_000);
    }

    if cf[0] != (0 ..= 4_000_000) {
      println!("{}, {:?}", row, cf);
    }
  });

  // answer: 12525726647448 (2_647_448 (row) + 3_131_431 (col) * 4_000_000)
}

fn main() {
  part_one();
  part_two();
}
