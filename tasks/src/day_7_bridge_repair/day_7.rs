fn get_result(with_concatenation: bool) -> usize {
    let data = include_str!("./input.txt")
        .lines()
        .map(|line| {
            let (res, nums) = line.split_once(':').unwrap();

            let nums_vec = nums[1..]
                .split_ascii_whitespace()
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            let r = res.parse::<usize>().unwrap();

            (r, nums_vec)
        })
        .collect::<Vec<_>>();

    data.iter().fold(0, |acc, item| {
        if check_is_calibration_result(0, item.0, &item.1, with_concatenation) {
            return acc + item.0;
        }
        acc
    })
}

fn check_is_calibration_result(
    acc: usize,
    res: usize,
    nums: &[usize],
    with_concatenation: bool,
) -> bool {
    if acc > res {
        false
    } else if nums.is_empty() {
        acc == res
    } else {
        if let Some((val, vals)) = nums.split_first() {
            let concat_check = if with_concatenation {
                let digits = (nums[0] as f64).log10().floor() as usize + 1;
                let concatenation = acc * 10_usize.pow(digits as u32) + nums[0];
                check_is_calibration_result(concatenation, res, vals, with_concatenation)
            } else {
                false
            };

            return check_is_calibration_result(acc + val, res, vals, with_concatenation)
                || check_is_calibration_result(acc * val, res, vals, with_concatenation)
                || concat_check;
        }
        false
    }
}

pub fn part_1() -> usize {
    get_result(false)
}

pub fn part_2() -> usize {
    get_result(true)
}
