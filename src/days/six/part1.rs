use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
enum Operation {
    Addition,
    Multiplication,
}

impl Operation {
	const fn from(char: char) -> Self {
		match char {
				'+' => Self::Addition,
				'*' => Self::Multiplication,
				_ => panic!()
		}
	}
}

fn calculate_numbers(nums: &HashMap<usize, Vec<char>>, operation: &Operation) -> u64 {
    let mut calculated_nums: Vec<u64> = vec![0];
    for line in nums.values() {
        let length = line.len() - 1;
        for (digit, num) in line.iter().enumerate() {
            let index = calculated_nums.len() - 1;
            calculated_nums[index] +=
                u64::from(num.to_digit(10).expect("NAN")) * 10_u64.pow((length - digit).try_into().unwrap());
        }
        calculated_nums.push(0);
    }
    calculated_nums.pop();
    match operation {
        Operation::Addition => calculated_nums.iter().sum(),
        Operation::Multiplication => calculated_nums.iter().product(),
    }
}

pub fn run() {
    let input = fs::read_to_string("./src/input/6.txt")
        .expect("Can't read")
        .lines()
        .map(|v| v.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let mut total: u64 = 0;
    let mut nums: HashMap<usize, Vec<char>> = HashMap::new();
    let mut first_index_found = usize::MAX;
    for i in 0..input[0].len() {
        let mut num_found = false;
        for (j, line) in input.iter().enumerate().take(4) {
            let num = line[i];
            if num.is_numeric() {
                nums.entry(j).or_default().push(num);
                num_found = true;
                if first_index_found == usize::MAX {
                    first_index_found = i;
                }
            }
        }

        if !num_found {
            total += calculate_numbers(&nums, &Operation::from(input[4][first_index_found]));
            nums.clear();
						first_index_found = usize::MAX;
        }
    }
    total += calculate_numbers(&nums, &Operation::from(input[4][first_index_found]));
    println!("{total}");
}
