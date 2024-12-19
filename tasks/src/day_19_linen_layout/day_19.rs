use std::collections::HashSet;

fn check_is_design_possible(design: &str, patterns: &HashSet<&str>) -> bool {
    for &pattern in patterns {
        if design.starts_with(pattern) {
            let pattern_len = pattern.len();
            if design.len() == pattern_len
                || check_is_design_possible(&design[pattern_len..], patterns)
            {
                return true;
            }
        }
    }

    false
}

fn check_is_pattern_redundant_rec(pattern: &str, patterns: &HashSet<&str>) -> bool {
    let len = pattern.len();
    for size in 1..len {
        let pair = pattern.split_at(size);
        match (patterns.contains(pair.0), patterns.contains(pair.1)) {
            (true, true) => {
                return true;
            }
            (true, false) => {
                if pair.1.len() > 1 && check_is_pattern_redundant_rec(pair.1, patterns) {
                    return true;
                }
            }
            (false, true) => {
                if pair.0.len() > 1 && check_is_pattern_redundant_rec(pair.0, patterns) {
                    return true;
                }
            }
            (false, false) => {
                if pair.0.len() > 1
                    && pair.1.len() > 1
                    && check_is_pattern_redundant_rec(pair.0, patterns)
                    && check_is_pattern_redundant_rec(pair.1, patterns)
                {
                    return true;
                }
            }
        }
    }
    false
}

pub fn part_1() -> usize {
    let data = include_str!("./input.txt").lines().collect::<Vec<_>>();
    let mut parts = data.splitn(2, |&a| a.is_empty());
    let mut patterns = HashSet::new();

    parts
        .next()
        .unwrap()
        .iter()
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|i| {
            i.split(", ").for_each(|p| {
                patterns.insert(p);
            })
        });

    let designs = parts.next().unwrap().iter().collect::<Vec<_>>();
    let patterns_to_iter = patterns.clone();

    for pattern in patterns_to_iter.clone() {
        if check_is_pattern_redundant_rec(pattern, &patterns_to_iter) {
            patterns.remove(pattern);
        }
    }

    designs
        .into_iter()
        .filter(|&d| check_is_design_possible(d, &patterns))
        .count()
}

pub fn part_2() -> usize {
    0
}
