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

    let mut pairmap: HashMap<String, i128> = HashMap::new();
    for i in 0..start.len() - 1 {
        let key = &start[i..i + 2];
        let c = pairmap.entry(String::from(key)).or_insert(0);
        *c += 1;
    }

    for i in 0..40 {
        // println!("Round {}", i);
        // println!("Pairmap {:?}", pairmap);

        let mut newmap: HashMap<String, i128> = HashMap::new();
        for (k, v) in &pairmap {
            if map.contains_key(k) {
                let insert = map.get(k).unwrap().chars().nth(0).unwrap();

                let mut lefts = String::new();
                lefts.push(k.chars().nth(0).unwrap());
                lefts.push(insert);
                let mut rights = String::new();
                rights.push(insert);
                rights.push(k.chars().nth(1).unwrap());

                // println!(
                //     "k {} -> {} v {} left {} right {}",
                //     k, insert, v, lefts, rights
                // );
                // // Ugly work around to avoid multiple borrows

                let mut leftc = *newmap.entry(lefts.clone()).or_insert(0);
                leftc += v;
                newmap.insert(lefts, leftc);

                let mut rightc = *newmap.entry(rights.clone()).or_insert(0);
                rightc += v;
                newmap.insert(rights, rightc);
            }
        }

        pairmap = newmap;
    }

    //println!("{:?}", pairmap);

    let mut countmap = HashMap::new();
    for (k, v) in &pairmap {
        let mut c = countmap.entry(k.get(0..1)).or_insert(0);
        *c += v;
        let mut d = countmap.entry(k.get(1..2)).or_insert(0);
        *d += v;
    }

    let countmap2 = countmap.clone();
    let vals = countmap2.values();
    let max = vals.clone().max().unwrap();
    let min = vals.clone().min().unwrap();
    //    println!("{:?}", vals.max().unwrap());
    //    println!("{:?}", vals.min().unwrap());

    //println!("{:?}", countmap);

    for (k, v) in countmap {
        println!("{} {}", k.unwrap(), v);
    }

    println!("Max {}", max);
    println!("Min {}", min);

    // This may be off because of endings withing or not, but it's close enough
    println!("Response {}", (max - min) / 2);
}
