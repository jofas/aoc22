use std::collections::{BinaryHeap, VecDeque};

const BOUNDS: u64 = 9_699_690;

fn throw<O>(
    i: usize,
    to_true: usize,
    to_false: usize,
    factor: u64,
    ops: &[O; 8],
    items: &mut [VecDeque<u64>; 8],
    counts: &mut [u128; 8],
) where
    O: Fn(u64) -> u64,
{
    while let Some(x) = items[i].pop_front() {
        counts[i] += 1;

        let x = ops[i](x) % BOUNDS;

        if x % factor == 0 {
            items[to_true].push_back(x);
        } else {
            items[to_false].push_back(x);
        }
    }
}

fn part_one() {
    let mut items = [
        VecDeque::from_iter([59, 74, 65, 86]),
        VecDeque::from_iter([62, 84, 72, 91, 68, 78, 51]),
        VecDeque::from_iter([78, 84, 96]),
        VecDeque::from_iter([97, 86]),
        VecDeque::from_iter([50]),
        VecDeque::from_iter([73, 65, 69, 65, 51]),
        VecDeque::from_iter([69, 82, 97, 93, 82, 84, 58, 63]),
        VecDeque::from_iter([81, 78, 82, 76, 79, 80]),
    ];

    let mut count = [0, 0, 0, 0, 0, 0, 0, 0];

    let ops = [
        |o: u64| (o * 19) / 3,
        |o: u64| (o + 1) / 3,
        |o: u64| (o + 8) / 3,
        |o: u64| o.pow(2) / 3,
        |o: u64| (o + 6) / 3,
        |o: u64| (o * 17) / 3,
        |o: u64| (o + 5) / 3,
        |o: u64| (o + 3) / 3,
    ];

    for _ in 0..20 {
        throw(0, 6, 2, 7, &ops, &mut items, &mut count);
        throw(1, 2, 0, 2, &ops, &mut items, &mut count);
        throw(2, 6, 5, 19, &ops, &mut items, &mut count);
        throw(3, 1, 0, 3, &ops, &mut items, &mut count);
        throw(4, 3, 1, 13, &ops, &mut items, &mut count);
        throw(5, 4, 7, 11, &ops, &mut items, &mut count);
        throw(6, 5, 7, 5, &ops, &mut items, &mut count);
        throw(7, 3, 4, 17, &ops, &mut items, &mut count);
    }

    let mut heap = BinaryHeap::from_iter(count.into_iter());

    let m1 = heap.pop().unwrap();
    let m2 = heap.pop().unwrap();

    println!("{:?}", m1 * m2);
}

fn part_two() {
    let mut items = [
        VecDeque::from_iter([59, 74, 65, 86]),
        VecDeque::from_iter([62, 84, 72, 91, 68, 78, 51]),
        VecDeque::from_iter([78, 84, 96]),
        VecDeque::from_iter([97, 86]),
        VecDeque::from_iter([50]),
        VecDeque::from_iter([73, 65, 69, 65, 51]),
        VecDeque::from_iter([69, 82, 97, 93, 82, 84, 58, 63]),
        VecDeque::from_iter([81, 78, 82, 76, 79, 80]),
    ];

    let mut count = [0, 0, 0, 0, 0, 0, 0, 0];

    let ops = [
        |o: u64| o * 19,
        |o: u64| o + 1,
        |o: u64| o + 8,
        |o: u64| o.pow(2),
        |o: u64| o + 6,
        |o: u64| o * 17,
        |o: u64| o + 5,
        |o: u64| o + 3,
    ];

    for _ in 0..10_000 {
        throw(0, 6, 2, 7, &ops, &mut items, &mut count);
        throw(1, 2, 0, 2, &ops, &mut items, &mut count);
        throw(2, 6, 5, 19, &ops, &mut items, &mut count);
        throw(3, 1, 0, 3, &ops, &mut items, &mut count);
        throw(4, 3, 1, 13, &ops, &mut items, &mut count);
        throw(5, 4, 7, 11, &ops, &mut items, &mut count);
        throw(6, 5, 7, 5, &ops, &mut items, &mut count);
        throw(7, 3, 4, 17, &ops, &mut items, &mut count);
    }

    let mut heap = BinaryHeap::from_iter(count.into_iter());

    let m1 = heap.pop().unwrap();
    let m2 = heap.pop().unwrap();

    println!("{:?}", m1 * m2);
}

fn main() {
    part_one();
    part_two();
}
