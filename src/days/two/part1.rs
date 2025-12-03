use std::fs;

fn read_input() -> Vec<(i64, i64)> {
    let input = fs::read_to_string("./src/input/2.txt").expect("Can't read");
    input
        .split(',')
        .map(|v| {
            let vals = v
                .split('-')
                .map(|v| v.parse::<i64>().expect("Not a number"))
                .collect::<Vec<i64>>();
            (vals[0], vals[1])
        })
        .collect::<Vec<(i64, i64)>>()
}

fn is_invalid(num: i64) -> bool {
    let num_str = num.to_string();
    let len = num_str.len();
    len.is_multiple_of(2) && num_str[..(len / 2)] == num_str[(len / 2)..]
}

pub fn run() {
    let mut total = 0;
    let input = read_input();
    for range in input {
        for i in range.0..=range.1 {
            if is_invalid(i) {
                total += i;
            }
        }
    }
    println!("Answer: {total}");
}
