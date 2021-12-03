use std::fs::File;

use std::io::{self, BufRead};

fn main() {
    let mut done = false;
    let mut digit = 0;
    let mut gammas = String::new();
    let mut epsilons = String::new();

    while !done {
        let (zero, one) = get_counts(digit);

        // Bail out?
        if zero < 0 {
            done = true;
            continue;
        }

        if zero > one {
            gammas.push('0');
            epsilons.push('1');
        } else {
            gammas.push('1');
            epsilons.push('0');
        }

        digit = digit + 1;
    }

    let gamma = i32::from_str_radix(&gammas, 2).unwrap();
    let epsilon = i32::from_str_radix(&epsilons, 2).unwrap();

    println!("{} * {} = {}", gamma, epsilon, gamma * epsilon);
}

fn get_counts(digit: usize) -> (isize, isize) {
    let lines = io::BufReader::new(File::open("input.txt").unwrap()).lines();

    let mut c: isize = 0;
    let mut nlines: isize = 0;

    for l in lines {
        nlines = nlines + 1;
        let v: Vec<char> = l.unwrap().chars().collect();

        if v.len() <= digit {
            return (-1, -1);
        }

        if v[digit] == '0' {
            c = c + 1;
        }
    }

    return (c, nlines - c);
}
