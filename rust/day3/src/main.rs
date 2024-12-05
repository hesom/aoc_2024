use std::fs;

fn main() {
    let content = fs::read_to_string("../inputs/day3.input").unwrap();

    let mut l = 0;

    let mut res = 0;
    while let Some(idx) = content[l..].find("mul") {
        let leftp = l + idx + 3;
        if content.chars().nth(leftp).unwrap_or(' ') == '(' {
            if let Some(rightp) = content[leftp..].find(")") {
                let vals: Vec<_> = content[leftp+1..leftp+rightp].split(",").collect();
                if vals.len() == 2 {
                    if let Ok(a) = vals[0].parse::<i32>() {
                        if let Ok(b) = vals[1].parse::<i32>() {
                            res += a * b;
                        }
                    };
                }
            };
        }
        l += idx + 1;
    }

    println!("Part 1: {}", res);
}
