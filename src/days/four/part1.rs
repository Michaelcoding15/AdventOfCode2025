use std::fs;

#[derive(Debug)]
enum Space {
    Roll,
    Empty,
}

fn read_input() -> Vec<Vec<Space>> {
    let input = fs::read_to_string("./src/input/4.txt").expect("Can't read");
    input
        .lines()
        .map(|v| {
            v.chars()
                .map(|l| match l {
                    '.' => Space::Empty,
                    '@' => Space::Roll,
                    _ => panic!("Invalid letter"),
                })
                .collect()
        })
        .collect::<Vec<Vec<Space>>>()
}

fn accessable(index: (usize, usize), floor: &[Vec<Space>]) -> bool {
    if matches!(floor[index.0][index.1], Space::Empty) {
        return false;
    }
    let points = [
        (index.0, index.1.wrapping_sub(1)),
        (index.0, index.1 + 1),
        (index.0.wrapping_sub(1), index.1.wrapping_sub(1)),
        (index.0.wrapping_sub(1), index.1),
        (index.0.wrapping_sub(1), index.1 + 1),
        (index.0 + 1, index.1.wrapping_sub(1)),
        (index.0 + 1, index.1),
        (index.0 + 1, index.1 + 1),
    ];
    points
        .map(|v| get_point(v, floor))
        .iter()
        .filter(|v| v.is_some_and(|v| matches!(*v, Space::Roll)))
        .count()
        < 4
}

fn get_point(index: (usize, usize), floor: &[Vec<Space>]) -> Option<&Space> {
    floor.get(index.0).and_then(|v| v.get(index.1))
}

pub fn run() {
    let input = read_input();
    let mut count = 0;
    println!("{input:?}");
    for (index_row, row) in input.iter().enumerate() {
        for (index_ele, _) in row.iter().enumerate() {
            if accessable((index_row, index_ele), &input) {
                count += 1;
            }
        }
    }
    println!("{count}");
}
