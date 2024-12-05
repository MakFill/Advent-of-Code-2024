use std::collections::{HashMap, HashSet};

fn get_result(with_reorder: bool) -> usize {
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
                if with_reorder {
                    let mut reordered_vec = pages.clone();

                    bubble_sort(&mut reordered_vec, &before_map);

                    let len = reordered_vec.len();
                    let middle_element = if len == 0 { 0 } else { reordered_vec[len / 2] };
                    res += middle_element;
                }
                continue 'outer;
            }
            if let Some(page_set) = before_map.get(page) {
                entry_set.extend(page_set);
            }
        }

        if !with_reorder {
            let len = pages.len();
            let middle_element = if len == 0 { 0 } else { pages[len / 2] };
            res += middle_element;
        }
    }

    res
}

fn bubble_sort(arr: &mut [usize], before_map: &HashMap<usize, HashSet<usize>>) {
    let len = arr.len();
    for i in 0..len {
        let mut swapped = false;
        for k in 0..len - i - 1 {
            if let Some(set) = before_map.get(&arr[k]) {
                if set.contains(&arr[k + 1]) {
                    arr.swap(k, k + 1);
                    swapped = true;
                }
            }
        }
        if !swapped {
            break;
        }
    }
}

pub fn part_1() -> usize {
    get_result(false)
}

pub fn part_2() -> usize {
    get_result(true)
}
