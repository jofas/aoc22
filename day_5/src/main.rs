use std::fs;

fn part_one() {
    let mut stacks = [
        vec!["M", "J", "C", "B", "F", "R", "L", "H"],
        vec!["Z", "C", "D"],
        vec!["H", "J", "F", "C", "N", "G", "W"],
        vec!["P", "J", "D", "M", "T", "S", "B"],
        vec!["N", "C", "D", "R", "J"],
        vec!["W", "L", "D", "Q", "P", "J", "G", "Z"],
        vec!["P", "Z", "T", "F", "R", "H"],
        vec!["L", "V", "M", "G"],
        vec!["C", "B", "G", "P", "F", "Q", "R", "J"],
    ];

    let moves = fs::read_to_string("input.txt").unwrap();

    for m in moves.lines().skip(10) {
        let (s, i, j) = match m.split(' ').collect::<Vec<&str>>()[..] {
            ["move", s, "from", i, "to", j] => (s, i, j),
            _ => unreachable!(),
        };

        let s: usize = s.parse().unwrap();
        let i: usize = i.parse().unwrap();
        let j: usize = j.parse().unwrap();

        for _ in 0..s {
            let v = stacks[i - 1].pop().unwrap();
            stacks[j - 1].push(v);
        }
    }

    let res = stacks
        .iter()
        .fold(String::new(), |acc, s| acc + s[s.len() - 1]);

    println!("{}", res);
}

fn part_two() {
    let mut stacks = [
        vec!["M", "J", "C", "B", "F", "R", "L", "H"],
        vec!["Z", "C", "D"],
        vec!["H", "J", "F", "C", "N", "G", "W"],
        vec!["P", "J", "D", "M", "T", "S", "B"],
        vec!["N", "C", "D", "R", "J"],
        vec!["W", "L", "D", "Q", "P", "J", "G", "Z"],
        vec!["P", "Z", "T", "F", "R", "H"],
        vec!["L", "V", "M", "G"],
        vec!["C", "B", "G", "P", "F", "Q", "R", "J"],
    ];

    let moves = fs::read_to_string("input.txt").unwrap();

    for m in moves.lines().skip(10) {
        let (s, i, j) = match m.split(' ').collect::<Vec<&str>>()[..] {
            ["move", s, "from", i, "to", j] => (s, i, j),
            _ => unreachable!(),
        };

        let s: usize = s.parse().unwrap();
        let i: usize = i.parse().unwrap();
        let j: usize = j.parse().unwrap();

        let src = stacks[i - 1].clone();

        for x in &src[src.len() - s..src.len()] {
            stacks[j - 1].push(x);
        }

        stacks[i - 1] = src[..src.len() - s].to_vec();
    }

    let res = stacks
        .iter()
        .fold(String::new(), |acc, s| acc + s[s.len() - 1]);

    println!("{}", res);
}

fn main() {
    part_one();
    part_two();
}
