type Position = (usize, usize);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Directions {
    Top,
    Bottom,
    Right,
    Left,
}

impl Directions {
    fn get_next_position(&self, pos: Position) -> Position {
        match self {
            Directions::Top => (pos.0 - 1, pos.1),
            Directions::Bottom => (pos.0 + 1, pos.1),
            Directions::Right => (pos.0, pos.1 + 1),
            Directions::Left => (pos.0, pos.1 - 1),
        }
    }

    fn get_possible_directions(&self) -> Vec<Directions> {
        match self {
            Directions::Top => vec![Directions::Top, Directions::Left, Directions::Right],
            Directions::Bottom => vec![Directions::Bottom, Directions::Left, Directions::Right],
            Directions::Right => vec![Directions::Top, Directions::Bottom, Directions::Right],
            Directions::Left => vec![Directions::Top, Directions::Bottom, Directions::Left],
        }
    }
}

fn get_cheats_for_one_position(
    position: Position,
    data: &[Vec<char>],
    dist_vec: &[Vec<i32>],
    picoseconds_to_save: i32,
    cheat_max_length: usize,
) -> usize {
    let mut res = 0;
    let start_cost = dist_vec[position.0][position.1];
    let row_first_index = if position.0 > cheat_max_length {
        position.0 - cheat_max_length
    } else {
        1
    };
    let row_last_index = if position.0 + cheat_max_length < data.len() - 2 {
        position.0 + cheat_max_length
    } else {
        data.len() - 2
    };
    let col_first_index = if position.1 > cheat_max_length {
        position.1 - cheat_max_length
    } else {
        1
    };
    let col_last_index = if position.1 + cheat_max_length < data[0].len() - 2 {
        position.1 + cheat_max_length
    } else {
        data[0].len() - 2
    };

    for row_index in row_first_index..=row_last_index {
        for col_index in col_first_index..=col_last_index {
            let c = data[row_index][col_index];
            if c == '.' {
                let end_cost = dist_vec[row_index][col_index];
                if start_cost >= end_cost {
                    continue;
                }
                let length =
                    (position.0.abs_diff(row_index) + position.1.abs_diff(col_index)) as i32;

                if length <= cheat_max_length as i32
                    && end_cost - start_cost - length >= picoseconds_to_save
                {
                    res += 1;
                }
            }
        }
    }

    res
}

fn get_result(picoseconds_to_save: i32, cheat_max_length: usize) -> usize {
    let mut start = (0, 0);
    let mut finish = (0, 0);
    let data = include_str!("./input.txt")
        .lines()
        .enumerate()
        .map(|(row_index, row)| {
            row.chars()
                .enumerate()
                .map(|(col_index, c)| match c {
                    'S' => {
                        start = (row_index, col_index);
                        '.'
                    }
                    'E' => {
                        finish = (row_index, col_index);
                        '.'
                    }
                    _ => c,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let rows = data.len();
    let cols = data[0].len();
    let mut dist_vec = vec![vec![0; cols]; rows];

    let mut curr = start;
    let all_directions = [
        Directions::Top,
        Directions::Bottom,
        Directions::Left,
        Directions::Right,
    ];
    let mut direction = *all_directions
        .iter()
        .filter(|d| {
            let next_pos = d.get_next_position(curr);
            if data[next_pos.0][next_pos.1] == '#' {
                return false;
            }

            true
        })
        .collect::<Vec<_>>()[0];
    let mut distance = 0;

    while curr != finish {
        let possible_directions = direction.get_possible_directions();
        let (next_pos, next_dir) = possible_directions
            .iter()
            .filter_map(|d| {
                let next_pos = d.get_next_position(curr);
                if data[next_pos.0][next_pos.1] == '#' {
                    None
                } else {
                    Some((next_pos, d))
                }
            })
            .collect::<Vec<_>>()[0];

        curr = next_pos;
        direction = *next_dir;
        distance += 1;
        dist_vec[curr.0][curr.1] = distance;
    }

    let mut res = 0;
    for row_index in 1..rows - 1 {
        for col_index in 1..cols - 1 {
            let c = data[row_index][col_index];
            if c == '.' {
                res += get_cheats_for_one_position(
                    (row_index, col_index),
                    &data,
                    &dist_vec,
                    picoseconds_to_save,
                    cheat_max_length,
                );
            }
        }
    }

    res
}

pub fn part_1() -> usize {
    get_result(100, 2)
}

pub fn part_2() -> usize {
    get_result(100, 20)
}
