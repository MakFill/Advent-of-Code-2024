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

pub fn part_2() {}
