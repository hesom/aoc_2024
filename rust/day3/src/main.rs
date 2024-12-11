use regex::Regex;
use std::fs;

fn main() {
    let content = fs::read_to_string("../inputs/day3.input").unwrap();

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let part1 = re
        .captures_iter(&content)
        .map(|caps| {
            let (_, [val1, val2]) = caps.extract();
            (val1.parse::<i32>().unwrap(), val2.parse::<i32>().unwrap())
        })
        .fold(0, |acc, (val1, val2)| acc + (val1 * val2));

    let re = Regex::new(r"mul\((\d+),(\d+)\)|(do\(\))|(don't\(\))").unwrap();
    let mut part2 = 0;
    let mut enabled = true;
    for caps in re.captures_iter(&content) {
        if let Some(_) = caps.get(3) {
            enabled = true;
            continue;
        }
        if let Some(_) = caps.get(4) {
            enabled = false;
            continue;
        }

        if enabled {
            let val1 = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let val2 = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
            part2 += val1 * val2;
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
