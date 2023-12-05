use std::error::Error;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Clone, Debug)]
struct Mapping(u32, u32, u32);

impl Mapping {
    fn from_str(s: &str) -> Option<Self> {
        let mut nums = s.split(" ").map(|t| t.trim().parse()).into_iter();
        Some(Self(nums.next()?.ok()?, nums.next()?.ok()?, nums.next()?.ok()?))
    }

    fn map_to(&self, n: u32) -> Option<u32> {
        let range = self.1..(self.1+self.2);
        if range.contains(&n) {
            let dest = self.0 + (n - self.1);
            return Some(dest)
        }
        None
    }
}

fn map(mappings: &Vec<Mapping>, n: u32) -> u32 {
    for mapping in mappings {
        if let Some(n) = mapping.map_to(n) {
            return n;
        }
    }
    n
}

fn pair_to_seeds(pairs: Vec<(u32,u32)>) -> Vec<u32> {
    pairs.into_iter().map(|pair| {
        println!("{:?}", pair);
        let mut res = Vec::new();
        for seed in pair.0..(pair.0 + pair.1) {
            res.push(seed)
        }
        res
    }).flatten().collect()
}

fn pair_to_seed(pair: (u32, u32)) -> Vec<u32> {
    let mut res = Vec::new();
        for seed in pair.0..(pair.0 + pair.1) {
            res.push(seed)
        }
    res
}

fn map_all(mappings: &Vec<Mapping>, ns: &Vec<u32>) -> Vec<u32> {
    ns.iter().map(|n| map(mappings, n.clone())).collect()
}

fn main() {
    let file = std::fs::read_to_string("aoc05.txt").unwrap();
    let seeds = file.lines().next().unwrap().split(":").nth(1).unwrap().split(" ").filter_map(|t| t.trim().parse::<u32>().ok()).collect::<Vec<u32>>();
    println!("here");
    let pairs = {
        let mut res = Vec::new();
        for i in (0..seeds.len()).step_by(2) {
            let pair = (seeds[i], seeds[i+1]);
            res.push(pair)
        }
        res
    };
    
    let results = pairs.iter().map(|pair| {
        let mut map_split = file.split("map:");
        let _ = map_split.next();
        let mut seeds = pair_to_seed(*pair);

        for i in 0..7 {
            let vec = map_split.next().unwrap().lines().filter_map(|s| Mapping::from_str(s)).collect::<Vec<Mapping>>();
            seeds = map_all(&vec, &seeds);
        }
        let found_min = *seeds.iter().min().unwrap();
        found_min
    }).min();

    println!("min {:?}", results);
}