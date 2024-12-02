pub fn part_1() -> usize {
    let data = include_str!("./input.txt")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|c| c.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let asc_range = 1..=3;
    let desc_range = -3..=-1;

    data.iter().fold(0, |acc, vec| {
        let mut is_asc = true;

        for (index, item) in vec.iter().enumerate().skip(1) {
            let diff = item - vec[index - 1];
            if index == 1 {
                if desc_range.contains(&diff) {
                    is_asc = false
                } else if !asc_range.contains(&diff) {
                    return acc;
                }
            } else {
                match is_asc {
                    true => {
                        if !asc_range.contains(&diff) {
                            return acc;
                        }
                    }
                    false => {
                        if !desc_range.contains(&diff) {
                            return acc;
                        }
                    }
                }
            }
        }

        acc + 1
    })
}

pub fn part_2() {}
