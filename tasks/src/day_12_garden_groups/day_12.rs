use std::collections::{HashMap, HashSet};

fn check_region_rec(
    position: (usize, usize),
    data: &[Vec<char>],
    visited_map_horizontal: &mut HashMap<isize, HashSet<isize>>,
    visited_map_vertical: &mut HashMap<isize, HashSet<isize>>,
    visited_set: &mut HashSet<(usize, usize)>,
    last_row_index: usize,
    last_column_index: usize,
) -> usize {
    let c = data[position.0][position.1];
    let mut res = 0;
    let directions = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    for d in directions {
        let next_row_position = position.0 as isize + d.0;
        let next_column_position = position.1 as isize + d.1;

        if next_row_position >= 0
            && next_row_position <= last_row_index as isize
            && next_column_position >= 0
            && next_column_position <= last_column_index as isize
        {
            if data[next_row_position as usize][next_column_position as usize] != c {
                res += 1;
                continue;
            }
            if !visited_set.insert((next_row_position as usize, next_column_position as usize)) {
                continue;
            }

            let next_position = (next_row_position as usize, next_column_position as usize);

            visited_map_horizontal
                .entry(next_row_position)
                .or_insert_with(HashSet::new)
                .insert(next_column_position);

            visited_map_vertical
                .entry(next_column_position)
                .or_insert_with(HashSet::new)
                .insert(next_row_position);

            res += check_region_rec(
                next_position,
                data,
                visited_map_horizontal,
                visited_map_vertical,
                visited_set,
                last_row_index,
                last_column_index,
            );
        } else {
            res += 1;
        }
    }
    res
}

fn get_bulk_sides(map: HashMap<isize, HashSet<isize>>) -> usize {
    let mut res = 0;

    for (row_index, columns_set) in map.iter() {
        let mut columns = columns_set.iter().collect::<Vec<_>>();
        columns.sort_unstable();

        let prev_row = map.get(&(row_index - 1));
        let next_row = map.get(&(row_index + 1));

        let neighbour_rows = [prev_row, next_row];

        for neighbour in neighbour_rows {
            for (index, item) in columns.iter().enumerate() {
                let prev = if index == 0 { -1 } else { *columns[index - 1] };
                let curr = *item;
                if let Some(neighbour_vec) = neighbour {
                    if index == 0 {
                        if !neighbour_vec.contains(curr) {
                            res += 1;
                        }
                    } else if !neighbour_vec.contains(curr)
                        && (prev != curr - 1 || neighbour_vec.contains(&(curr - 1)))
                    {
                        res += 1;
                    }
                } else if index == 0 || prev != curr - 1 {
                    res += 1;
                }
            }
        }
    }

    res
}

pub fn get_res() -> (usize, usize) {
    let data = include_str!("./input.txt")
        .lines()
        .map(|i| i.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let last_row_index = data.len() - 1;
    let last_column_index = data[0].len() - 1;
    let mut visited_set = HashSet::<(usize, usize)>::new();
    let mut res_1 = 0;
    let mut res_2 = 0;

    (0..=last_row_index).for_each(|row_index| {
        (0..=last_column_index).for_each(|column_index| {
            let position = (row_index, column_index);
            if visited_set.contains(&position) {
                return;
            }
            let mut region_map_horizontal =
                HashMap::from([(position.0 as isize, HashSet::from([position.1 as isize]))]);
            let mut region_map_vertical =
                HashMap::from([(position.1 as isize, HashSet::from([position.0 as isize]))]);
            let mut region_set = HashSet::<(usize, usize)>::from([position]);
            let temp_res = check_region_rec(
                position,
                &data,
                &mut region_map_horizontal,
                &mut region_map_vertical,
                &mut region_set,
                last_row_index,
                last_column_index,
            );

            res_1 += region_set.len() * temp_res;

            let horizontal_sides = get_bulk_sides(region_map_horizontal);
            let vertical_sides = get_bulk_sides(region_map_vertical);

            res_2 += (horizontal_sides + vertical_sides) * region_set.len();

            visited_set.extend(region_set);
        })
    });

    (res_1, res_2)
}
