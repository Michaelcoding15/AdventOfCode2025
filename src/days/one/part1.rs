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
}

impl Dial {
    const fn right(&mut self, amount: i32) {
        self.num = (self.num + amount).rem_euclid(100);
    }

    const fn left(&mut self, amount: i32) {
        self.num = (self.num - amount).rem_euclid(100);
    }

    const fn new() -> Self {
        Self { num: 50 }
    }
}

pub fn run() {
    let mut dial = Dial::new();
    let input = read_input();
    let mut count = 0;
    for entry in input {
        match entry.0 {
            'L' => dial.left(entry.1),
            'R' => dial.right(entry.1),
            _ => panic!("Unknown direction"),
        }
        if dial.num == 0 {
            count += 1;
        }
    }
    println!("Answer: {count}");
}
