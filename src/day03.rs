use std::collections::HashSet;
use itertools::Itertools;
use crate::get_file_lines;

pub fn day03() {
    let duplicate_priority = duplicates(get_file_lines("data/03_data.txt"));
    println!("priority: {duplicate_priority}");
    let badge_priority = badges(get_file_lines("data/03_data.txt"));
    println!("badges: {badge_priority}");
}

fn priority(item: u8) -> u32 {
    match item {
        b'a'..=b'z' => (item - b'a' + 1) as u32,
        b'A'..=b'Z' => (item - b'A' + 27) as u32,
        _ => panic!("unknown item '{item}'")
    }
}

/// Finds duplicate items found in the first half and the second half of each line,
/// and sums their priority values.
fn duplicates<E>(lines: E) -> u32 where E: Iterator<Item = String> {
    lines
        .map(|line| line.into_bytes())
        .map(|mut items| {
            let right = items.split_off(items.len() / 2);
            let left_set = HashSet::<u8>::from_iter(items);
            let right_set = HashSet::<u8>::from_iter(right);
            right_set.intersection(&left_set)
                .map(|&item| priority(item))
                .sum::<u32>()
        })
        .sum()
}

/// Finds duplicate items found in 3 consecutive lines, and sums their priority values.
fn badges<E>(lines: E) -> u32 where E: Iterator<Item = String> {
    lines
        .map(|line| line.into_bytes())
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let mut items = chunk.map(|sack| HashSet::<u8>::from_iter(sack)).collect::<Vec<_>>();
            let mut badges = items.pop().unwrap();
            // .intersection() will not easily work iteratively for 3 sets:
            badges.retain(|item| items[0].contains(item) && items[1].contains(item));
            badges
        })
        .map(|badges| {
            assert_eq!(badges.len(), 1);
            let badge = badges.into_iter().next().unwrap();
            priority(badge)
        })
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str =
"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn get_priority() {
        let lines = INPUT.lines().map(|s| s.to_string());
        let priority = duplicates(lines);
        assert_eq!(priority, 157);
    }

    #[test]
    fn get_badges() {
        let lines = INPUT.lines().map(|s| s.to_string());
        let priority = badges(lines);
        assert_eq!(priority, 70);
    }

    #[test]
    fn today() {
        day03();
    }
}
