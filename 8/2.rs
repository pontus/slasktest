use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut sum = 0 as u32;
    let lines = io::BufReader::new(File::open("input.txt").unwrap()).lines();

    let _uniquelengths = Vec::from([2, 7, 4, 3]);

    for l in lines {
        let line = l.unwrap();
        let indata: Vec<&str> = line.split("|").collect();

        let digitsleft: Vec<&str> = indata[0].trim().split(" ").collect();
        let digitsright: Vec<&str> = indata[1].trim().split(" ").collect();

        let mut zero: HashSet<u8> = HashSet::new();

        let mut one: HashSet<u8> = HashSet::new();
        let mut two: HashSet<u8> = HashSet::new();
        let mut three: HashSet<u8> = HashSet::new();
        let mut four: HashSet<u8> = HashSet::new();
        let mut five: HashSet<u8> = HashSet::new();
        let mut six: HashSet<u8> = HashSet::new();
        let mut seven: HashSet<u8> = HashSet::new();
        let mut eight: HashSet<u8> = HashSet::new();
        let mut nine: HashSet<u8> = HashSet::new();

        for _redo in 0..10 {
            for p in &digitsleft {
                let x = p.len();

                let s = p.to_owned();
                if x == 2 && one.is_empty() {
                    one.extend(s.as_bytes());
                }
                if x == 3 && seven.is_empty() {
                    seven.extend(s.as_bytes());
                }
                if x == 4 && four.is_empty() {
                    four.extend(s.as_bytes());
                }

                if x == 5 && !one.is_empty() && !four.is_empty() {
                    let mut set = HashSet::new();
                    set.extend(s.as_bytes());

                    if one.is_subset(&set) {
                        three.extend(set);
                    } else {
                        let mut leftmiddle = HashSet::new();
                        leftmiddle.extend(four.difference(&one));

                        if leftmiddle.is_subset(&set) {
                            five.extend(set);
                        } else {
                            two.extend(set);
                        }
                    }
                }
                if x == 6 && !one.is_empty() && !four.is_empty() {
                    let mut set = HashSet::new();
                    set.extend(s.as_bytes());
                    if four.is_subset(&set) {
                        nine.extend(set);
                    } else {
                        if one.is_subset(&set) {
                            zero.extend(set);
                        } else {
                            six.extend(set);
                        }
                    }
                }
                if x == 7 {
                    eight.extend(s.as_bytes());
                }
            }
        }
        let mut numberstring = String::new();
        for p in &digitsright {
            let mut set: HashSet<u8> = HashSet::new();
            let s = p.to_owned();
            set.extend(s.as_bytes());

            if set == zero {
                numberstring.push('0');
            }

            if set == one {
                numberstring.push('1');
            }
            if set == two {
                numberstring.push('2');
            }
            if set == three {
                numberstring.push('3');
            }
            if set == four {
                numberstring.push('4');
            }
            if set == five {
                numberstring.push('5');
            }
            if set == six {
                numberstring.push('6');
            }
            if set == seven {
                numberstring.push('7');
            }
            if set == eight {
                numberstring.push('8');
            }
            if set == nine {
                numberstring.push('9');
            }
        }
        println!("{}", numberstring);
        let n: u32 = numberstring.parse().unwrap();

        sum = sum + n;
    }
    println!("{}", sum);
}
