use std::cmp::max;
use std::cmp::min;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut lines = io::BufReader::new(File::open("input.txt").unwrap()).lines();

    for l in lines {
        let mut s = l.unwrap();
        //    s = s.trim();
        let mut i = 0;
        let mut v = Vec::new();
        println!("{}", s);

        while i < s.len() {
            let mut j = i + 1;
            if i + 2 <= s.len() {
                j = i + 2;
            }
            //println!("Parsing {}", &s[i..j]);
            let n = usize::from_str_radix(&s[i..j], 16).unwrap();
            //println!("{:?}", n);

            v.push(n as u8);
            i = j;
        }

        let mut b = get_bits(&v);

        println!("{}", handle_packet(&b).0);
        // println!("{:?}", v);
    }
    //    println!("{:?}", get_bits(0, 3, v));
}

fn to_num(v: &[bool]) -> u64 {
    let mut n = 0;
    for i in 0..v.len() {
        n = n << 1;

        if v[i] {
            n += 1;
        }
    }

    return n;
}

fn handle_packet(v: &[bool]) -> (u64, usize) {
    let version = to_num(&v[0..3]);
    let ptype = to_num(&v[3..6]);

    // println!(
    //     "Working on {:?}, version is {}, ptype {} length is {}",
    //     v,
    //     version,
    //     ptype,
    //     v.len()
    // );

    if ptype == 4 {
        // literal
        // println!("Literal");

        let mut plength = 6;
        let mut num = 0;
        while to_num(&v[plength..plength + 1]) != 0 {
            num *= 16;
            num += to_num(&v[plength + 1..plength + 5]);
            plength += 5;
        }

        num *= 16;
        num += to_num(&v[plength + 1..plength + 5]);

        // println!("Consumed {}", plength + 5);
        // println!("Returning literal {}", num);
        return (num, plength + 5);
    }

    let mut num = 0;

    // println!("Operator is {}", ptype);
    // operator here
    let lengthtypeid = to_num(&v[6..7]);

    if lengthtypeid == 0 {
        let plength = to_num(&v[7..22]) as usize;
        // println!("Length in bits to consume packets {}", plength);

        let mut consumed: usize = 0;

        while consumed < plength {
            let (a, b) = handle_packet(&v[22 + consumed..]);

            if consumed > 0 {
                if ptype < 4 {
                    num = operator(num, a, ptype);
                } else {
                    num = cmpoperator(num, a, ptype);
                }
            } else {
                num = a;
            }

            consumed += b;

            // println!("Consumed {} plength {} ", consumed, plength);
        }

        // println!("Returning from packet length consumption {}", consumed);
        // println!("Returned value from length consumption is {}", num);
        return (num, 22 + plength as usize);
    }

    // sub packets
    let mut packets_to_do = to_num(&v[7..18]);
    let mut consumed: usize = 0;

    // println!("Length in packets {}", packets_to_do);

    while packets_to_do > 0 {
        // println!("Consumed is {}", consumed);
        let (a, b) = handle_packet(&v[18 + consumed..]);

        if consumed > 0 {
            if ptype < 4 {
                num = operator(num, a, ptype);
            } else {
                num = cmpoperator(num, a, ptype);
            }
        } else {
            num = a;
        }

        consumed += b;
        packets_to_do -= 1;
    }
    // println!("Returning from packet length consumption {}", consumed);
    // println!("Returned value from packet count is {}", num);

    return (num, consumed + 18);
}

fn cmpoperator(a: u64, b: u64, op: u64) -> u64 {
    if op == 5 {
        if a > b {
            return 1;
        }
        return 0;
    }

    if op == 6 {
        if a < b {
            return 1;
        }
        return 0;
    }

    if op == 7 {
        if a == b {
            return 1;
        }
        return 0;
    }

    return 0;
}

fn operator(a: u64, b: u64, op: u64) -> u64 {
    if op == 0 {
        return a + b;
    }
    if op == 1 {
        return a * b;
    }
    if op == 2 {
        return min(a, b);
    }
    if op == 3 {
        return max(a, b);
    }

    return 0;
}

fn get_bits(v: &Vec<u8>) -> Vec<bool> {
    let mut bits = Vec::new();
    for i in 0..v.len() * 8 {
        // println!("Getting bits for {:?} index {}", v, i);
        let mut b = v[i / 8];
        //println!("Got byte  {}", b);

        b = b >> (7 - (i % 8));
        b = b & 1;
        bits.push(b == 1);
    }
    return bits;
}
