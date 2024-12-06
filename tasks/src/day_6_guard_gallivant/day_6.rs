use std::collections::HashSet;

#[derive(Debug)]
enum Directions {
    Top,
    Right,
    Bottom,
    Left,
}

const MARKERS: [char; 4] = ['^', '>', 'v', '<'];

pub fn part_1() -> usize {
    let mut current_position = (0, 0);
    let mut direction = Directions::Top;
    let mut distinct_positions_set = HashSet::<(usize, usize)>::new();
    let data = include_str!("./input.txt")
        .lines()
        .enumerate()
        .map(|(row_index, line)| {
            let mut line_vec = vec![];
            for (column_index, c) in line.chars().enumerate() {
                if MARKERS.contains(&c) {
                    current_position = (row_index, column_index);
                    distinct_positions_set.insert(current_position);
                    line_vec.push('.');
                    direction = match c {
                        '^' => Directions::Top,
                        '>' => Directions::Right,
                        'v' => Directions::Bottom,
                        _ => Directions::Left,
                    }
                } else {
                    line_vec.push(c);
                }
            }
            line_vec
        })
        .collect::<Vec<_>>();

    let row_last_index = data.len() - 1;
    let column_last_index = data[0].len() - 1;

    while current_position.0 != 0
        && current_position.0 != row_last_index
        && current_position.1 != 0
        && current_position.1 != column_last_index
    {
        let (next_position, new_direction) = match direction {
            Directions::Top => {
                let next_position = (current_position.0 - 1, current_position.1);
                (next_position, Directions::Right)
            }
            Directions::Right => {
                let next_position = (current_position.0, current_position.1 + 1);
                (next_position, Directions::Bottom)
            }
            Directions::Bottom => {
                let next_position = (current_position.0 + 1, current_position.1);
                (next_position, Directions::Left)
            }
            Directions::Left => {
                let next_position = (current_position.0, current_position.1 - 1);
                (next_position, Directions::Top)
            }
        };
        if data[next_position.0][next_position.1] == '#' {
            direction = new_direction;
        } else {
            current_position = next_position;
            distinct_positions_set.insert(current_position);
        }
    }

    distinct_positions_set.len()
}

pub fn part_2() -> usize {
    0
}
