use std::fs::File;

use std::io::{self, BufRead};

fn main() {
    let mut done = false;
    let mut digit = 0;
    let mut oxygen = String::new();
    let mut co2 = String::new();

    while !done {
        let (ozero, oone) = get_counts(digit, &oxygen);
        // Bail out?
        if ozero < 0 {
            done = true;
            continue;
        }
        println!(
            "Checked digit {}, prefix {} yielded {}, {}",
            digit, oxygen, ozero, oone
        );

        if ozero + oone == 1 {
            if ozero == 1 {
                oxygen.push('0');
            } else {
                oxygen.push('1');
            }
        } else {
            if ozero > oone {
                oxygen.push('0');
            } else {
                oxygen.push('1');
            }
        }

        let (czero, cone) = get_counts(digit, &co2);

        println!(
            "Checked digit {}, prefix {} yielded {}, {}",
            digit, co2, czero, cone
        );

        if czero + cone == 1 {
            if czero == 1 {
                co2.push('0');
            } else {
                co2.push('1');
            }
        } else {
            if czero <= cone {
                co2.push('0');
            } else {
                co2.push('1');
            }
        }

        digit = digit + 1;
    }
    println!("{} and {}", oxygen, co2);

    let oxygenv = i32::from_str_radix(&oxygen, 2).unwrap();
    let co2v = i32::from_str_radix(&co2, 2).unwrap();

    println!("{} and {} yield {}", oxygenv, co2v, oxygenv * co2v);
}

fn get_counts(digit: usize, s: &String) -> (isize, isize) {
    let lines = io::BufReader::new(File::open("input.txt").unwrap()).lines();

    let mut c: isize = 0;
    let mut nlines: isize = 0;

    let slen = s.len();

    for l in lines {
        let m = l.unwrap();

        // Matches string
        if !s.eq(&(m[..slen])) {
            continue;
        }

        nlines = nlines + 1;
        let v: Vec<char> = m.chars().collect();

        if v.len() <= digit {
            return (-1, -1);
        }

        if v[digit] == '0' {
            c = c + 1;
        }
    }

    return (c, nlines - c);
}
