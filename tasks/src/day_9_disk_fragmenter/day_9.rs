use std::collections::HashSet;

pub fn part_1() -> usize {
    let mut curr_id = 0_usize;
    let data = include_str!("./input.txt")
        .chars()
        .enumerate()
        .fold(vec![], |mut acc, (i, c)| {
            let size = c.to_digit(10).unwrap() as usize;
            let id = if i % 2 == 0 {
                let res = Some(curr_id);
                curr_id += 1;
                res
            } else {
                None
            };

            (0..size).for_each(|_| {
                acc.push(id);
            });

            acc
        });

    let mut last_index = data.len() - 1;
    let mut res_vec = vec![];

    for (index, item) in data.iter().enumerate() {
        match item {
            Some(val) => {
                res_vec.push(*val);
            }
            None => {
                let mut last_id = data[last_index];
                while last_id.is_none() {
                    last_index -= 1;
                    last_id = data[last_index];
                }
                res_vec.push(last_id.unwrap());
                last_index -= 1;
            }
        }
        if index == last_index {
            break;
        }
    }

    res_vec
        .into_iter()
        .enumerate()
        .fold(0, |acc, (index, item)| acc + (index * item))
}

fn handle_free_space_rec(
    data: &[(Option<usize>, usize)],
    res_vec: &mut Vec<(Option<usize>, usize)>,
    moved_indexes: &mut HashSet<usize>,
    diff: usize,
    data_last_index: usize,
    index: usize,
) {
    let mut last_index = data_last_index;
    while index != last_index {
        let last_block = data[last_index];
        if last_block.0.is_none() || moved_indexes.contains(&last_index) {
            last_index -= 1;
            continue;
        }
        if last_block.1 == diff {
            res_vec.push(last_block);
            moved_indexes.insert(last_index);
            return;
        }
        if last_block.1 < diff {
            res_vec.push(last_block);
            moved_indexes.insert(last_index);
            let diff = diff - last_block.1;
            return handle_free_space_rec(
                data,
                res_vec,
                moved_indexes,
                diff,
                data_last_index,
                index,
            );
        }
        last_index -= 1;
    }
    res_vec.push((None, diff));
}

pub fn part_2() -> usize {
    let mut curr_id = 0_usize;
    let data = include_str!("./input.txt")
        .chars()
        .enumerate()
        .fold(vec![], |mut acc, (i, c)| {
            let size = c.to_digit(10).unwrap() as usize;
            if size == 0 {
                return acc;
            }
            let id = if i % 2 == 0 {
                let res = Some(curr_id);
                curr_id += 1;
                res
            } else {
                None
            };
            acc.push((id, size));
            acc
        });

    let data_last_index = data.len() - 1;
    let mut res_vec = vec![];
    let mut moved_indexes = HashSet::new();

    for (index, item) in data.iter().enumerate() {
        match item.0 {
            Some(_) => {
                if !moved_indexes.contains(&index) {
                    res_vec.push(*item);
                } else {
                    res_vec.push((None, item.1));
                }
            }
            None => {
                handle_free_space_rec(
                    &data,
                    &mut res_vec,
                    &mut moved_indexes,
                    item.1,
                    data_last_index,
                    index,
                );
            }
        }
    }

    res_vec
        .into_iter()
        .flat_map(|i| {
            let mut item_vec = vec![];
            (0..i.1).for_each(|_| item_vec.push(i.0));
            item_vec
        })
        .enumerate()
        .fold(0, |acc, (index, item)| {
            if let Some(val) = item {
                return acc + (index * val);
            }
            acc
        })
}
