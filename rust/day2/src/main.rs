use std::fs;

fn main() {
    let content = fs::read_to_string("../inputs/day2.input").unwrap();

    let mut part1 = 0;
    let mut part2 = 0;
    for line in content.lines() {
        let nums: Vec<i32> = line.split(" ").map(|x| x.parse().unwrap()).collect();
        let n = nums.len();

        let mut neg_cnt = 0;
        let mut pos_cnt = 0;
        let mut valid_cnt = 0;

        for i in 1..n {
            let diff = nums[i] - nums[i - 1];

            if diff < 0 {
                neg_cnt += 1;
            }
            if diff > 0 {
                pos_cnt += 1;
            }

            if diff.abs() > 0 && diff.abs() < 4 && (pos_cnt == i || neg_cnt == i) {
                valid_cnt += 1;
            }
        }

        let n = n as i32 - 1;

        if valid_cnt == n {
            part1 += 1;
            part2 += 1;
        } else {
            let mut success = false;
            for to_remove in 0..nums.len() {
                let mut nums = nums.clone();
                nums.remove(to_remove);
                let n = nums.len();
                let mut neg_cnt = 0;
                let mut pos_cnt = 0;
                let mut valid_cnt = 0;
                for i in 1..n {
                    let diff = nums[i] - nums[i - 1];

                    if diff < 0 {
                        neg_cnt += 1;
                    }
                    if diff > 0 {
                        pos_cnt += 1;
                    }

                    if diff.abs() > 0 && diff.abs() < 4 && (pos_cnt == i || neg_cnt == i) {
                        valid_cnt += 1;
                    }
                }

                if valid_cnt == n as i32 - 1 {
                    success = true;
                    break;
                }
            }

            if success {
                part2 += 1;
            }
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
