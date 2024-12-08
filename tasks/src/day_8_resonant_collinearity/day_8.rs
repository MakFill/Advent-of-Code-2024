use std::collections::{HashMap, HashSet};

fn get_res(with_harmonics: bool) -> usize {
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
        let is_add_position = (positions.len() > 1) && with_harmonics;
        for (index, position) in positions.iter().enumerate() {
            if is_add_position {
                unique_locations_set.insert(*position);
            }
            for another_position in positions.iter().skip(index + 1) {
                let row_diff = position.0.abs_diff(another_position.0) as isize;
                let column_diff = position.1.abs_diff(another_position.1) as isize;

                let (antinode_diff_1, antinode_diff_2) = if position.1 > another_position.1 {
                    ((-row_diff, column_diff), (row_diff, -column_diff))
                } else {
                    ((-row_diff, -column_diff), (row_diff, column_diff))
                };

                let mut antinode_position_1 = (
                    position.0 + antinode_diff_1.0,
                    position.1 + antinode_diff_1.1,
                );
                let mut antinode_position_2 = (
                    another_position.0 + antinode_diff_2.0,
                    another_position.1 + antinode_diff_2.1,
                );

                if with_harmonics {
                    let mut allowed_position_1 =
                        is_allowed_position(antinode_position_1, row_last_index, column_last_index);
                    let mut allowed_position_2 =
                        is_allowed_position(antinode_position_2, row_last_index, column_last_index);

                    while allowed_position_1 || allowed_position_2 {
                        if allowed_position_1 {
                            unique_locations_set.insert(antinode_position_1);
                            antinode_position_1 = (
                                antinode_position_1.0 + antinode_diff_1.0,
                                antinode_position_1.1 + antinode_diff_1.1,
                            );
                            allowed_position_1 = is_allowed_position(
                                antinode_position_1,
                                row_last_index,
                                column_last_index,
                            );
                        }
                        if allowed_position_2 {
                            unique_locations_set.insert(antinode_position_2);
                            antinode_position_2 = (
                                antinode_position_2.0 + antinode_diff_2.0,
                                antinode_position_2.1 + antinode_diff_2.1,
                            );
                            allowed_position_2 = is_allowed_position(
                                antinode_position_2,
                                row_last_index,
                                column_last_index,
                            );
                        }
                    }
                } else {
                    [antinode_position_1, antinode_position_2]
                        .into_iter()
                        .for_each(|pos| {
                            if is_allowed_position(pos, row_last_index, column_last_index) {
                                unique_locations_set.insert(pos);
                            }
                        });
                }
            }
        }
    }

    unique_locations_set.len()
}

fn is_allowed_position(
    position: (isize, isize),
    row_last_index: isize,
    column_last_index: isize,
) -> bool {
    position.0 >= 0
        && position.0 <= row_last_index
        && position.1 >= 0
        && position.1 <= column_last_index
}

pub fn part_1() -> usize {
    get_res(false)
}

pub fn part_2() -> usize {
    get_res(true)
}
