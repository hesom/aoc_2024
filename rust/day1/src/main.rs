use std::{collections::HashMap, fs, iter::zip};

fn main() {
    let content = fs::read_to_string("../inputs/day1.input").unwrap();

    let (mut left, mut right): (Vec<_>, Vec<_>) = content
        .lines()
        .map(|l| {
            let mut splits = l.split_whitespace();
            return (
                splits.next().unwrap().parse::<i32>().unwrap(),
                splits.next().unwrap().parse::<i32>().unwrap(),
            );
        })
        .unzip();

    left.sort();
    right.sort();

    let mut counts: HashMap<i32, i32> = HashMap::new();

    for r in &right {
        if !counts.contains_key(r) {
            counts.insert(*r, 0);
        }

        let entry = counts.get_mut(&r).unwrap();
        *entry += 1;
    }


    let mut diff = 0;
    let mut similarity = 0;
    for (l, r) in zip(left, right) {
        diff += l.abs_diff(r);

        if counts.contains_key(&l) {
            similarity += counts.get(&l).unwrap() * l;
        }
    }

    println!("Part 1: {}", diff);
    println!("Part 2: {}", similarity);
}
