use std::collections::VecDeque;

const ROWS: usize = 71;
const COLS: usize = 71;
const FALLEN_BYTES: usize = 1024;

type Position = (usize, usize);

#[derive(Debug)]
enum Directions {
    Top,
    Bottom,
    Right,
    Left,
}

fn get_new_position(direction: Directions, pos: Position) -> Option<Position> {
    let row: isize;
    let col: isize;
    match direction {
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

    if row < 0 || row == ROWS as isize || col < 0 || col == COLS as isize {
        None
    } else {
        Some((row as usize, col as usize))
    }
}

fn get_minimum_steps(grid: &[Vec<char>]) -> usize {
    let start = (0_usize, 0_usize);
    let finish = (ROWS - 1, COLS - 1);
    let mut queue = VecDeque::new();
    let mut dist = vec![vec![usize::MAX; COLS]; ROWS];
    queue.push_back(start);

    dist[0][0] = 0;

    while let Some(pos) = queue.pop_front() {
        if pos == finish {
            break;
        }

        let new_positions = [
            Directions::Top,
            Directions::Bottom,
            Directions::Left,
            Directions::Right,
        ]
        .into_iter()
        .filter_map(|d| get_new_position(d, pos))
        .collect::<Vec<_>>();

        let new_cost = dist[pos.0][pos.1] + 1;

        for new_position in new_positions {
            let cost = dist[new_position.0][new_position.1];
            if grid[new_position.0][new_position.1] == '#' || new_cost >= cost {
                continue;
            }
            queue.push_back(new_position);
            dist[new_position.0][new_position.1] = new_cost;
        }
    }

    dist[finish.0][finish.1]
}

pub fn part_1() -> usize {
    let data = include_str!("./input.txt")
        .lines()
        .map(|line| {
            let pair = line.split(',').collect::<Vec<_>>();
            (
                pair[1].parse::<usize>().unwrap(),
                pair[0].parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut grid = vec![vec!['.'; COLS]; ROWS];

    data[0..FALLEN_BYTES].iter().for_each(|pos| {
        grid[pos.0][pos.1] = '#';
    });

    get_minimum_steps(&grid)
}

pub fn part_2() -> usize {
    0
}
