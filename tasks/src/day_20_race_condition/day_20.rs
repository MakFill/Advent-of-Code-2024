use std::collections::HashSet;

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

    fn get_cheat_position(
        &self,
        pos: Position,
        data: &[Vec<char>],
        cheat_pos_set: &HashSet<(Position, Position)>,
    ) -> Option<(Position, Position)> {
        let obstacle_pos = self.get_next_position(pos);
        if obstacle_pos.0 == 0
            || obstacle_pos.0 == data.len() - 1
            || obstacle_pos.1 == 0
            || obstacle_pos.1 == data[0].len() - 1
        {
            return None;
        }
        let cheat_end_pos = self.get_next_position(obstacle_pos);
        if data[cheat_end_pos.0][cheat_end_pos.1] == '.'
            && !cheat_pos_set.contains(&(cheat_end_pos, pos))
        {
            Some((pos, cheat_end_pos))
        } else {
            None
        }
    }
}

pub fn part_1() -> usize {
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
    let mut cheat_pos_set = HashSet::new();

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
                if let Some(cheat_pos) = d.get_cheat_position(start, &data, &cheat_pos_set) {
                    cheat_pos_set.insert(cheat_pos);
                }
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
                    if let Some(cheat_pos) = d.get_cheat_position(curr, &data, &cheat_pos_set) {
                        cheat_pos_set.insert(cheat_pos);
                    }
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

    cheat_pos_set.iter().fold(0, |acc, (start, end)| {
        let cheat_distance = dist_vec[end.0][end.1] - dist_vec[start.0][start.1] - 2;
        acc + if cheat_distance >= 100 { 1 } else { 0 }
    })
}

pub fn part_2() -> usize {
    0
}
