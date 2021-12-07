use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut l = String::new();
    let mut count = 0;
    io::BufReader::new(File::open("input.txt").unwrap()).read_line(&mut l);
    let indata: Vec<&str> = l.split(",").collect();

    let mut f: [i64; 9] = [0; 9];
    for p in indata {
        let a: usize = p.trim().parse().unwrap();
        f[a] = f[a] + 1;
    }

    for p in 0..256 {
        let t = f[0];
        f[0] = f[1];
        f[1] = f[2];
        f[2] = f[3];
        f[3] = f[4];
        f[4] = f[5];
        f[5] = f[6];
        f[6] = t + f[7];
        f[7] = f[8];
        f[8] = t;
    }

    for p in f {
        count = count + p;
    }

    println!("{}", count);
}
