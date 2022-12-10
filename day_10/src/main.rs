use std::fs;

fn signal_strength(cycle: i32, x: i32) -> i32 {
  match cycle {
    20 | 60 | 100 | 140 | 180 | 220 => cycle * x,
    _ => 0,
  }
}

fn part_one() {
  let input = fs::read_to_string("input.txt").unwrap();

  let mut cycle = 0;
  let mut x = 1;
  let mut signal_strength_sum = 0;

  for line in input.lines() {
    let line  = line.trim();

    cycle += 1;
    signal_strength_sum += signal_strength(cycle, x);

    if line == "noop" {
      continue;
    }

    let mut addx = line.split(' ');
    addx.next().unwrap();

    let add: i32 = addx.next().unwrap().parse().unwrap();

    cycle += 1;
    signal_strength_sum += signal_strength(cycle, x);

    x += add;
  }

  println!("{}", signal_strength_sum);
}

fn part_two() {
  let input = fs::read_to_string("input.txt").unwrap();

  let mut cycle = 0;
  let mut x = 1;

  let mut screen = ['.'; 40 * 6];

  for line in input.lines() {
    let line  = line.trim();

    if x - 1 <= (cycle % 40) && (cycle % 40) <= x + 1 {
      screen[cycle as usize] = '#';
    }

    cycle += 1;

    if line == "noop" {
      continue;
    }

    let mut addx = line.split(' ');
    addx.next().unwrap();

    let add: i32 = addx.next().unwrap().parse().unwrap();

    if x - 1 <= (cycle % 40) && (cycle % 40) <= x + 1 {
      screen[cycle as usize] = '#';
    }

    cycle += 1;

    x += add;
  }

  for (i, c) in screen.iter().enumerate() {
    print!("{}", c);

    if (i + 1) % 40 == 0 {
      print!("\n");
    }
  }
}

fn main() {
  part_one();
  part_two();
}
