use simple_matrix::Matrix;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let linesthrow = io::BufReader::new(File::open("input.txt").unwrap()).lines();

    let mut h = 0;
    let mut w = 0;

    for l in linesthrow {
        h += 1;
        w = l.unwrap().len();
    }

    let lines = io::BufReader::new(File::open("input.txt").unwrap()).lines();

    let mut count = 0;
    let mut mat: Matrix<u16> = Matrix::new(w, h);
    let mut y = 0;
    for l in lines {
        let s = l.unwrap();
        for (x, c) in s.chars().enumerate() {
            mat.set(x, y, c as u16 - 48);
        }
        y += 1;
    }

    for _i in 0..100 {
        for x in 0..w {
            for y in 0..h {
                mat.set(x, y, mat.get(x, y).unwrap() + 1);
            }
        }

        let mut flashedalready = Vec::new();
        let mut flashesdetected = true;
        while flashesdetected {
            flashesdetected = false;

            for x in 0..w {
                for y in 0..h {
                    if flashedalready.contains(&(x, y)) {
                        continue;
                    }

                    if *mat.get(x, y).unwrap() > 9 {
                        flashesdetected = true;
                        flashedalready.push((x, y));

                        if x > 0 {
                            mat.set(x - 1, y, mat.get(x - 1, y).unwrap() + 1);
                        }
                        if x > 0 && y > 0 {
                            mat.set(x - 1, y - 1, mat.get(x - 1, y - 1).unwrap() + 1);
                        }
                        if y > 0 {
                            mat.set(x, y - 1, mat.get(x, y - 1).unwrap() + 1);
                        }
                        if y > 0 && x < w - 1 {
                            mat.set(x + 1, y - 1, mat.get(x + 1, y - 1).unwrap() + 1);
                        }

                        if x < w - 1 {
                            mat.set(x + 1, y, mat.get(x + 1, y).unwrap() + 1);
                        }
                        if x < w - 1 && y < h - 1 {
                            mat.set(x + 1, y + 1, mat.get(x + 1, y + 1).unwrap() + 1);
                        }

                        if y < h - 1 {
                            mat.set(x, y + 1, mat.get(x, y + 1).unwrap() + 1);
                        }
                        if x > 0 && y < h - 1 {
                            mat.set(x - 1, y + 1, mat.get(x - 1, y + 1).unwrap() + 1);
                        }
                    }
                }
            }
        }

        count += flashedalready.len();
        for p in flashedalready {
            mat.set(p.0, p.1, 0);
        }

        println!("{}", count);
    }

    println!("{}", count);
}
