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

pub fn part_2() -> usize {
    0
}
