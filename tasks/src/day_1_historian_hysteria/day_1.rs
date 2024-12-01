use std::{
    cmp::{max, min},
    collections::HashMap,
};

pub fn part_1() -> usize {
    let mut list_1: Vec<usize> = vec![];
    let mut list_2: Vec<usize> = vec![];

    include_str!("./input.txt").lines().for_each(|line| {
        let mut is_first_list = true;
        line.split_whitespace().for_each(|i| {
            if let Ok(val) = i.parse::<usize>() {
                if is_first_list {
                    list_1.push(val);
                    is_first_list = false;
                } else {
                    list_2.push(val);
                }
            }
        })
    });

    list_1.sort_unstable();
    list_2.sort_unstable();

    list_1
        .iter()
        .zip(&list_2)
        .fold(0, |acc, (v1, v2)| acc + max(v1, v2) - min(v1, v2))
}

pub fn part_2() -> usize {
    let mut list_1: HashMap<usize, usize> = HashMap::new();
    let mut list_2: HashMap<usize, usize> = HashMap::new();

    include_str!("./input.txt").lines().for_each(|line| {
        let mut is_first_list = true;
        line.split_whitespace().for_each(|i| {
            if let Ok(val) = i.parse::<usize>() {
                if is_first_list {
                    *list_1.entry(val).or_insert(0) += 1;
                    is_first_list = false;
                } else {
                    *list_2.entry(val).or_insert(0) += 1;
                }
            }
        })
    });

    list_1.iter().fold(0, |acc, (&key, &val)| {
        if let Some(v) = list_2.get(&key) {
            acc + key * val * v
        } else {
            acc
        }
    })
}
