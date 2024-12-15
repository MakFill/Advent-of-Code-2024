use std::collections::{HashSet, VecDeque};

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

    fn get_box_half_column(&self, position: (usize, usize), c: char) -> usize {
        if c == '[' {
            position.1 + 1
        } else {
            position.1 - 1
        }
    }

    fn get_box_next_positions(&self, position: (usize, usize), c: char) -> Vec<(usize, usize)> {
        let pos_1 = match self {
            Directions::Top => (position.0 - 1, position.1),
            Directions::Bottom => (position.0 + 1, position.1),
            _ => position,
        };

        let pos_2 = (pos_1.0, self.get_box_half_column(position, c));

        vec![pos_1, pos_2]
    }

    fn chart_incremental_update(
        &self,
        chart: &mut [Vec<char>],
        position: (usize, usize),
        positions_to_update: &mut VecDeque<(usize, usize)>,
        positions_set: &mut HashSet<(usize, usize)>,
    ) -> bool {
        let c = chart[position.0][position.1];
        match c {
            '[' | ']' => match self {
                Directions::Top | Directions::Bottom => {
                    let second_half = (position.0, self.get_box_half_column(position, c));

                    if positions_set.insert(position) && positions_set.insert(second_half) {
                        let second_c = if c == '[' { ']' } else { '[' };
                        let next_positions = [
                            self.get_box_next_positions(position, c),
                            self.get_box_next_positions(second_half, second_c),
                        ];

                        for &pos in next_positions.iter().flatten() {
                            if !self.chart_incremental_update(
                                chart,
                                pos,
                                positions_to_update,
                                positions_set,
                            ) {
                                return false;
                            }
                        }

                        positions_to_update.push_back(position);
                        positions_to_update.push_back(second_half);
                    }
                }
                _ => {
                    let next_pos = self.get_new_position(position);
                    if self.chart_incremental_update(
                        chart,
                        next_pos,
                        positions_to_update,
                        positions_set,
                    ) {
                        positions_to_update.push_back(position);
                    } else {
                        return false;
                    }
                }
            },
            '#' => {
                return false;
            }
            _ => {
                return true;
            }
        }
        true
    }

    fn update_chart_with_shift_2(&self, chart: &mut [Vec<char>], position: (usize, usize)) -> bool {
        let mut boxes_queue = VecDeque::<(usize, usize)>::new();
        let mut positions_set = HashSet::new();

        if !self.chart_incremental_update(chart, position, &mut boxes_queue, &mut positions_set) {
            return false;
        }

        while let Some(item) = boxes_queue.pop_front() {
            let new_pos = self.get_new_position(item);
            let temp = chart[item.0][item.1];
            chart[item.0][item.1] = chart[new_pos.0][new_pos.1];
            chart[new_pos.0][new_pos.1] = temp
        }

        true
    }
}

fn get_res(is_part_2: bool) -> usize {
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
                .flat_map(|(col_index, c)| {
                    if is_part_2 {
                        match c {
                            '@' => {
                                curr_position.0 = row_index;
                                curr_position.1 = col_index * 2;
                                vec!['.', '.']
                            }
                            'O' => vec!['[', ']'],
                            v => vec![v, v],
                        }
                    } else {
                        if c == '@' {
                            curr_position.0 = row_index;
                            curr_position.1 = col_index;
                            return vec!['.'];
                        }
                        vec![c]
                    }
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

        if is_part_2 && chart_item == '[' || chart_item == ']' {
            if direction.update_chart_with_shift_2(&mut chart, new_position) {
                curr_position = new_position;
            }
        } else if chart_item == 'O' {
            if direction.update_chart_with_shift(&mut chart, new_position) {
                curr_position = new_position;
            }
        } else {
            curr_position = new_position;
        }
    }

    chart.iter().enumerate().fold(0, |acc, (row_index, row)| {
        acc + row.iter().enumerate().fold(0, |acc, (col_index, c)| {
            if (is_part_2 && *c == '[') || *c == 'O' {
                return acc + row_index * 100 + col_index;
            }
            acc
        })
    })
}

pub fn part_1() -> usize {
    get_res(false)
}

pub fn part_2() -> usize {
    get_res(true)
}
