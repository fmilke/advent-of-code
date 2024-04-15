use std::env;

const INPUT: &str = include_str!("../input");

type Generation = usize;
type Lut = [Generation; 100];

const PREFIX_LEN: usize = "Card XXX:".len();

fn read_winning_numbers<'a>(s: &'a str, map: &mut Lut, gen: Generation) -> &'a str {

    let (left, right) = s[PREFIX_LEN..]
        .split_once("|").unwrap();

    left
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .for_each(|a| map[a] = gen);

    right
}

fn first_part() {
    let mut map: &mut [Generation; 100] = &mut [0; 100];
    let mut total = 0;

    for (gen, l) in INPUT.lines().enumerate() {
        let gen = gen + 1;
        let actual_numbers = read_winning_numbers(l, &mut map, gen);
        let mut points = 1;

        actual_numbers
            .split_whitespace()
            .for_each(|s| {
                let v = s.parse::<usize>().unwrap();
                if map[v] == gen {
                    points <<= 1;
                }
            });

        points >>= 1;
        total += points;


        continue;
        println!("line: {}; points: {}", l, points);

        if gen > 11 {
            break;
        }

    }

    println!("Part one: Total points: {}", total);
}

fn second_part() {
}

fn main() {

    let do_second_part = env::args().any(|s| s == "-2");

    if do_second_part {
        second_part();
    } else {
        first_part();
    }
}
