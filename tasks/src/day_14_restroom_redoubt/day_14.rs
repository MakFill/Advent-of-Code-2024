use regex::Regex;

pub fn part_1() -> usize {
    let re = Regex::new(r"-?\d+").unwrap();
    let data = include_str!("./input.txt")
        .lines()
        .map(|line| {
            let entry = line
                .split_ascii_whitespace()
                .map(|pair| {
                    let item = re
                        .find_iter(pair)
                        .map(|i| i.as_str().parse::<isize>().unwrap())
                        .collect::<Vec<_>>();
                    (item[1], item[0])
                })
                .collect::<Vec<_>>();
            (entry[0], entry[1])
        })
        .collect::<Vec<_>>();

    let seconds = 100;
    let rows = 103;
    let columns = 101;
    let mut squadrons = [0; 4];

    for (position, direction) in data.iter() {
        let mut new_row = position.0 + direction.0 * seconds;
        new_row = if new_row < 0 {
            let rem = new_row % rows;
            if rem < 0 {
                rows + rem
            } else {
                0
            }
        } else {
            new_row % rows
        };

        let mut new_column = position.1 + direction.1 * seconds;
        new_column = if new_column < 0 {
            let rem = new_column % columns;
            if rem < 0 {
                columns + rem
            } else {
                0
            }
        } else {
            new_column % columns
        };

        match (new_row + 1).cmp(&(rows / 2 + 1)) {
            std::cmp::Ordering::Less => match (new_column + 1).cmp(&(columns / 2 + 1)) {
                std::cmp::Ordering::Less => squadrons[0] += 1,
                std::cmp::Ordering::Equal => (),
                std::cmp::Ordering::Greater => squadrons[1] += 1,
            },
            std::cmp::Ordering::Equal => (),
            std::cmp::Ordering::Greater => match (new_column + 1).cmp(&(columns / 2 + 1)) {
                std::cmp::Ordering::Less => squadrons[2] += 1,
                std::cmp::Ordering::Equal => (),
                std::cmp::Ordering::Greater => squadrons[3] += 1,
            },
        }
    }

    squadrons.iter().product::<usize>()
}

pub fn part_2() -> usize {
    0
}
