use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut ways = Vec::new();
    let lines = io::BufReader::new(File::open("input.txt").unwrap()).lines();

    for l in lines {
        let s = l.unwrap().clone();
        let line = s.split("-").collect::<Vec<&str>>();
        ways.push((String::from(line[0]), String::from(line[1])));
        ways.push((String::from(line[1]), String::from(line[0])));
    }

    let mut v = Vec::new();
    v.push(String::from("start"));
    let mut all_paths = return_paths(ways, v).unwrap();

    println!("{}", all_paths.len());
}

fn is_lower(s: &String) -> bool {
    let mut lower = true;
    for c in s.chars() {
        if c.is_uppercase() {
            lower = false;
        }
    }
    return lower;
}

fn return_paths(ways: Vec<(String, String)>, started: Vec<String>) -> Option<Vec<Vec<String>>> {
    let mut rp = Vec::new();

    //println!("{:?}", started);
    // Already complete
    if started.last().unwrap() == "end" {
        rp.push(started.clone());

        return Some(rp);
    }

    let current_path = started.clone();

    for j in 0..ways.len() {
        if ways[j].0 == *current_path.last().unwrap() {
            if !is_lower(&ways[j].1) {
                let mut new_path = current_path.clone();
                new_path.push(ways[j].1.clone());
                rp.push(new_path);
            } else {
                // &ways[j].1 is lowercase

                let key = ways[j].1.clone();
                if current_path.contains(&key) && (key == "start" || key == "end") {
                    continue;
                }

                let mut map = HashMap::new();

                for p in &current_path {
                    if is_lower(p) {
                        let counter = map.entry(p.clone()).or_insert(0);
                        *counter += 1;
                    }
                }

                let e = *map.entry(key).or_insert(0);
                if e == 2 {
                    // This one has already appeared twice?
                    continue;
                }

                // We've already seen one with two lower and have one of ours
                if map.values().find(|&x| *x == 2).is_some() && e == 1 {
                    // Some other has already appeared twice?
                    continue;
                }

                let mut new_path = current_path.clone();
                new_path.push(ways[j].1.clone());
                rp.push(new_path);
            }
        }
    }
    let mut rrp = Vec::new();
    for i in 0..rp.len() {
        let r = return_paths(ways.clone(), rp[i].clone());
        if r.is_some() {
            let returned_paths = r.unwrap();
            for j in 0..returned_paths.len() {
                rrp.push(returned_paths[j].clone());
            }
        }
    }

    return Some(rrp);
}
