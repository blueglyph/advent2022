use std::str::FromStr;
use itertools::Itertools;
use crate::get_file_lines;

pub fn day01() {
    let lines = get_file_lines("data/01_data.txt");
    let result = process(lines);
    println!("max = {}, sum of 3 max = {}", result.0, result.1);
}

/// Takes string numbers separated by empty strings, extracts
/// - a = the maximum value
/// - b = the sum of the 3 maximum values
///
/// Returns (a, b)
fn process<E>(lines: E) -> (i32, i32) where E: Iterator<Item = String> {
    let values = lines
        .map(|line| if line.is_empty() { -1 } else { i32::from_str(&line).unwrap() })
        .coalesce(|i1, i2|
            if i1 >= 0 && i2 >= 0 {
                Ok(i1 + i2)
            } else {
                Err((i1, i2))
            }
        )
        .sorted_by(|a, b| b.cmp(a))
        .collect::<Vec<_>>();
    (values[0], values.iter().take(3).sum())
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn it_works() {
        let lines = INPUT.lines().map(|s| s.to_string());
        assert_eq!(process(lines), (24000, 45000));
    }

    #[test]
    fn today() {
        day01();
    }
}
