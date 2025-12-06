use std::cmp::Ordering;
use std::fs;

fn greater_than_eq(num: &str, compare: &str) -> bool {
    match num.len().cmp(&compare.len()) {
        Ordering::Greater => true,
        Ordering::Less => false,
        Ordering::Equal => {
            for (i, char) in num.chars().enumerate() {
                let compare_char = compare.chars().nth(i).expect("Lengths are not the same");
								match char.cmp(&compare_char) {
									Ordering::Greater => return true,
									Ordering::Less => return false,
									Ordering::Equal => ()
								}
            }
            true
        }
    }
}

fn less_than_eq(num: &str, compare: &str) -> bool {
	match num.len().cmp(&compare.len()) {
			Ordering::Greater => false,
			Ordering::Less => true,
			Ordering::Equal => {
					for (i, char) in num.chars().enumerate() {
							let compare_char = compare.chars().nth(i).expect("Lengths are not the same");
							match char.cmp(&compare_char) {
								Ordering::Greater => return false,
								Ordering::Less => return true,
								Ordering::Equal => ()
							}
					}
					true
			}
	}
}


fn read_input() -> (Vec<(String, String)>, Vec<String>) {
    let input = fs::read_to_string("./src/input/5.txt").expect("Can't read");
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let fresh_id_ranges = parts[0]
        .lines()
        .map(|line| {
            let mut nums = line
                .split('-')
                .map(std::string::ToString::to_string)
                .collect::<Vec<String>>()
                .into_iter();
            let first = nums.next().expect("Invalid range");
            let second = nums.next().expect("Invalid range");

            (first, second)
        })
        .collect::<Vec<(String, String)>>();

    let ingredient_ids = parts[1]
        .lines()
        .map(std::string::ToString::to_string)
        .collect::<Vec<String>>();
    (fresh_id_ranges, ingredient_ids)
}

pub fn run() {
    let mut count = 0;

    let (fresh, ingredients) = read_input();

    'outer: for ingredient in ingredients {
        for range in &fresh {
            if greater_than_eq(&ingredient, &range.0) && less_than_eq(&ingredient, &range.1) {
                count += 1;
                continue 'outer;
            }
        }
    }

    println!("Answer: {count}");
}
