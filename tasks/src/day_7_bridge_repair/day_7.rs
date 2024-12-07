pub fn part_1() -> usize {
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
        if check_is_calibration_result(0, item.0, &item.1) {
            return acc + item.0;
        }
        acc
    })
}

fn check_is_calibration_result(acc: usize, res: usize, nums: &[usize]) -> bool {
    let acc1 = acc + nums[0];
    let acc2 = acc * nums[0];
    if acc1 == res || acc2 == res {
        return true;
    } else if nums.len() == 1 {
        return false;
    }
    let acc_res1 = if acc1 > res {
        false
    } else {
        check_is_calibration_result(acc1, res, &nums[1..])
    };
    let acc_res2 = if acc2 > res {
        false
    } else {
        check_is_calibration_result(acc2, res, &nums[1..])
    };

    acc_res1 || acc_res2
}

pub fn part_2() -> usize {
    0
}
