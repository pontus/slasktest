use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut l = String::new();
    let count = 0;
    io::BufReader::new(File::open("input.txt").unwrap()).read_line(&mut l);
    let indata: Vec<&str> = l.split(",").collect();

    let mut v = Vec::new();
    for p in indata {
        let a: i32 = p.trim().parse().unwrap();
        v.push(a);
    }

    let mut best = 1000000000;
    let mut bestpos = -1;
    for p in *v.iter().min().unwrap()..=*v.iter().max().unwrap() {
        let cost = calc_cost(&v, p);
        if cost < best {
            best = cost;
            bestpos = p;
        }
    }

    println!("{} {}", bestpos, best);
}

fn calc_cost(v: &Vec<i32>, pos: i32) -> i32 {
    let mut cost = 0;

    for p in v {
        let distance = (p - pos).abs();

        cost += distance * (distance + 1) / 2;
    }
    return cost;
}
