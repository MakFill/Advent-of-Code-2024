use std::collections::{HashMap, HashSet};

fn dfs_paths(
    data: &[Vec<u32>],
    start: (usize, usize),
    curr_position: (usize, usize),
    row_last_index: usize,
    column_last_index: usize,
    trail_map: &mut HashMap<(usize, usize), HashSet<(usize, usize)>>,
) {
    if data[curr_position.0][curr_position.1] == 9 {
        trail_map
            .entry(start)
            .or_insert_with(HashSet::new)
            .insert(curr_position);
    } else {
        let item = data[curr_position.0][curr_position.1];
        let next_positions = [
            match curr_position.0 != 0 {
                true => Some((curr_position.0 - 1, curr_position.1)),
                false => None,
            },
            match curr_position.0 != row_last_index {
                true => Some((curr_position.0 + 1, curr_position.1)),
                false => None,
            },
            match curr_position.1 != 0 {
                true => Some((curr_position.0, curr_position.1 - 1)),
                false => None,
            },
            match curr_position.1 != column_last_index {
                true => Some((curr_position.0, curr_position.1 + 1)),
                false => None,
            },
        ];
        next_positions.iter().for_each(|i| {
            if let Some(next_position) = i {
                if item + 1 == data[next_position.0][next_position.1] {
                    dfs_paths(
                        data,
                        start,
                        *next_position,
                        row_last_index,
                        column_last_index,
                        trail_map,
                    );
                }
            }
        });
    }
}

pub fn part_1() -> usize {
    let data = include_str!("./input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|i| i.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let row_last_index = data.len() - 1;
    let column_last_index = data[0].len() - 1;
    let mut trail_map = HashMap::new();

    for (row_index, row) in data.iter().enumerate() {
        for (column_index, &item) in row.iter().enumerate() {
            if item == 0 {
                let start = (row_index, column_index);
                dfs_paths(
                    &data,
                    start,
                    start,
                    row_last_index,
                    column_last_index,
                    &mut trail_map,
                );
            }
        }
    }

    trail_map.values().fold(0, |acc, i| acc + i.len())
}

pub fn part_2() -> usize {
    0
}
