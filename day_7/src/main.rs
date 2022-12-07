use regex::Regex;

use lazy_static::lazy_static;

use map_macro::map;

use std::fs;

lazy_static! {
  static ref CD: Regex = Regex::new(r"^\$ cd (?P<d>.+)$").unwrap();
  static ref LS: Regex = Regex::new(r"^\$ ls$").unwrap();
  static ref FILE: Regex = Regex::new(r"^(?P<s>\d+).+$").unwrap();
  static ref DIR: Regex = Regex::new(r"^dir (?P<d>.+)$").unwrap();
}

struct Cursor<'a> {
  path: Vec<&'a str>,
}

impl<'a> Cursor<'a> {
  fn new() -> Self {
    Self { path: Vec::new() }
  }

  fn cd(&mut self, dir: &'a str) {
    if dir == ".." {
      self.path.pop();
    } else {
      self.path.push(dir);
    }
  }

  fn path(&self) -> String {
    if self.path.len() == 0 {
      "/".to_owned()
    } else {
      self.path.iter().fold(String::new(), |acc, d| format!("{}/{}", acc, d))
    }
  }

  fn path_of(&self, dir: &'a str) -> String {
    if self.path.len() == 0 {
      format!("/{}", dir)
    } else {
      format!("{}/{}", self.path(), dir)
    }
  }
}


fn part_one() {
  let ops = fs::read_to_string("input.txt").unwrap();

  let mut dirs = map!{
    "/".to_owned() => 0,
  };

  let mut cursor = Cursor::new();

  for line in ops.lines().skip(2) {
    if CD.is_match(line) {
      let d = CD.captures(line).unwrap().name("d").unwrap().as_str();
      cursor.cd(d);
    } else if LS.is_match(line) {
      // do nothing
    } else if FILE.is_match(line) {
      let c = FILE.captures(line).unwrap();

      let s: u64 = c.name("s").unwrap().as_str().parse().unwrap();

      *dirs.get_mut(&cursor.path()).unwrap() += s;
    } else if DIR.is_match(line) {
      let d = DIR.captures(line).unwrap().name("d").unwrap().as_str();
      dirs.insert(cursor.path_of(d), 0);
    } else {
      unreachable!()
    }
  }

  let clone = dirs.clone();

  for dir in clone.keys() {
    for (k, v) in clone.iter() {
      if k.starts_with(dir) && dir != k {
        *dirs.get_mut(dir).unwrap() += v;
      }
    }
  }

  let mut sum = 0;

  for v in dirs.values() {
    if *v <= 100_000 {
      sum += v;
    }
  }

  println!("{}", sum);
}

fn part_two() {
  let ops = fs::read_to_string("input.txt").unwrap();

  let mut dirs = map!{
    "/".to_owned() => 0_u64,
  };

  let mut cursor = Cursor::new();

  for line in ops.lines().skip(2) {
    if CD.is_match(line) {
      let d = CD.captures(line).unwrap().name("d").unwrap().as_str();
      cursor.cd(d);
    } else if LS.is_match(line) {
      // do nothing
    } else if FILE.is_match(line) {
      let c = FILE.captures(line).unwrap();

      let s: u64 = c.name("s").unwrap().as_str().parse().unwrap();

      *dirs.get_mut(&cursor.path()).unwrap() += s;
    } else if DIR.is_match(line) {
      let d = DIR.captures(line).unwrap().name("d").unwrap().as_str();
      dirs.insert(cursor.path_of(d), 0);
    } else {
      unreachable!()
    }
  }

  let clone = dirs.clone();

  for dir in clone.keys() {
    for (k, v) in clone.iter() {
      if k.starts_with(dir) && dir != k {
        let d = dirs.get_mut(dir).unwrap();
        *d = d.checked_add(*v).unwrap();
      }
    }
  }

  let space = 70_000_000;
  let space_needed = 30_000_000;
  let available_space = space - dirs["/"];
  let space_to_free = space_needed - available_space;


  let mut dir_size = dirs["/"];

  for v in dirs.values() {
    if *v >= space_to_free {
      dir_size = dir_size.min(*v);
    }
  }

  println!("{}", dir_size);
}

fn main() {
  part_one();
  part_two();
}
