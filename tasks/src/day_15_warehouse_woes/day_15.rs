#[derive(Debug)]
enum Directions {
    Top,
    Bottom,
    Right,
    Left,
}

impl Directions {
    fn get_new_position(&self, pos: (usize, usize)) -> (usize, usize) {
        let row: isize;
        let col: isize;
        match self {
            Directions::Top => {
                row = pos.0 as isize - 1;
                col = pos.1 as isize;
            }
            Directions::Bottom => {
                row = pos.0 as isize + 1;
                col = pos.1 as isize;
            }
            Directions::Right => {
                row = pos.0 as isize;
                col = pos.1 as isize + 1;
            }
            Directions::Left => {
                row = pos.0 as isize;
                col = pos.1 as isize - 1;
            }
        }

        (row as usize, col as usize)
    }

    fn update_chart_with_shift(&self, chart: &mut [Vec<char>], position: (usize, usize)) -> bool {
        let row_len = chart.len();
        let col_len = chart[0].len();
        let init_row = position.0;
        let init_col = position.1;
        match self {
            Directions::Top | Directions::Left => {
                let mut box_count = 0;
                let (mut last_index, to) = match self {
                    Directions::Top => (init_row, init_row),
                    _ => (init_col, init_col),
                };

                for index in (0..=to).rev() {
                    let c = match self {
                        Directions::Top => chart[index][init_col],
                        _ => chart[init_row][index],
                    };
                    match c {
                        'O' => box_count += 1,
                        '#' => {
                            if to - box_count == index {
                                return false;
                            }
                            last_index = index + 1;
                            break;
                        }
                        _ => {
                            last_index = index;
                            break;
                        }
                    }
                }

                for index in last_index..=to {
                    let c = if box_count > 0 {
                        box_count -= 1;
                        'O'
                    } else {
                        '.'
                    };
                    match self {
                        Directions::Top => chart[index][init_col] = c,
                        _ => chart[init_row][index] = c,
                    };
                }
            }
            Directions::Bottom | Directions::Right => {
                let mut box_count = 0;
                let (mut last_index, from) = match self {
                    Directions::Bottom => (row_len - 1, init_row),
                    _ => (col_len - 1, init_col),
                };
                for index in from..=last_index {
                    let c = match self {
                        Directions::Bottom => chart[index][init_col],
                        _ => chart[init_row][index],
                    };
                    match c {
                        'O' => box_count += 1,
                        '#' => {
                            if from + box_count == index {
                                return false;
                            }
                            last_index = index - 1;
                            break;
                        }
                        _ => {
                            last_index = index;
                            break;
                        }
                    }
                }

                for index in (from..=last_index).rev() {
                    let c = if box_count > 0 {
                        box_count -= 1;
                        'O'
                    } else {
                        '.'
                    };
                    match self {
                        Directions::Bottom => chart[index][init_col] = c,
                        _ => chart[init_row][index] = c,
                    };
                }
            }
        };

        true
    }
}

pub fn part_1() -> usize {
    let mut curr_position = (0, 0);
    let binding = include_str!("./input.txt").lines().collect::<Vec<_>>();
    let mut parts = binding.splitn(2, |&a| a.is_empty());

    let mut chart = parts
        .next()
        .unwrap()
        .to_vec()
        .iter()
        .enumerate()
        .map(|(row_index, i)| {
            i.chars()
                .enumerate()
                .map(|(col_index, c)| {
                    if c == '@' {
                        curr_position.0 = row_index;
                        curr_position.1 = col_index;
                        return '.';
                    }
                    c
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let movements = parts
        .next()
        .unwrap()
        .to_vec()
        .iter()
        .flat_map(|i| i.lines().map(|line| line.chars().collect::<Vec<_>>()))
        .flatten()
        .collect::<Vec<_>>();

    for mov in movements.iter() {
        let direction = match mov {
            '^' => Directions::Top,
            '>' => Directions::Right,
            'v' => Directions::Bottom,
            _ => Directions::Left,
        };

        let new_position = direction.get_new_position(curr_position);
        let chart_item = chart[new_position.0][new_position.1];

        if chart_item == '#' {
            continue;
        }

        if chart_item == 'O' {
            if direction.update_chart_with_shift(&mut chart, new_position) {
                curr_position = new_position;
            }
        } else {
            curr_position = new_position;
        }
    }

    chart.iter().enumerate().fold(0, |acc, (row_index, row)| {
        acc + row.iter().enumerate().fold(0, |acc, (col_index, c)| {
            if *c == 'O' {
                return acc + row_index * 100 + col_index;
            }
            acc
        })
    })
}

pub fn part_2() -> usize {
    0
}
