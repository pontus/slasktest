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

            let n = usize::from_str_radix(&s[i..j], 16).unwrap();
            v.push(n as u8);
            i = j;
        }

        let mut b = get_bits(0, (i / 2) * 8, &v);

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
    //     "Working on {:?}, version is {}, ptype {}",
    //     v, version, ptype
    // );

    if ptype == 4 {
        // literal
        // println!("Literal");

        let mut plength = 6;

        while to_num(&v[plength..plength + 1]) != 0 {
            plength += 5;
        }

        // println!("Consumed {}", plength + 5);

        return (version, plength + 5);
    }

    let mut versionsum = version;

    // operator here
    let lengthtypeid = to_num(&v[6..7]);

    if lengthtypeid == 0 {
        let plength = to_num(&v[7..22]) as usize;
        // println!("Length in bits to consume packets {}", plength);

        let mut consumed: usize = 0;

        while consumed < plength {
            let (a, b) = handle_packet(&v[22 + consumed..]);
            consumed += b;
            versionsum += a;
            // println!("Consumed {} plength {} ", consumed, plength);
        }

        // println!("Returning from packet length consumption {}", consumed);

        return (versionsum, 22 + plength as usize);
    }

    // sub packets
    let mut packets_to_do = to_num(&v[7..18]);
    let mut consumed: usize = 0;

    // println!("Length in packets {}", packets_to_do);

    while packets_to_do > 0 {
        let (a, b) = handle_packet(&v[18 + consumed..]);
        consumed += b;
        versionsum += a;
        packets_to_do -= 1;
    }
    //            return handle_packet(&v[18..22 + plength as usize]);
    // println!("Returning from packet length consumption {}", consumed);

    return (versionsum, consumed + 18);
}

fn get_bits(start: usize, end: usize, v: &Vec<u8>) -> Vec<bool> {
    let mut bits = Vec::new();
    for i in start..end {
        let mut b = v[i / 8];
        b = b >> (7 - (i % 8));
        b = b & 1;
        bits.push(b == 1);
    }
    return bits;
}
