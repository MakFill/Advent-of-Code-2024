use regex::Regex;

fn get_answer(is_part_2: bool) -> usize {
    let re = Regex::new(r"\d+").unwrap();
    let data = include_str!("./input.txt")
        .lines()
        .collect::<Vec<_>>()
        .split(|&a| a.is_empty())
        .map(|slice| {
            slice
                .iter()
                .map(|&i| {
                    let mut res: (usize, usize) = (0, 0);
                    for (index, item) in re.find_iter(i).enumerate() {
                        let val = item.as_str().parse::<usize>().unwrap();
                        if index == 0 {
                            res.0 = val;
                        } else {
                            res.1 = val;
                        }
                    }
                    res
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut res = 0;

    for i in data {
        let a = i[0];
        let b = i[1];
        let result = if is_part_2 {
            (i[2].0 + 10000000000000, i[2].1 + 10000000000000)
        } else {
            (i[2].0, i[2].1)
        };

        let temp1 = (result.1 * b.0).abs_diff(result.0 * b.1);
        let temp2 = (a.1 * b.0).abs_diff(a.0 * b.1);

        if temp1 % temp2 == 0 {
            let a_count = temp1 / temp2;
            let temp_b = result.0 - a_count * a.0;
            if temp_b % b.0 == 0 {
                let b_count = temp_b / b.0;
                res += a_count * 3 + b_count;
            }
        }
    }

    res
}

pub fn part_1() -> usize {
    get_answer(false)
}

pub fn part_2() -> usize {
    get_answer(true)
}
