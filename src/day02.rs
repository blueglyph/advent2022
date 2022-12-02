use crate::get_file_lines;

pub fn day02() {
    let result_1 = process(get_file_lines("data/02_data.txt"), false);
    let result_2 = process(get_file_lines("data/02_data.txt"), true);
    println!("score 1 = {result_1}, score 2 = {result_2}");
}

/// If strategy == false: X, Y, Z means rock, paper, scissor
/// if strategy == true: X, Y, Z means lose, draw, win
fn process<E>(lines: E, strategy: bool) -> i32 where E: Iterator<Item = String> {
    lines
        .map(|s| s.into_bytes())
        .map(|bytes| ((bytes[0] - b'A') as i32, (bytes[2] - b'X') as i32))
        // .inspect(|(a, b)| println!("elf={a} player={b}"))
        .map(|(elf, hint)| {
            let player = if strategy {
                // 0 = lose, 1 = draw, 2 = win
                (elf + hint + 2) % 3
            } else {
                // 0 = rock, 1 = paper, 2 = scissor
                hint
            };
            let outcome = match (player - elf + 3) % 3 {
                1 => 6,
                2 => 0,
                _ => 3
            };
            player + 1 + outcome
        })
        // .inspect(|score| println!(" -> score {score}"))
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn it_works_no_strategy() {
        let lines = INPUT.lines().map(|s| s.to_string());
        let score = process(lines, false);
        assert_eq!(score, 15);
    }

    #[test]
    fn it_works_strategy() {
        let lines = INPUT.lines().map(|s| s.to_string());
        let score = process(lines, true);
        assert_eq!(score, 12);
    }

    #[test]
    fn today() {
        day02();
    }
}
