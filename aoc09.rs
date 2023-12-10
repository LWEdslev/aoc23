fn inputs() -> Vec<Vec<i64>> {
    let file = include_str!("../input.txt");
    file.lines().map(|l| l.trim().split_ascii_whitespace().map(|token| {
        token.parse::<i64>().unwrap()
    }).collect()).collect()
}

fn part1() {
    let inputs = inputs();
    let extensions = inputs.into_iter().map(|input| {
        let triangle = make_triangle(&input);
        let extended_triangle = extend_triangle(&triangle);
        last_in_triangle(&extended_triangle)
    }).collect::<Vec<_>>();
    let sum: i64 = extensions.iter().sum();
    println!("{sum}");
}

fn part2() {
    let inputs = inputs();
    let extensions = inputs.into_iter().map(|input| {
        let triangle = make_triangle(&input);
        let extended_triangle = extend_triangle_backwards(&triangle);
        first_in_triangle(&extended_triangle)
    }).collect::<Vec<_>>();
    let sum: i64 = extensions.iter().sum();
    println!("{sum}");
}

fn main() {
    part1();
    part2();
}

fn make_triangle(line: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut res = Vec::new();
    let mut curr_line = line.clone();
    while !curr_line.iter().all(|i| *i == 0) {
        res.push(curr_line.clone());
        curr_line = curr_line.windows(2).map(|w| {
            let f = w[0];
            let s = w[1];
            s - f
        }).collect();
    }
    res.push(curr_line.clone());

    res
}

fn extend_triangle(triangle: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    triangle.iter().rev().fold(Vec::new(), |acc, line| {
        let mut acc = acc.clone();
        if acc.is_empty() { acc.push(line.clone()) ; return acc }
        let add = acc.last().unwrap().last().unwrap();
        let last_in_line = line.last().unwrap();
        let mut new_line = line.clone();
        new_line.push(last_in_line + add);
        acc.push(new_line);
        acc
    }).into_iter().rev().collect()
}

fn extend_triangle_backwards(triangle: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    triangle.iter().rev().fold(Vec::new(), |acc, line| {
        let mut acc = acc.clone();
        if acc.is_empty() { acc.push(line.clone()) ; return acc }
        let add = acc.last().unwrap().first().unwrap();
        let first_in_line = line.first().unwrap();
        let mut new_line = line.clone();
        new_line.insert(0, first_in_line - add);
        acc.push(new_line);
        acc
    }).into_iter().rev().collect()
}

fn last_in_triangle(triangle: &Vec<Vec<i64>>) -> i64 {
    *triangle.first().unwrap().last().unwrap()
}

fn first_in_triangle(triangle: &Vec<Vec<i64>>) -> i64 {
    *triangle.first().unwrap().first().unwrap()
}