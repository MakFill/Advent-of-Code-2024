use std::collections::BinaryHeap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Directions {
    Top,
    Bottom,
    Right,
    Left,
}

impl Directions {
    fn get_next_position(&self, pos: (usize, usize)) -> (usize, usize) {
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

    fn get_possible_directions(&self) -> [Directions; 3] {
        match self {
            Directions::Top => [Directions::Top, Directions::Left, Directions::Right],
            Directions::Bottom => [Directions::Bottom, Directions::Left, Directions::Right],
            Directions::Right => [Directions::Top, Directions::Bottom, Directions::Right],
            Directions::Left => [Directions::Top, Directions::Bottom, Directions::Left],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State {
    cost: usize,
    position: (usize, usize),
    direction: Directions,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(
    data: &[Vec<char>],
    start: (usize, usize),
    finish: (usize, usize),
    direction: Directions,
) -> usize {
    let row_length = data.len();
    let col_length = data[0].len();
    let mut dist = vec![vec![usize::MAX; col_length]; row_length];
    let mut heap = BinaryHeap::new();

    dist[start.0][start.1] = 0;

    heap.push(State {
        cost: 0,
        position: start,
        direction,
    });

    while let Some(State {
        cost,
        position,
        direction,
    }) = heap.pop()
    {
        if position == finish {
            return cost;
        }

        if cost > dist[position.0][position.1] {
            continue;
        }

        let neighbours = direction
            .get_possible_directions()
            .map(|d| (d.get_next_position(position), d));

        let allowed_next_positions = neighbours
            .iter()
            .filter(|(p, _)| data[p.0][p.1] != '#')
            .collect::<Vec<_>>();

        for (pos, dir) in allowed_next_positions {
            let next = State {
                position: *pos,
                direction: *dir,
                cost: cost + if *dir == direction { 1 } else { 1001 },
            };

            if next.cost < dist[pos.0][pos.1] {
                heap.push(next);
                dist[pos.0][pos.1] = next.cost;
            }
        }
    }

    0
}

pub fn part_1() -> usize {
    let mut start = (0, 0);
    let mut finish = (0, 0);
    let init_direction = Directions::Right;
    let data = include_str!("./input.txt")
        .lines()
        .enumerate()
        .map(|(row_i, r)| {
            r.chars()
                .enumerate()
                .map(|(col_i, c)| {
                    if c == 'S' {
                        start = (row_i, col_i);
                        return '.';
                    } else if c == 'E' {
                        finish = (row_i, col_i);
                        return '.';
                    }
                    c
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    dijkstra(&data, start, finish, init_direction)
}

pub fn part_2() -> usize {
    0
}
