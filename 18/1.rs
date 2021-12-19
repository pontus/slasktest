use std::cmp::max;
use std::fs::File;
use std::io::{self, BufRead};
use std::ptr;

#[derive(Debug, Clone)]
struct SNumber {
    regular_left: isize,
    pair_left: Vec<SNumber>,
    regular_right: isize,
    pair_right: Vec<SNumber>,
}

fn main() {
    let mut lines = io::BufReader::new(File::open("input.txt").unwrap()).lines();

    //let start = lines.next().unwrap().unwrap();

    for l in lines {
        let s = l.unwrap().clone();
        //        println!("Parsing {}", s);
        let (n, i) = parse_number(&s);
        //        println!("{:?} length {}", n, i);

        println!("{:?}", n);

        let (s, b) = explode(n);
        println!("{:?}", s);
    }

    // let s = SNumber {
    //     regular_left: 24,
    //     pair_left: Vec::new(),
    //     regular_right: 4,
    //     pair_right: Vec::new(),
    // };

    // let mut cont = true;
    // let mut o = s.clone();
    // while cont {
    //     let (a, b) = split(o);
    //     cont = b;
    //     o = a.clone();
    // }
    // println!("{:?} {:?}", o, explode(o.clone()).0);
}

fn split(s: SNumber) -> (SNumber, bool) {
    let mut rets = s.clone();

    if s.pair_left.len() == 0 {
        if s.regular_left > 9 {
            rets.regular_left = 0;
            rets.pair_left.push(SNumber {
                regular_left: s.regular_left / 2,
                pair_left: Vec::new(),
                regular_right: s.regular_left - s.regular_left / 2,
                pair_right: Vec::new(),
            });
            return (rets, true);
        }
    } else {
        let (s1, done_now) = split(s.pair_left[0].clone());

        if done_now {
            rets.pair_left.remove(0);
            rets.pair_left.push(s1);
            return (rets, done_now);
        }
    }

    if s.pair_right.len() == 0 {
        if s.regular_right > 9 {
            rets.regular_right = 0;
            rets.pair_right.push(SNumber {
                regular_left: s.regular_right / 2,
                pair_left: Vec::new(),
                regular_right: s.regular_right - s.regular_right / 2,
                pair_right: Vec::new(),
            });
            return (rets, true);
        }
    } else {
        let (s1, done_now) = split(s.pair_right[0].clone());

        if done_now {
            rets.pair_right.remove(0);
            rets.pair_right.push(s1);
            return (rets, done_now);
        }
    }

    return (s, false);
}

fn find_depth(s: &SNumber, depth: usize) -> Vec<(usize, &SNumber)> {
    let mut ret = Vec::new();

    println!("Find depth called with {:?}", s);
    if s.pair_left.len() == 0 && s.pair_right.len() == 0 {
        ret.push((depth, s));
        println!("Returning {:?} from regular", ret);

        return ret;
    }

    println!("Ret is {:?} in between", ret);

    if s.pair_left.len() > 0 {
        let mut l = find_depth(&s.pair_left[0], depth + 1);
        ret.append(&mut l);
        ret.push((depth, s));
        println!("Ret is {:?} after left", ret);
    }
    if s.pair_right.len() > 0 {
        let mut r = find_depth(&s.pair_right[0], depth + 1);
        ret.push((depth, s));

        ret.append(&mut r);
        println!("Ret is {:?} after right", ret);
    }

    println!("Returning {:?}", ret);

    return ret;
}

fn replace_left(s: &mut SNumber, obj: &SNumber, newval : isize) {
    if ptr::eq(s as &SNumber, obj) {
        s.regular_left = obj.regular_left;
        s.pair_left = obj.pair_left.clone();
    } else {
        if s.pair_left.len() > 0 {
            replace_left(&mut s.pair_left[0], obj, newval);
        }
        if s.pair_right.len() > 0 {
            replace_left(&mut s.pair_right[0], obj, newval);
        }
    }
}

fn explode(s: SNumber) -> (SNumber, bool) {
    let mut rets = s.clone();
    let mut d = find_depth(&rets, 0);
    let i = 0;

    while i < d.len() {
        if d[i].0 <= 4 {
            let left = d[i].1.regular_left;
            let right = d[i].1.regular_right;

            if i > 0 {
                let leftof = d[i - 1].1;
                replace_left(&mut rets, leftof, leftof.regular_right + left);
                //    let &mut leftsn = leftof;
                //  leftof.regular_left+=left;
            }

            if i < d.len() - 1 {
                let rightof = d[i + 1].1;
                //  d[i+1].1.regular_left += right;
            }

            return (rets, true);
        }
    }

    // Shouldn't get here
    return (rets, false);
}

fn reduce(s: SNumber) -> SNumber {
    return s;
}

fn parse_number(s: &str) -> (SNumber, usize) {
    let mut pair_left: Vec<SNumber> = Vec::new();
    let mut pair_right: Vec<SNumber> = Vec::new();

    let mut i = 0;

    let mut sn = SNumber {
        regular_left: 0,
        pair_left: pair_left,
        regular_right: 0,
        pair_right: pair_right,
    };

    // println!("Parsing number {}", s);
    if s.chars().nth(0).unwrap() != '[' {
        return (sn.clone(), 0);
    }

    if s.chars().nth(1).unwrap() == '[' {
        // println!("Left subnumber beginning at 1, {}", &s[1..]);
        let (left, consumed) = parse_number(&s[1..]);
        sn.pair_left.push(left.clone());

        i = consumed + 3;
        // println!("Remainder: {}", &s[i..]);
    } else {
        let comma = s.find(',').unwrap();
        let n: isize = s[1..comma].parse().unwrap();

        sn.regular_left = n;

        i = comma + 1;
        // println!("parsed {} until {}", &s[1..comma], i);
    }

    // println!("Doing right side on {}", &s[i..]);

    if s.chars().nth(i).unwrap() == '[' {
        // println!("Right subnumber beginning at {}, {}", i, &s[i..]);

        let (right, consumed) = parse_number(&s[i..]);
        sn.pair_right.push(right.clone());

        i = i + consumed + 1;
    } else {
        let bracket = s[i..].find(']').unwrap() + i;
        // println!("will parse between {} and {}", i, bracket);

        // println!("will parse '{}', index {}", &s[i..bracket], i);
        let n: isize = s[i..bracket].parse().unwrap();

        sn.regular_right = n;
        i = bracket;
    }

    // println!("Returning {:?} consumed {} bytes", sn, i + 1);
    return (sn.clone(), i);
}
