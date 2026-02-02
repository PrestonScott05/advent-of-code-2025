use crate::utils::read_input;

pub fn run() {
    let input = read_input(1);

    let mut pos: i32 = 50;
    let mut hits_zero: i32 = 0;

    for raw_line in input.lines() {
        let line = raw_line.trim();

        if line.is_empty() {
            continue;
        }

        let dir = line.chars().next().unwrap();
        let dist_str = &line[1..];
        let dist: i32 = dist_str.parse().unwrap();

        hits_zero += count_zero_hits(pos, dir, dist);

        match dir {
            'L' => pos = (pos - dist).rem_euclid(100),
            'R' => pos = (pos + dist).rem_euclid(100),
            _ => panic!("Bad direction: {}", dir),
        }
    }

    println!("Day 01 - Part 2: {}", hits_zero)
}

fn count_zero_hits(pos: i32, dir: char, dist: i32) -> i32 {
    let pos = pos.rem_euclid(100);

    let offset = match dir {
        'R' => (100 - pos).rem_euclid(100),
        'L' => pos,
        _ => panic!("Bad direction: {}", dir),
    };

    if offset == 0 {
        return dist / 100
    }
    else if dist < offset {
        return 0
    }
    else {
        return 1 + (dist - offset) / 100
    }
}