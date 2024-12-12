use std::collections::HashSet;

fn check_region_rec(
    position: (usize, usize),
    data: &[Vec<char>],
    visited_set: &mut HashSet<(usize, usize)>,
    last_row_index: usize,
    last_column_index: usize,
) -> usize {
    let c = data[position.0][position.1];
    let mut res = 0;

    let directions = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    for d in directions {
        match (
            position.0 == 0 && d.0 == -1,
            position.0 == last_row_index && d.0 == 1,
            position.1 == 0 && d.1 == -1,
            position.1 == last_column_index && d.1 == 1,
        ) {
            (false, false, false, false) => {
                let next_position = (
                    (position.0 as isize + d.0) as usize,
                    (position.1 as isize + d.1) as usize,
                );
                if data[next_position.0][next_position.1] != c {
                    res += 1;
                } else if visited_set.insert(next_position) {
                    res += check_region_rec(
                        next_position,
                        data,
                        visited_set,
                        last_row_index,
                        last_column_index,
                    );
                }
            }
            _ => {
                res += 1;
            }
        }
    }

    res
}

pub fn part_1() -> usize {
    let data = include_str!("./input.txt")
        .lines()
        .map(|i| i.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let last_row_index = data.len() - 1;
    let last_column_index = data[0].len() - 1;
    let mut visited_set = HashSet::<(usize, usize)>::new();
    let mut res = 0;

    (0..=last_row_index).for_each(|row_index| {
        (0..=last_column_index).for_each(|column_index| {
            let position = (row_index, column_index);
            if visited_set.contains(&position) {
                return;
            }
            let mut region_set = HashSet::from([position]);
            let fences_amount = check_region_rec(
                position,
                &data,
                &mut region_set,
                last_row_index,
                last_column_index,
            );
            res += fences_amount * region_set.len();
            visited_set.extend(region_set);
        })
    });

    res
}

pub fn part_2() -> usize {
    0
}
