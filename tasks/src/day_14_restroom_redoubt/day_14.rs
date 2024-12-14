use std::collections::{HashMap, HashSet};

use regex::Regex;

fn get_data() -> Vec<((isize, isize), (isize, isize))> {
    let re = Regex::new(r"-?\d+").unwrap();
    include_str!("./input.txt")
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
        .collect::<Vec<_>>()
}

const ROWS: isize = 103;
const COLUMNS: isize = 101;

pub fn part_1() -> usize {
    let data = get_data();
    let seconds = 100;
    let mut squadrons = [0; 4];

    for (position, direction) in data.iter() {
        let mut new_row = position.0 + direction.0 * seconds;
        new_row = if new_row < 0 {
            let rem = new_row % ROWS;
            if rem < 0 {
                ROWS + rem
            } else {
                0
            }
        } else {
            new_row % ROWS
        };

        let mut new_column = position.1 + direction.1 * seconds;
        new_column = if new_column < 0 {
            let rem = new_column % COLUMNS;
            if rem < 0 {
                COLUMNS + rem
            } else {
                0
            }
        } else {
            new_column % COLUMNS
        };

        match (new_row + 1).cmp(&(ROWS / 2 + 1)) {
            std::cmp::Ordering::Less => match (new_column + 1).cmp(&(COLUMNS / 2 + 1)) {
                std::cmp::Ordering::Less => squadrons[0] += 1,
                std::cmp::Ordering::Equal => (),
                std::cmp::Ordering::Greater => squadrons[1] += 1,
            },
            std::cmp::Ordering::Equal => (),
            std::cmp::Ordering::Greater => match (new_column + 1).cmp(&(COLUMNS / 2 + 1)) {
                std::cmp::Ordering::Less => squadrons[2] += 1,
                std::cmp::Ordering::Equal => (),
                std::cmp::Ordering::Greater => squadrons[3] += 1,
            },
        }
    }

    squadrons.iter().product::<usize>()
}

fn check_is_tree(data_map: &HashMap<isize, HashSet<isize>>, position: (isize, isize)) -> bool {
    for row_i in 0..3 {
        if let Some(col_set) = data_map.get(&(position.0 + row_i)) {
            for col_i in 0..3 {
                if !col_set.contains(&(position.1 + col_i)) {
                    return false;
                }
            }
        } else {
            return false;
        }
    }

    true
}

pub fn part_2() -> usize {
    let mut data = get_data();
    let mut seconds = 0;

    'outer: loop {
        seconds += 1;
        let mut data_map = HashMap::<isize, HashSet<isize>>::new(); // line: set of columns
        for (position, direction) in data.iter_mut() {
            let mut new_row = position.0 + direction.0;
            new_row = match new_row.cmp(&0) {
                std::cmp::Ordering::Less => ROWS + new_row,
                std::cmp::Ordering::Equal => new_row,
                std::cmp::Ordering::Greater => match new_row.cmp(&ROWS) {
                    std::cmp::Ordering::Greater => new_row - ROWS,
                    std::cmp::Ordering::Equal => 0,
                    _ => new_row,
                },
            };

            let mut new_col = position.1 + direction.1;
            new_col = match new_col.cmp(&0) {
                std::cmp::Ordering::Less => COLUMNS + new_col,
                std::cmp::Ordering::Equal => new_col,
                std::cmp::Ordering::Greater => match new_col.cmp(&COLUMNS) {
                    std::cmp::Ordering::Greater => new_col - COLUMNS,
                    std::cmp::Ordering::Equal => 0,
                    _ => new_col,
                },
            };
            position.0 = new_row;
            position.1 = new_col;

            data_map
                .entry(new_row)
                .or_insert_with(HashSet::new)
                .insert(new_col);
        }

        for (row, columns) in &data_map {
            for column in columns {
                let position = (*row, *column);
                if check_is_tree(&data_map, position) {
                    break 'outer;
                }
            }
        }
    }

    seconds
}
