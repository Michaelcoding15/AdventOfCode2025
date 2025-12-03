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

fn get_factors(num: i64) -> Vec<i64> {
    #[allow(clippy::cast_possible_wrap)]
    // Will not get to size for a overflow
    let length = num.to_string().len() as i64;
    let mut factors = vec![1];
    for i in 2..=length.isqrt() {
        if length % i == 0 {
            factors.push(i);
            if i != length / i {
                factors.push(length / i);
            }
        }
    }
    factors.sort_unstable();
    factors
}

fn is_invalid(num: i64) -> bool {
    let num_str = num.to_string();
    let factors = get_factors(num);
    for factor in factors {
        let chars = num_str.chars().collect::<Vec<char>>();
        let parts = chars
            .chunks(factor.try_into().expect("i64 not a usize"))
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>();
        if parts.len() > 1 && parts.windows(2).all(|v| v[0] == v[1]) {
            return true;
        }
    }
    false
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
