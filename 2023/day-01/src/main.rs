use regex::Regex;

const INPUT: &str = include_str!("../input");

fn is_digit(n: u8) -> bool {
    return 47 < n && n < 58;
}

fn part_one() {
    let mut sum = 0;
    for line in INPUT.lines() {

        println!("{}", line);

        let mut first: u32 = 0;
        let mut second: u32 = 0;

        let bytes = line.as_bytes();
        let len = bytes.len();
        let mut offset = 0;

        for i in 0..len {
            let b = bytes[i];
            if is_digit(b) {
                first = 10 * ((b as u32) - 48);
                offset = i;
                break;
            }
        }

        for i in (offset..len).rev() {
            print!("{},", i);
            let b = bytes[i];
            if is_digit(b) {
                second = (b - 48) as u32;
                break;
            }
        }

        println!();
        println!("First and second: {}, {}", first, second);

        sum += first + second;
    }

    println!("Part one: Sum is: {}", sum);
}


/*
one
two
three
four
five
six
seven
eight
nine
zero
*/

fn get_value(s: &str) -> i32 {
    return match s {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        _ => 0,
    }
}

fn part_two() {
    let re = Regex::new(r".*?(one|two|three|four|five|six|seven|eight|nine|zero|\d)").unwrap();
    let re2 = Regex::new(r".*(one|two|three|four|five|six|seven|eight|nine|zero|\d)").unwrap();

    let mut sum = 0;

    for l in INPUT.lines() {
        let mut first = 0;
        let mut second = 0;

        println!("{}", l);

        match re.captures(l) {
            Some(c) => {
                let (_, [num]) = c.extract();
                let v = get_value(num);
                first = v;
                print!("First: {} ({}), ", num, v);
            },
            None => {},
        }

        match re2.captures(l) {
            Some(c) => {
                let (_, [num]) = c.extract();
                let v = get_value(num);
                second = v;
                println!("Second: {} ({})", num, v);
            },
            None => {},
        }


        sum += first * 10 + second;
    }

    println!("Part two: Sum is: {}", sum);
}

fn main() {
    if false {
        part_one();
    } else {
        part_two();
    }
}
