fn get_reports_number(with_dampener: bool) -> usize {
    let data = include_str!("./input.txt")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|c| c.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    data.iter().fold(0, |acc, vec| {
        let error_index = check_line(vec);

        if error_index == -1 {
            return acc + 1;
        }

        match with_dampener {
            true => {
                let index1 = error_index as usize;
                let slice1 = [&vec[..index1], &vec[index1 + 1..]].concat();
                if check_line(&slice1) != -1 {
                    let index2 = (error_index - 1) as usize;
                    let slice2 = [&vec[..index2], &vec[index2 + 1..]].concat();
                    if check_line(&slice2) != -1 {
                        if index2 == 1 {
                            return if check_line(&vec[1..]) == -1 {
                                acc + 1
                            } else {
                                acc
                            };
                        }
                        return acc;
                    }
                }
                acc + 1
            }
            false => acc,
        }
    })
}

fn check_line(vec: &[isize]) -> i8 {
    let mut is_asc = true;
    let asc_range = 1..=3;
    let desc_range = -3..=-1;

    for (index, item) in vec.iter().enumerate().skip(1) {
        let diff = item - vec[index - 1];
        if index == 1 {
            if desc_range.contains(&diff) {
                is_asc = false
            } else if !asc_range.contains(&diff) {
                return index as i8;
            }
        } else {
            match is_asc {
                true => {
                    if !asc_range.contains(&diff) {
                        return index as i8;
                    }
                }
                false => {
                    if !desc_range.contains(&diff) {
                        return index as i8;
                    }
                }
            }
        }
    }
    -1
}

pub fn part_1() -> usize {
    get_reports_number(false)
}

pub fn part_2() -> usize {
    get_reports_number(true)
}
