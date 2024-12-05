use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
};

fn main() {
    let mut before: HashMap<i32, Vec<i32>> = HashMap::default();

    let mut file = File::open("data/input").expect("Open data");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Failed to write to buffer");
    let lines: Vec<_> = buf.lines().collect();

    let mut idx = 0;
    while !lines[idx].is_empty() {
        let num_queue: Vec<_> = lines[idx]
            .split("|")
            .filter_map(|num| num.parse::<i32>().ok())
            .collect();

        before
            .entry(num_queue[1])
            .or_insert_with(|| vec![])
            .push(num_queue[0]);

        idx += 1;
    }

    idx += 1;

    let mut sum_valid = 0;
    let mut sum_reordered = 0;

    for line in lines[idx..].iter() {
        let update: Vec<_> = line
            .split(",")
            .filter_map(|num| num.parse::<i32>().ok())
            .collect();

        if let Some(middle_entry) = is_valid_update(&before, &update) {
            sum_valid += middle_entry;
        } else {
            let reordered_update = reorder_update(&before, &update);
            let middle_entry = get_middle_page(&reordered_update);
            sum_reordered += middle_entry;
        }
    }

    println!("Sum of middle pages in valid updates: {sum_valid}");
    println!("Sum of middle pages in reordered updates: {sum_reordered}");
}

pub fn is_valid_update(rules: &HashMap<i32, Vec<i32>>, updates: &[i32]) -> Option<i32> {
    let mut seen: HashSet<i32> = HashSet::new();
    for val in updates {
        if let Some(all_rules_for_val) = rules.get(val) {
            for &before_needed in all_rules_for_val {
                if updates.contains(&before_needed) && !seen.contains(&before_needed) {
                    return None;
                }
            }
        }
        seen.insert(*val);
    }
    Some(get_middle_page(updates))
}

pub fn reorder_update(rules: &HashMap<i32, Vec<i32>>, updates: &[i32]) -> Vec<i32> {
    let mut remaining: HashSet<_> = updates.iter().collect();
    let mut ordered = Vec::new();

    while !remaining.is_empty() {
        let mut added = false;

        for &candidate in remaining.iter() {
            if let Some(all_rules_for_candidate) = rules.get(&candidate) {
                if all_rules_for_candidate.iter().all(|&before_needed| {
                    !remaining.contains(&before_needed) || ordered.contains(&before_needed)
                }) {
                    ordered.push(*candidate);
                    remaining.remove(&candidate);
                    added = true;
                    break;
                }
            } else {
                ordered.push(*candidate);
                remaining.remove(&candidate);
                added = true;
                break;
            }
        }

        if !added {
            panic!("Cycle detected in ordering rules!");
        }
    }

    ordered
}

pub fn get_middle_page(updates: &[i32]) -> i32 {
    let middle = updates.len() / 2;
    updates[middle]
}
