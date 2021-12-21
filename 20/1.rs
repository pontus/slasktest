use simple_matrix::Matrix;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut linesthrow = io::BufReader::new(File::open("input.txt").unwrap()).lines();

    let alg = linesthrow.next().unwrap().unwrap();
    linesthrow.next().unwrap().unwrap();

    let algc = alg.chars().collect::<Vec<char>>();

    let mut h = 0;
    let mut w = 0;

    for l in linesthrow {
        h += 1;
        w = l.unwrap().len();
    }

    let origw = w;
    let origh = h;

    w += 640;
    h += 640;

    let mut lines = io::BufReader::new(File::open("input.txt").unwrap()).lines();

    let mut mat: Matrix<usize> = Matrix::new(w, h);
    let mut other: Matrix<usize> = Matrix::new(w, h);

    lines.next().unwrap().unwrap();
    lines.next().unwrap().unwrap();

    // Read image
    let mut y = 0;
    for l in lines {
        let s = l.unwrap();
        for (x, c) in s.chars().enumerate() {
            if c == '#' {
                mat.set(x + 80, y + 80, 1);
            }
        }
        y += 1;
    }

    // Enhance

    for x in 0..w {
        for y in 0..h {
            let c = get_neighbors(&mat, x, y);
            if algc[c] == '#' {
                other.set(x, y, 1);
            } else {
                other.set(x, y, 0);
            }
        }
    }

    mat = other.clone();

    for x in 0..w {
        for y in 0..h {
            let c = get_neighbors(&mat, x, y);
            if algc[c] == '#' {
                other.set(x, y, 1);
            } else {
                other.set(x, y, 0);
            }
        }
    }

    let mut count = 0;
    for x in 78..82 + origw {
        for y in 78..82 + origh {
            if *other.get(x, y).unwrap() == 1 {
                count += 1;
            }
        }
    }

    println!("{}", count);
}

fn get_neighbors(mat: &Matrix<usize>, x: usize, y: usize) -> usize {
    // println!("Starting lookup for {},{}", x, y);
    //    println!("Path here {:?}", path);

    let mut num: usize = 0;

    for i in (y as isize) - 1..=(y + 1) as isize {
        for j in (x as isize) - 1..=(x + 1) as isize {
            num = num * 2;
            if i < 0 || j < 0 || i >= mat.rows() as isize || j >= mat.cols() as isize {
                continue;
            }
            num += mat.get(j as usize, i as usize).unwrap();
        }
    }

    // println!("Returning {}", num);
    return num;
}
