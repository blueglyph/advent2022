use std::collections::VecDeque;
use itertools::Itertools;
use regex::Regex;
use crate::get_file_lines;

#[allow(dead_code)]
#[derive(PartialEq)]
enum Verbose { Quiet, Final, All }

pub fn day05() {
    let crates = top_crates(get_file_lines("data/05_data.txt"), false, Verbose::Final);
    println!("CrateMover 9000, top crates: {crates}");
    let crates = top_crates(get_file_lines("data/05_data.txt"), true, Verbose::Final);
    println!("CrateMover 9001, top crates: {crates}");
}

/// Displays the stacks.
fn show_stacks(text: &str, stacks: &Vec<VecDeque<char>>) {
    println!("{text}");
    for (i, s) in stacks.iter().enumerate() {
        println!("- {:2}: {}", i + 1, s.iter()
            .map(|&c| if c == ' ' { "   ".to_string() } else { format!("[{c}]") })
            .join(" ")
        )
    }
}

/// Simulates a crane moving crates from stack to stack.
/// The initial position of the crates and the moves are described in `lines`.
/// - multi: true if the crane moves several crates at the same time, false if it picks them one at a time
/// - verbose: level of verbosity
fn top_crates<E>(mut lines: E, multi: bool, verbose: Verbose) -> String where E: Iterator<Item = String> {
    // extract the initial content of the stacks
    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    let re_crates = Regex::new("(?:.(.).) ?").unwrap();
    let re_nums = Regex::new("\\d+").unwrap();

    while let Some(line) = lines.next() {
        if !line.contains('[') {
            // stack numbers, check if there are any empty stacks to add
            let numbers = re_nums.find_iter(&line).map(|m| m.as_str()).count();
            for _ in stacks.len()..numbers {
                stacks.push(VecDeque::new());
            }
            break;
        }
        // captures the items, ' ' is nothing, otherwise it's a crate
        let crates = re_crates.captures_iter(&line)
            .map(|m| m.get(1).unwrap().as_str().chars().next().unwrap())
            .collect::<Vec<_>>();
        for _ in stacks.len()..crates.len() {
            stacks.push(VecDeque::new());
        }
        for (s, c) in stacks.iter_mut().zip(crates).filter(|(_, c)| *c != ' ') {
            s.push_front(c);
        }
    }
    if verbose == Verbose::All {
        show_stacks("Initial:", &stacks);
    }

    // empty line
    lines.next();

    // move crates
    while let Some(line) = lines.next() {
        let numbers = re_nums.find_iter(&line).map(|m| m.as_str().parse::<usize>().unwrap()).collect::<Vec<_>>();
        let (num, from, to) = (numbers[0], numbers[1] - 1, numbers[2] - 1);
        let mut chunk = VecDeque::<char>::new();
        for _ in 0..num {
            let c = stacks[from].pop_back().unwrap();
            if multi {
                chunk.push_front(c);
            } else {
                chunk.push_back(c);
            }
        }
        stacks[to].append(&mut chunk);
        if verbose == Verbose::All {
            show_stacks(&format!("{}: ", line.trim_end()), &stacks);
        }
    }
    if verbose == Verbose::Final {
        show_stacks("Final:", &stacks);
    }

    stacks.iter().map(|s| s.iter().last().unwrap_or(&' ').to_owned()).join("")
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str =
"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn get_top_crates_9000() {
        let lines = INPUT.lines().map(|s| s.to_string());
        let crates = top_crates(lines, false, Verbose::All);
        assert_eq!(crates, "CMZ");
    }

    #[test]
    fn get_top_crates_9001() {
        let lines = INPUT.lines().map(|s| s.to_string());
        let crates = top_crates(lines, true, Verbose::All);
        assert_eq!(crates, "MCD");
    }

    #[test]
    fn today() {
        day05();
    }
}
