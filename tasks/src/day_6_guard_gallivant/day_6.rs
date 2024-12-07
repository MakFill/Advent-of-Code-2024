use std::collections::HashSet;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
enum Directions {
    Top,
    Right,
    Bottom,
    Left,
}

impl Directions {
    fn get_next_position(&self, current_position: (usize, usize)) -> ((usize, usize), Directions) {
        match self {
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
        }
    }
}

const MARKERS: [char; 4] = ['^', '>', 'v', '<'];

fn get_result(with_obstacle: bool) -> usize {
    let mut start_position = (0, 0);
    let mut current_position = (0, 0);
    let mut direction = Directions::Top;
    let mut start_direction = Directions::Top;
    let mut positions_set = HashSet::<(usize, usize)>::new();
    let data = include_str!("./input.txt")
        .lines()
        .enumerate()
        .map(|(row_index, line)| {
            let mut line_vec = vec![];
            for (column_index, c) in line.chars().enumerate() {
                if MARKERS.contains(&c) {
                    current_position = (row_index, column_index);
                    start_position = current_position;
                    if !with_obstacle {
                        positions_set.insert(current_position);
                    }
                    line_vec.push('.');
                    direction = match c {
                        '^' => Directions::Top,
                        '>' => Directions::Right,
                        'v' => Directions::Bottom,
                        _ => Directions::Left,
                    };
                    start_direction = direction.clone();
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
        let (next_position, new_direction) = direction.get_next_position(current_position);
        if data[next_position.0][next_position.1] == '#' {
            direction = new_direction;
        } else {
            match with_obstacle {
                true => {
                    if !positions_set.contains(&next_position)
                        && check_is_cycle(
                            start_position,
                            next_position,
                            start_direction.clone(),
                            row_last_index,
                            column_last_index,
                            &data,
                        )
                    {
                        positions_set.insert(next_position);
                    }
                }
                false => {
                    positions_set.insert(next_position);
                }
            };
            current_position = next_position;
        }
    }

    positions_set.len()
}

fn check_is_cycle(
    mut current_position: (usize, usize),
    obstacle_position: (usize, usize),
    mut direction: Directions,
    row_last_index: usize,
    column_last_index: usize,
    data: &[Vec<char>],
) -> bool {
    let mut positions_set = HashSet::new();
    while current_position.0 != 0
        && current_position.0 != row_last_index
        && current_position.1 != 0
        && current_position.1 != column_last_index
    {
        let (next_position, new_direction) = direction.get_next_position(current_position);

        if data[next_position.0][next_position.1] == '#' || next_position == obstacle_position {
            let entry = (current_position.0, current_position.1, direction.clone());
            if !positions_set.insert(entry) {
                return true;
            }

            direction = new_direction;
        } else {
            current_position = next_position;
        }
    }
    false
}

pub fn part_1() -> usize {
    get_result(false)
}

pub fn part_2() -> usize {
    get_result(true)
}
