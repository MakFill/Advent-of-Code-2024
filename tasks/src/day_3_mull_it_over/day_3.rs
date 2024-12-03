use regex::Regex;

pub fn part_1() -> usize {
    let data = include_str!("./input.txt");

    let re1 = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let re2 = Regex::new(r"\d{1,3}").unwrap();

    re1.find_iter(data).fold(0, |acc, mat| {
        acc + re2
            .find_iter(mat.as_str())
            .fold(1, |acc, i| acc * i.as_str().parse::<usize>().unwrap())
    })
}

pub fn part_2() -> usize {
    let data = include_str!("./input.txt");

    let re1 = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don\'t\(\)").unwrap();
    let re2 = Regex::new(r"\d{1,3}").unwrap();

    let mut is_skip = false;

    re1.find_iter(data).fold(0, |acc, mat| {
        let mat_str = mat.as_str();

        match (is_skip, mat_str) {
            (_, "do()") => {
                is_skip = false;
                acc
            }
            (true, _) => acc,
            (false, "don't()") => {
                is_skip = true;
                acc
            }
            (false, _) => {
                acc + re2
                    .find_iter(mat_str)
                    .fold(1, |acc, i| acc * i.as_str().parse::<usize>().unwrap())
            }
        }
    })
}
