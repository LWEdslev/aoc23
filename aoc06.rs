fn part1() {
    let file = std::fs::read_to_string("aoc05.txt").unwrap();
    let mut lines = file.lines();
    let time_line = lines.next().unwrap();
    let distance_line = lines.next().unwrap();

    let parse = |s: &str| {
        s
        .split(":")
        .nth(1).unwrap()
        .split_ascii_whitespace()
        .filter_map(|t| t.trim().parse().ok()).collect::<Vec<i32>>()
    };

    let times = parse(time_line);
    let dists = parse(distance_line);


    let mut total = 1;
    let zipped = times.into_iter().zip(dists.into_iter());
    for (time, record) in zipped {
        let mut count = 0;
        for time_held in 1..time {
            let time_left_to_move = time - time_held;
            let speed = time_held;
            let actual_dist = time_left_to_move * speed;
            if actual_dist > record { count += 1 }
        }        
        total *= count;
    }

    println!("{total}");
}

fn part2() {
    let file = std::fs::read_to_string("aoc05.txt").unwrap();
    let mut lines = file.lines();
    let time_line = lines.next().unwrap();
    let distance_line = lines.next().unwrap();

    let parse = |s: &str| {
        s
        .split(":")
        .nth(1).unwrap()
        .split_ascii_whitespace()
        .filter_map(|t| t.trim().parse::<i64>().ok())
        .map(|i| i.to_string())
        .collect::<Vec<String>>().join("").parse::<i64>().unwrap()
    };

    let time = parse(time_line);
    let record = parse(distance_line);
    let mut count = 0;
    for time_held in 1..time {
        let time_left_to_move = time - time_held;
        let speed = time_held;
        let actual_dist = time_left_to_move * speed;
        if actual_dist > record { count += 1 }
    }        
    println!("{count}");
}

fn main() {
    part1();
    part2();
}