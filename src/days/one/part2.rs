use core::panic;
use std::fs;

fn read_input() -> Vec<(char, i32)> {
    let input = fs::read_to_string("./src/input/1.txt").expect("Can't read");
    input
        .split('\n')
        .map(|v| {
            let mut chars = v.chars();
            let first = chars.next().expect("Empty line");
            let num: i32 = chars.collect::<String>().parse().expect("Not a number");
            (first, num)
        })
        .collect::<Vec<(char, i32)>>()
}

struct Dial {
    num: i32,
    zero_count: i32,
}

impl Dial {
    fn right(&mut self, amount: i32) {
        for _ in 0..amount {
            if self.num == 99 {
                self.num = -1;
            }
            self.num += 1;
            if self.num == 0 {
                self.zero_count += 1;
            }
        }
    }

    fn left(&mut self, amount: i32) {
        for _ in 0..amount {
            if self.num == 0 {
                self.num = 100;
            }
            self.num -= 1;
            if self.num == 0 {
                self.zero_count += 1;
            }
        }
    }

    fn new() -> Self {
        Dial {
            num: 50,
            zero_count: 0,
        }
    }
}

pub fn run() {
    let mut dial = Dial::new();
    let input = read_input();
    for entry in input {
        match entry.0 {
            'L' => dial.left(entry.1),
            'R' => dial.right(entry.1),
            _ => panic!("Unknown direction"),
        }
    }
    println!("Answer: {}", dial.zero_count)
}
