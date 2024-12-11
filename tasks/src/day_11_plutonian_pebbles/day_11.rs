use std::collections::HashMap;

fn get_result(blinks_amount: usize) -> usize {
    let data = include_str!("./input.txt")
        .split_ascii_whitespace()
        .map(|i| i.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut map = HashMap::<(usize, usize), usize>::new();
    let mut res = 0;

    data.iter()
        .for_each(|i| res += handle_blink_rec(*i, 0, blinks_amount, &mut map));

    res
}

fn handle_blink_rec(
    val: usize,
    blink_step: usize,
    blinks_amount: usize,
    map: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if blink_step == blinks_amount {
        return 1;
    }
    if let Some(res) = map.get(&(val, blink_step)) {
        return *res;
    }
    let mut res = 0;

    if val == 0 {
        let result = handle_blink_rec(1, blink_step + 1, blinks_amount, map);
        map.insert((val, blink_step), result);
        res += result;
    } else {
        let digits_count = count_digits(val);
        if digits_count % 2 == 0 {
            let temp_val = 10_usize.pow(digits_count / 2);
            let first_val = val / temp_val;
            let second_val = val % temp_val;

            let result_first = handle_blink_rec(first_val, blink_step + 1, blinks_amount, map);
            map.insert((first_val, blink_step + 1), result_first);
            res += result_first;

            let result_second = handle_blink_rec(second_val, blink_step + 1, blinks_amount, map);
            map.insert((second_val, blink_step + 1), result_second);
            res += result_second;
        } else {
            let next_val = val * 2024;
            let result = handle_blink_rec(next_val, blink_step + 1, blinks_amount, map);
            res += result;

            map.insert((val, blink_step), result);
        }
    }

    res
}

fn count_digits(num: usize) -> u32 {
    (num as f64).log10().floor() as u32 + 1
}

pub fn part_1() -> usize {
    get_result(25)
}

pub fn part_2() -> usize {
    get_result(75)
}
