use std::fs;

fn main() {
    let content = fs::read_to_string("../inputs/day4.input").unwrap();
    let lines: Vec<Vec<_>> = content.lines().map(|line| line.chars().collect()).collect();

    let rows = lines.len();
    let cols = lines[0].len();

    let search = vec!['X', 'M', 'A', 'S'];

    let dirs = vec![
        (1, 0),
        (0, 1),
        (-1, 0),
        (0, -1),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];

    let mut part1 = 0;

    for row in 0..rows {
        for col in 0..cols {
            for (dx, dy) in &dirs {
                let mut found = true;
                for i in 0..search.len() {
                    let x = (i as i32 * dx) + row as i32;
                    let y = (i as i32 * dy) + col as i32;
                    if x < 0 || x >= rows as i32 || y < 0 || y >= cols as i32 {
                        found = false;
                        break;
                    }

                    let c = lines[x as usize][y as usize];
                    if c != search[i] {
                        found = false;
                        break;
                    }
                }
                if found {
                    part1 += 1;
                }
            }
        }
    }

    let mut part2 = 0;

    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            if lines[row][col] != 'A' {
                continue;
            }

            let mut found1 = false;
            if lines[row - 1][col - 1] == 'M' {
                if lines[row + 1][col + 1] == 'S' {
                    found1 = true;
                }
            }

            if lines[row + 1][col + 1] == 'M' {
                if lines[row - 1][col - 1] == 'S' {
                    found1 = true;
                }
            }

            let mut found2 = false;
            if lines[row + 1][col - 1] == 'M' {
                if lines[row - 1][col + 1] == 'S' {
                    found2 = true;
                }
            }

            if lines[row - 1][col + 1] == 'M' {
                if lines[row + 1][col - 1] == 'S' {
                    found2 = true;
                }
            }

            if found1 && found2 {
                part2 += 1;
            }
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
