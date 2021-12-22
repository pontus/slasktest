use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut lines = io::BufReader::new(File::open("input.txt").unwrap()).lines();
    let mut pos = Vec::new();

    for l in lines {
        let line = l.unwrap();
        let split = line.split(" ").collect::<Vec<&str>>();
        let startpos = u16::from_str_radix(split.last().unwrap(), 10).unwrap();

        pos.push(startpos - 1);
    }

    let mut scores: Vec<u16> = Vec::new();
    for p in &pos {
        scores.push(0);
    }

    println!("{:?}", pos);

    println!("{:?}", scores);
    let mut dicenext = 1;
    let mut dicerolls = 0;
    while scores[0] < 1000 && scores[1] < 1000 {
        for p in 0..pos.len() {
            let mut dicesum = 0;

            for j in 0..3 {
                println!("Throwing dice sum is {}, next is {}", dicesum, dicenext);
                dicesum += dicenext;
                dicenext = (dicenext + 1) % 1000;
                dicerolls += 1;
            }

            let newpos = (pos[p] + dicesum) % 10;
            scores[p] += newpos + 1;
            pos[p] = newpos;

            println!(
                "Player {} has {} on {} dices were {}",
                p,
                scores[p],
                pos[p] + 1,
                dicesum
            );

            if scores[p] >= 1000 {
                break;
            }
        }
    }

    println!("{:?}", scores);
    println!("{}", dicerolls);
}
