use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut l = String::new();
    let count = 0;
    io::BufReader::new(File::open("input.txt").unwrap()).read_line(&mut l);
    let indata: Vec<&str> = l.split(",").collect();

    let mut v = Vec::new();
    for p in indata {
        let a: i8 = p.trim().parse().unwrap();
        v.push(a);
    }

    for day in 0..80 {
        for p in 0..v.len() {
            v[p] = v[p] - 1;
        }

        for p in 0..v.len() {
            if v[p] == -1 {
                v[p] = 6;
                v.push(8);
            }
        }
    }

    println!("{}", v.len());
}
