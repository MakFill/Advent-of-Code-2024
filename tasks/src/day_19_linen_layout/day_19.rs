use std::collections::HashMap;

fn get_count(
    design: String,
    patterns: &[&str],
    patterns_map: &mut HashMap<String, usize>,
) -> usize {
    if let Some(&res) = patterns_map.get(&design) {
        return res;
    }

    if design.is_empty() {
        return 1;
    }

    let res_count = patterns
        .iter()
        .filter(|p| design.starts_with(*p))
        .fold(0, |acc, pattern| {
            let next_pattern_res =
                get_count(design[pattern.len()..].to_string(), patterns, patterns_map);
            acc + next_pattern_res
        });

    patterns_map.insert(design, res_count);
    res_count
}

pub fn get_res() -> (usize, usize) {
    let data = include_str!("./input.txt").lines().collect::<Vec<_>>();
    let mut parts = data.splitn(2, |&a| a.is_empty());
    let mut patterns = Vec::new();

    parts
        .next()
        .unwrap()
        .iter()
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|i| {
            i.split(", ").for_each(|p| {
                patterns.push(p);
            })
        });

    let designs = parts.next().unwrap().iter().collect::<Vec<_>>();
    let mut part_1 = 0;

    let part_2 = designs.iter().fold(0, |acc, d| {
        let res = get_count(d.to_string(), &patterns, &mut HashMap::new());
        if res > 0 {
            part_1 += 1;
        }
        acc + res
    });

    (part_1, part_2)
}
