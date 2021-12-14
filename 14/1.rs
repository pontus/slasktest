use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut lines = io::BufReader::new(File::open("input.txt").unwrap()).lines();

    let start = lines.next().unwrap().unwrap();
    let throwaway = lines.next().unwrap().unwrap();

    let mut map = HashMap::new();

    for l in lines {
        let s = l.unwrap().clone();
        let line = s.split(" -> ").collect::<Vec<&str>>();
        map.insert(String::from(line[0]), String::from(line[1]));
    }

    let mut line = String::from(start);
    for i in 0..10 {
        let mut j = 1;
        while j < line.len() {
            let current_pair = &line[j - 1..j + 1];
            if map.contains_key(current_pair) {
                line.insert(j, map[current_pair].chars().nth(0).unwrap());
                j += 1;
            }
            j += 1;
        }
        println!("Line is {}", line.len());
    }

    let mut countmap = HashMap::new();
    for c in line.chars() {
        let count = countmap.entry(c).or_insert(0);
        *count += 1;
    }

    println!("{:?}", countmap);
}
