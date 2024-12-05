use std::collections::{HashMap, HashSet};

pub fn part_1() -> usize {
    let mut pages_vec = vec![];
    let mut before_map = HashMap::new();

    let mut is_fill_ordering_rules = true;

    for line in include_str!("./input.txt").lines() {
        if line.is_empty() {
            is_fill_ordering_rules = false;
            continue;
        }
        match is_fill_ordering_rules {
            true => line.split_once('|').iter().for_each(|(a, b)| {
                let before_page = a.parse::<usize>().unwrap();
                let after_page = b.parse::<usize>().unwrap();
                before_map
                    .entry(before_page)
                    .or_insert_with(HashSet::new)
                    .insert(after_page);
            }),
            false => pages_vec.push(
                line.split(',')
                    .map(|i| i.parse::<usize>().unwrap())
                    .rev()
                    .collect::<Vec<_>>(),
            ),
        }
    }

    let mut res = 0;

    'outer: for pages in pages_vec.iter() {
        let mut entry_set: HashSet<usize> = HashSet::new();
        for page in pages.iter() {
            if entry_set.contains(page) {
                continue 'outer;
            }
            if let Some(page_set) = before_map.get(page) {
                entry_set.extend(page_set);
            }
        }

        let len = pages.len();
        let middle_element = if len == 0 { 0 } else { pages[len / 2] };
        res += middle_element;
    }

    res
}

pub fn part_2() -> usize {
    0
}
