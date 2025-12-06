use std::cmp::Ordering;
use std::fs;

fn read_input() -> Vec<(i64, i64)> {
    let input = fs::read_to_string("./src/input/5.txt").expect("Can't read");
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    parts[0]
        .lines()
        .map(|line| {
            let mut nums = line
                .split('-')
                .map(|v| v.parse::<i64>().expect("Not a number"))
                .collect::<Vec<i64>>()
                .into_iter();
            let first = nums.next().expect("Invalid range");
            let second = nums.next().expect("Invalid range");

            (first, second)
        })
        .collect()
}

const fn merge_ranges(
    earlier_start: (i64, i64),
    later_start: (i64, i64),
) -> ((i64, i64), Option<(i64, i64)>) {
    if earlier_start.0 == later_start.0 {
        if earlier_start.1 >= later_start.1 {
            return (earlier_start, None);
        }
        return (later_start, None);
    }

    if earlier_start.1 >= later_start.1 {
        return (earlier_start, None);
    }

    if earlier_start.1 < later_start.0 {
        // Do not overlap
        return (earlier_start, Some(later_start));
    }

    ((earlier_start.0, later_start.0 - 1), Some(later_start))
}

pub fn run() {
    let mut input = read_input();
    let mut count = 0;

    // Sort is acsending for first and decending for second
    input.sort_unstable_by(|n, i| {
        let cmp = n.0.cmp(&i.0);
        if cmp != Ordering::Equal {
            return cmp;
        }
        n.1.cmp(&i.1).reverse()
    });

    let mut index = 0;
    while index < input.len() - 1 {
        let merge = merge_ranges(input[index], input[index + 1]);
        input[index] = merge.0;
        if let Some(p) = merge.1 {
            input[index + 1] = p;
            index += 1;
        } else {
            input.remove(index + 1);
        }
    }

    for ele in input {
        count += 1 + ele.1 - ele.0;
    }

    println!("Answer: {count}");
}
