use num::integer::div_rem;

use std::fs;

const D: usize = 99;

fn part_one() {
  let input = fs::read_to_string("input.txt").unwrap();

  let mut trees: [[u8;D];D] = [[0;D];D];

  for (i, c) in input.replace('\n', "").chars().enumerate() {
    let (y, x) = div_rem(i, D);
    trees[x][y] = c.to_digit(10).unwrap().try_into().unwrap();
  }

  let mut visible: [[bool;D];D] = [[false;D];D];

  for i in 0..D {
    for j in 0..D {
      if i == 0 || i == D - 1 {
        visible[i][j] = true;
      }

      if j == 0 || j == D - 1 {
        visible[i][j] = true;
      }

      let mut tallest = true;

      for x in 0..i {
        if trees[x][j] >= trees[i][j] {
          tallest = false;
        }
      }

      if tallest {
        visible[i][j] = true;
        continue;
      } else {
        tallest = true;
      }

      for x in i+1..D {
        if trees[x][j] >= trees[i][j] {
          tallest = false;
        }
      }

      if tallest {
        visible[i][j] = true;
        continue;
      } else {
        tallest = true;
      }

      for y in 0..j {
        if trees[i][y] >= trees[i][j] {
          tallest = false;
        }
      }

      if tallest {
        visible[i][j] = true;
        continue;
      } else {
        tallest = true;
      }

      for y in j+1..D {
        if trees[i][y] >= trees[i][j] {
          tallest = false;
        }
      }

      if tallest {
        visible[i][j] = true;
      }
    }
  }

  let mut count = 0;

  for i in 0..D {
    for j in 0..D {
      if visible[i][j] {
        count += 1;
      }
    }
  }

  println!("{}", count);
}

fn part_two() {
  let input = fs::read_to_string("input.txt").unwrap();

  let mut trees: [[u8;D];D] = [[0;D];D];

  for (i, c) in input.replace('\n', "").chars().enumerate() {
    let (y, x) = div_rem(i, D);
    trees[x][y] = c.to_digit(10).unwrap().try_into().unwrap();
  }

  let mut most_trees = 0;

  for i in 0..D {
    for j in 0..D {
      let h = trees[i][j];

      let mut n = 0;
      let mut e = 0;
      let mut s = 0;
      let mut w = 0;

      if j > 0 {
        for y in (0..j).rev() {
          n += 1;

          if trees[i][y] >= h {
            break;
          }
        }
      }

      for y in j+1..D {
        s += 1;

        if trees[i][y] >= h {
          break;
        }
      }

      if i > 0 {
        for x in (0..i).rev() {
          w += 1;

          if trees[x][j] >= h {
            break;
          }
        }
      }

      for x in i+1..D {
        e += 1;

        if trees[x][j] >= h {
          break;
        }
      }

      most_trees = most_trees.max(n * e * s * w);
    }
  }

  println!("{}", most_trees);
}

fn main() {
  part_one();
  part_two();
}
