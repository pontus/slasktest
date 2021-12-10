use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let lines = io::BufReader::new(File::open("input.txt").unwrap()).lines();
    let mut lookup = HashMap::new();
    let mut points = HashMap::new();

    let mut scores = Vec::new();
    lookup.insert(']', '[');
    lookup.insert('}', '{');
    lookup.insert('>', '<');
    lookup.insert(')', '(');

    points.insert('(', 1);
    points.insert('[', 2);
    points.insert('{', 3);
    points.insert('<', 4);

    for l in lines {
        let s = l.unwrap();
        let mut stack: Vec<char> = Vec::new();
        println!("Checking line {}", s);
        let mut corrupt = false;

        for (x, c) in s.chars().enumerate() {
            if lookup.contains_key(&c) {
                if stack.pop().unwrap() != *lookup.get(&c).unwrap() {
                    println!("{} found at {}", c, x);
                    corrupt = true;
                    break;
                }
            } else {
                stack.push(c);
            }
        }

        let mut linescore: u64 = 0;
        if !corrupt {
            // Line is possibly incomplete but not corrupt

            stack.reverse();
            for p in stack.iter() {
                //                println!("{}", p);
                linescore *= 5;
                linescore += points.get(p).unwrap();
            }
            println!("Linescore was {}", linescore);
            scores.push(linescore);
        }
    }

    scores.sort();
    println!("{}", scores[scores.len() / 2]);
}
