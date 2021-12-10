use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let lines = io::BufReader::new(File::open("input.txt").unwrap()).lines();
    let mut lookup = HashMap::new();
    let mut points = HashMap::new();

    lookup.insert(']', '[');
    lookup.insert('}', '{');
    lookup.insert('>', '<');
    lookup.insert(')', '(');

    points.insert(')', 3);
    points.insert(']', 57);
    points.insert('}', 1197);
    points.insert('>', 25137);

    let mut score = 0;

    for l in lines {
        let s = l.unwrap();
        let mut stack: Vec<char> = Vec::new();
        println!("Checking line {}", s);

        for (x, c) in s.chars().enumerate() {
            if lookup.contains_key(&c) {
                if stack.pop().unwrap() != *lookup.get(&c).unwrap() {
                    println!("{} found at {}", c, x);
                    score = score + points.get(&c).unwrap();
                    break;
                }
            } else {
                stack.push(c);
            }
        }
    }
    println!("Score was {}", score);
}
