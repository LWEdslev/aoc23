use std::collections::HashMap;

fn part1() {
    let file = include_str!("../aoc06.txt");
    let mut lines = file.lines();
    let directions = lines.next().unwrap().trim();
    let _ = lines.next();
    let parse = |s: &str| {
        let key = s[0..3].to_string();
        let value1 = s[7..10].to_string();
        let value2 = s[12..15].to_string();
        (key, value1, value2)
    };

    let mut map = HashMap::new();

    lines.for_each(|line| {
        let (k, l, r) = parse(line);
        map.insert(k.clone(), (l,r));
    });

    let mut current_key = "AAA".to_string();
    let mut count = 0;

    for d in directions.chars().cycle() {
        count += 1;
        let right = match d {
            'R' => true,
            'L' => false,
            _ => unreachable!()
        };

        let (l, r) = map.get(&current_key).unwrap();
        current_key = if right { r.to_string() } else { l.to_string() };
        if current_key == "ZZZ".to_string() { 
            break;
        }
    }

    println!("{}", count);
}

fn part2() {
    let file = include_str!("../aoc06.txt");
    let mut lines = file.lines();
    let directions = lines.next().unwrap().trim();
    let _ = lines.next();
    let parse = |s: &str| {
        let key = s[0..3].to_string();
        let value1 = s[7..10].to_string();
        let value2 = s[12..15].to_string();
        (key, value1, value2)
    };

    let mut map = HashMap::new();

    lines.for_each(|line| {
        let (k, l, r) = parse(line);
        map.insert(k.clone(), (l,r));
    });

    let current_keys = map.keys().filter(|s| s.ends_with("A")).map(|s| s.clone()).collect::<Vec<_>>();
    let mut results = Vec::new();

    for key in current_keys.into_iter() {
        let mut count: u64 = 0;
        let mut current_key = key;
        for d in directions.chars().cycle() {
            count += 1;
            let right = match d {
                'R' => true,
                'L' => false,
                _ => unreachable!()
            };
    
            let (l, r) = map.get(&current_key).unwrap();
            current_key = if right { r.to_string() } else { l.to_string() };
            if current_key.ends_with("Z") { 
                results.push(count);
                break;
            }
        }
    }

    let mut res = 1;
    for r in results {
        res = num::integer::lcm(res, r)
    }
    println!("{res}");
}

fn main() {
    part1();
    part2();
}