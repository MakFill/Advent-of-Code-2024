use std::collections::{HashMap, HashSet};

pub fn part_1() -> usize {
    let mut antenas_map = HashMap::new();
    let mut row_last_index = 0;
    let mut column_last_index = 0;
    include_str!("./input.txt")
        .lines()
        .enumerate()
        .for_each(|(row_index, line)| {
            if row_last_index < row_index as isize {
                row_last_index = row_index as isize
            }
            if column_last_index == 0 {
                column_last_index = (line.len() - 1) as isize;
            }
            for (column_index, c) in line.chars().enumerate() {
                if c != '.' {
                    antenas_map
                        .entry(c)
                        .or_insert_with(Vec::new)
                        .push((row_index as isize, column_index as isize));
                }
            }
        });

    let mut unique_locations_set = HashSet::new();

    for positions in antenas_map.values() {
        for (index, position) in positions.iter().enumerate() {
            for another_position in positions.iter().skip(index + 1) {
                let row_diff = position.0.abs_diff(another_position.0) as isize;
                let column_diff = position.1.abs_diff(another_position.1) as isize;
                let position_row_minus = position.0 - row_diff;
                let another_position_row_plus = another_position.0 + row_diff;
                let position_column_plus = position.1 + column_diff;
                let position_column_minus = position.1 - column_diff;
                let another_position_column_plus = another_position.1 + column_diff;
                let another_position_column_minus = another_position.1 - column_diff;
                let (antinode_position_1, antinode_position_2) = if position.1 > another_position.1
                {
                    (
                        (position_row_minus, position_column_plus),
                        (another_position_row_plus, another_position_column_minus),
                    )
                } else {
                    (
                        (position_row_minus, position_column_minus),
                        (another_position_row_plus, another_position_column_plus),
                    )
                };

                [antinode_position_1, antinode_position_2]
                    .into_iter()
                    .for_each(|pos| {
                        if pos.0 >= 0
                            && pos.0 <= row_last_index
                            && pos.1 >= 0
                            && pos.1 <= column_last_index
                        {
                            unique_locations_set.insert(pos);
                        }
                    });
            }
        }
    }

    unique_locations_set.len()
}

pub fn part_2() -> usize {
    0
}
