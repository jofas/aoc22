use std::fs;
use std::cmp::Ordering;

use lalrpop_util::lalrpop_mod;

#[derive(Clone, PartialEq, Eq, Ord)]
pub enum Value {
  List(Vec<Value>),
  Number(u32),
}

impl PartialOrd for Value {
  fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
    match (self, other) {
      (Self::List(l1), Self::List(l2)) => {
        let mut l1 = l1.iter();
        let mut l2 = l2.iter();

        loop {
          let v1 = l1.next();
          let v2 = l2.next();

          match (v1, v2) {
            (None, Some(_)) => return Some(Ordering::Less),
            (Some(_), None) => return Some(Ordering::Greater),
            (Some(v1), Some(v2)) => {
              if let Some(res) = v1.partial_cmp(v2) {
                return Some(res);
              }
            }
            (None, None) => return None,
          }
        }
      },
      (Self::List(_), Self::Number(_)) => {
        self.partial_cmp(&Value::List(vec![other.clone()]))
      },
      (Self::Number(_), Self::List(_)) => {
        Value::List(vec![self.clone()]).partial_cmp(&other)
      },
      (Self::Number(n1), Self::Number(n2)) => {
        match n1.cmp(&n2) {
          Ordering::Less => Some(Ordering::Less),
          Ordering::Equal => None,
          Ordering::Greater => Some(Ordering::Greater),
        }
      },
    }
  }
}

lalrpop_mod!(pub parser);

use parser::ValueParser;

lazy_static::lazy_static! {
  static ref PARSER: ValueParser = ValueParser::new();
}

fn part_one() {
  let input = fs::read_to_string("input.txt").unwrap();

  let mut m1 = "";
  let mut m2 = "";

  let mut pair = 1;

  let mut sum_pairs = 0;

  for (i, line) in input.lines().enumerate() {
    match (i + 1) % 3 {
      1 => m1 = line.trim(),
      2 => m2 = line.trim(),
      _ => {
        if PARSER.parse(m1).unwrap() < PARSER.parse(m2).unwrap() {
          sum_pairs += pair;
        }
        pair += 1;
      }
    }
  }

  if PARSER.parse(m1).unwrap() < PARSER.parse(m2).unwrap() {
    sum_pairs += pair;
  }

  println!("{}", sum_pairs);
}

fn part_two() {
  let input = fs::read_to_string("input.txt").unwrap();

  let d1 = Value::List(vec![Value::List(vec![Value::Number(2)])]);
  let d2 = Value::List(vec![Value::List(vec![Value::Number(6)])]);

  let mut v = vec![d1.clone(), d2.clone()];

  for line in input.lines() {
    let line = line.trim();

    if line != "" {
      v.push(PARSER.parse(line).unwrap());
    }
  }

  v.sort();

  let i1 = v.iter().enumerate().find(|(_, x)| *x == &d1).unwrap().0 + 1;
  let i2 = v.iter().enumerate().find(|(_, x)| *x == &d2).unwrap().0 + 1;

  println!("{}", i1 * i2);
}

fn main() {
  part_one();
  part_two();
}

#[cfg(test)]
mod tests {
  use super::parser;

  #[test]
  fn calculator1() {
    assert!(parser::ValueParser::new().parse("22").is_ok());
    assert!(parser::ValueParser::new().parse("[22, 23, [24]]").is_ok());
    assert!(parser::ValueParser::new().parse("[[[[22]]]]").is_ok());
    assert!(parser::ValueParser::new().parse("[[22]").is_err());
  }
}
