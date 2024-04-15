use std::env;

const INPUT: &str = include_str!("../input");


fn parse_line(l: &str, out: &mut [u32]) {
    let mut i = 0usize;
    let bytes = l.as_bytes();
    let end = bytes.len();

    for k in 0..end {
        for _ in 0..3 {
            print!("{}", bytes[k]);
        }
    }

    loop {
        let c = bytes[i];
        i += 1;
        if c.is_ascii_digit() {

            let start = i - 1;

            loop {
                let c2 = bytes[i];

                println!("c2: {}, is_ascii_digit: {}, i: {}", c2, c2.is_ascii_digit(), i);

                if i == end || !c2.is_ascii_digit() {
                    let sub = &l[start..i];

                    print!("sub: {}; start: {}; end: {};", sub, start, i);

                    let v = sub.parse::<u32>().unwrap();
                    print!(" parsed: {}, ", v);

                    for k in start..i {
                        out[k] = v;
                    }

                    break;
                }

                i += 1;
            }

            if i >= end {
                break;
            }
        }
    }

    println!("");

    for k in 0..bytes.len() {
        print!("{:03}", out[k].to_string());
    }

    let mut s = String::new();
    for i in 0..bytes.len() {
        s += i.to_string().as_str();
    }
    println!("line: {}", s);
}

fn part_one() {

    let mut upper = &mut [0u32; 200];
    let mut current = &mut [0u32; 200];
    let mut lower = &mut [0u32; 200];

    for l in INPUT.lines() {
        parse_line(l, current);

        // Prepare pointers for next iteration
        let tmp = upper;
        upper = current;
        current = lower;
        tmp.fill(0u32);
        lower = tmp;
    }
}

fn part_two() {
}

fn main() {
    if env::args().any(|s| s == "-2") {
        part_two();
    } else {
        part_one();
    }
}

