use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut count = 0 as u32;
    let lines = io::BufReader::new(File::open("input.txt").unwrap()).lines();

    let uniquelengths = Vec::from([2, 7, 4, 3]);

    for l in lines {
        let s = l.unwrap();
        let indata: Vec<&str> = s.split("|").collect();
        let digits: Vec<&str> = indata[1].trim().split(" ").collect();

        for p in digits {
            if uniquelengths.contains(&p.len()) {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
