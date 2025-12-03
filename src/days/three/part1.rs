use std::fs;

fn get_data() -> Vec<Vec<i32>> {
    let input = fs::read_to_string("./src/input/3.txt").expect("Can't read");
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|n| n.to_digit(10).expect("Can't parse").cast_signed())
                .collect()
        })
        .collect::<Vec<Vec<i32>>>()
}

fn find_output_joltage(vec: &[i32]) -> i32 {
    let mut largest_digit = vec[0];
    let mut second_digit = vec[1];
    for i in 1..(vec.len() - 1) {
        let ele = vec[i];
        if ele > largest_digit {
            largest_digit = ele;
            second_digit = vec[i + 1];
        } else if ele > second_digit {
            second_digit = ele;
        }
    }
    let last = *vec.last().expect("No last element");
    if last > second_digit {
        second_digit = last;
    }
    largest_digit * 10 + second_digit
}

pub fn run() {
    let data = get_data();
    let mut total = 0;
    for pack in data {
        let jolt = find_output_joltage(&pack);
        total += jolt;
    }
    println!("Total joltage: {total}");
}
