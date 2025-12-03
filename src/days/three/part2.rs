use std::fs;

fn get_data() -> Vec<Vec<i64>> {
    let input = fs::read_to_string("./src/input/3.txt").expect("Can't read");
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|n| i64::from(n.to_digit(10).expect("Can't parse")))
                .collect()
        })
        .collect::<Vec<Vec<i64>>>()
}

fn find_output_joltage(vec: &[i64]) -> i64 {
    let mut digits: Vec<i64> = vec![];
    let length = vec.len();

    for (i, ele_ref) in vec.iter().enumerate().take(length) {
        let ele = *ele_ref;
        let digit_index = if i >= length - 11 {
            i - (length - 12)
        } else {
            0
        };
        'inner: for j in digit_index..12 {
            let Some(digit) = digits.get(j) else {
                digits.push(ele);
                break 'inner;
            };
            if ele > *digit {
                digits.drain(j..);
                digits.push(ele);
                break 'inner;
            }
        }
    }

    let num_digits = digits.len() - 1;

    let mut total: i64 = 0;
    for digit in digits.iter().enumerate() {
        total += digit.1
            * 10_i64.pow(
                (num_digits - digit.0)
                    .try_into()
                    .expect("Can't convert to u32"),
            );
    }

    total
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
