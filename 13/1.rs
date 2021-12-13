use simple_matrix::Matrix;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut lines = io::BufReader::new(File::open("input.txt").unwrap()).lines();

    let maxx = 1400;
    let maxy = 1400;

    let mut lowestxfold = maxx;
    let mut lowestyfold = maxy;
    let mut count = 0;
    let mut mat: Matrix<u8> = Matrix::new(maxx, maxy);
    let mut reading_dots = true;
    let mut reading_folds = true;

    while reading_dots {
        let mut thisline = lines.next().unwrap().unwrap();
        let mut s = thisline.trim();

        if s.len() != 0 {
            let digits: Vec<&str> = s.split(",").collect();
            let x = u16::from_str_radix(digits[0], 10).unwrap();
            let y = u16::from_str_radix(digits[1], 10).unwrap();
            mat.set(x as usize, y as usize, 1);
        } else {
            reading_dots = false;
        }
    }

    while reading_folds {
        let mut tline = lines.next();

        if tline.is_none() {
            break;
        }

        let thisline = tline.unwrap().unwrap();
        let mut s = thisline.trim();
        let words: Vec<&str> = s.split(" ").collect();
        let fold = words.last().unwrap();
        let axis = fold.split("=").collect::<Vec<&str>>();
        let around = usize::from_str_radix(axis[1], 10).unwrap();

        if axis[0] == "x" {
            if lowestxfold > around {
                println!("x fold {}", around);
                lowestxfold = around;
            }

            for y in 0..maxy {
                let mut distance = 1;
                while distance <= around {
                    let upper = mat.get(around + distance, y).unwrap();
                    let lower = mat.get(around - distance, y).unwrap();
                    if *upper == 1 && *lower == 0 {
                        mat.set(around - distance, y, 1);
                    }
                    mat.set(around + distance, y, 0);
                    distance += 1;
                }
            }
        } else {
            if lowestyfold > around {
                println!("y fold {}", around);
                lowestyfold = around;
            }
            for x in 0..maxx {
                let mut distance = 1;
                while distance <= around {
                    let upper = mat.get(x, around + distance).unwrap();
                    let lower = mat.get(x, around - distance).unwrap();
                    if *upper == 1 && *lower == 0 {
                        mat.set(x, around - distance, 1);
                    }
                    mat.set(x, around + distance, 0);
                    distance += 1;
                }
            }
        }
        println!("{}", fold);

        let mut count = 0;
        for x in 0..maxx {
            for y in 0..maxy {
                if *mat.get(x, y).unwrap() == 1 {
                    count += 1;
                }
            }
        }
        println!("Count is {}", count);
    }

    for y in 0..lowestxfold {
        for x in 0..lowestyfold {
            if *mat.get(x, y).unwrap() == 1 {
                print!("#");
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }

    // println!("{}", count);
}
