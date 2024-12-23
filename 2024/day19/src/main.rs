use std::{collections::HashMap, fs::File, io::Read};

fn main() {
    let mut file = File::open("data/input").expect("Open data");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Failed to write to buffer");

    let patterns: Vec<_> = buf.lines().filter(|line| !line.is_empty()).collect();

    let (has, possible) = (patterns[0], patterns[1..].to_vec());
    let has = has.split(", ").collect::<Vec<_>>();

    let mut initial = has
        .clone()
        .into_iter()
        .map(|pat| (pat.to_string(), true))
        .collect::<HashMap<String, bool>>();

    let can_create = possible
        .iter()
        .filter(|pattern| pattern_can_be_created(&has, pattern, &mut initial))
        .count();

    println!("Possible patterns we can make: {can_create}");

    let combinations: usize = patterns
        .iter()
        .map(|pattern| count_unique_combinations(&has, pattern, &mut HashMap::new()))
        .sum();
    println!("{combinations} possible combinations");
}

fn pattern_can_be_created(
    options: &[&str],
    pattern: &str,
    cache: &mut HashMap<String, bool>,
) -> bool {
    if cache.contains_key(pattern) {
        cache[pattern]
    } else {
        for option in options {
            if pattern.starts_with(option) {
                let possible = pattern_can_be_created(options, &pattern[option.len()..], cache);
                if possible {
                    cache.insert(pattern.to_string(), true);
                    return true;
                }
            }
        }
        cache.insert(pattern.to_string(), false);
        false
    }
}

fn count_unique_combinations(
    options: &[&str],
    pattern: &str,
    cache: &mut HashMap<String, usize>,
) -> usize {
    if pattern.is_empty() {
        1
    } else if let Some(&cached) = cache.get(pattern) {
        cached
    } else {
        let mut count = 0;
        for option in options {
            if pattern.starts_with(option) {
                count += count_unique_combinations(options, &pattern[option.len()..], cache);
            }
        }

        cache.insert(pattern.to_string(), count);
        count
    }
}
