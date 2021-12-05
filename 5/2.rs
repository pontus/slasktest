use simple_matrix::Matrix;
use std::cmp::max;
use std::cmp::min;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let dim = 1000;

    let mut mat: Matrix<i32> = Matrix::new(dim, dim);
    let lines = io::BufReader::new(File::open("input.txt").unwrap()).lines();

    for l in lines {
        let s = l.unwrap();

        println!("{}", &s);
        let v: Vec<&str> = s.split(" -> ").collect();
        let froms: Vec<&str> = v[0].split(",").collect();
        let tos: Vec<&str> = v[1].split(",").collect();

        let x1: usize = froms[0].parse().unwrap();
        let y1: usize = froms[1].parse().unwrap();

        let x2: usize = tos[0].parse().unwrap();
        let y2: usize = tos[1].parse().unwrap();

        if x1 == x2 {
            for y in min(y1, y2)..=max(y1, y2) {
                mat.set(x1, y, mat.get(x1, y).unwrap() + 1);
            }
        } else if y1 == y2 {
            for x in min(x1, x2)..=max(x1, x2) {
                println!("Setting x from {} to {}, current is {},{}", x1, x2, x, y1);

                mat.set(x, y1, mat.get(x, y1).unwrap() + 1);
            }
        } else {
            // Diagonal

            let mut k = 1;
            let mut y = y1;
            let startx = min(x1, x2);
            if startx == x2 {
                y = y2
            }

            if y == max(y1, y2) {
                // Starting at max, go downwards
                k = -1
            }

            for x in startx..=max(x1, x2) {
                mat.set(x, y, mat.get(x, y).unwrap() + 1);
                y = (y as i32 + k) as usize;
            }
        }
    }

    let mut count = 0;
    for y in 0..dim {
        for x in 0..dim {
            let a = mat.get(x, y).unwrap();
            print!("{}", a);

            if a >= &2 {
                count = count + 1;
            }
        }
        println!("");
    }

    println!("{}", count);
}
