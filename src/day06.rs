use std::collections::HashSet;
use itertools::Itertools;
use crate::get_file_lines;

#[allow(dead_code)]

pub fn day06() {
    let packets = packet_offsets(get_file_lines("data/06_data.txt"))
        .map(|offset: usize| offset.to_string())
        .join(", ");
    println!("Packet offsets: {packets}");
    let messages = message_offsets(get_file_lines("data/06_data.txt"))
        .map(|offset: usize| offset.to_string())
        .join(", ");
    println!("Message offsets: {messages}");
}

/// Finds the offset of the packet in each line. The offset is the first
/// character after the marker, defined as 4 consecutive different characters.
fn packet_offsets<E>(lines: E) -> impl Iterator<Item = usize>
    where E: Iterator<Item = String>
{
    lines.map(|line| {
        line.chars()
            .tuple_windows()
            .position(|(a, b, c, d)| a != b && b != c && c != d && a != c && a != d && b != d)
            .and_then(|x| Some(x + 4))
            .unwrap_or(0)
    })
}

/// Finds the offset of the message in each line. The offset is the first
/// character after the marker, defined as 14 consecutive different characters.
fn message_offsets<E>(lines: E) -> impl Iterator<Item = usize>
    where E: Iterator<Item = String>
{
    lines.map(|line| {
        (0..=line.len() - 14)
            .find_map(|i| {
                let s = &line[i..i+14].chars().collect::<HashSet<_>>();
                if s.len() == 14 { Some(i + 14) } else { None }
            })
            .unwrap_or(0)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str =
"mjqjpqmgbljsphdztnvjfqwrcgsmlb
bvwbjplbgvbhsrlpgdmjqwftvncz
nppdvjthqldpwncqszvftbrmjlhg
nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg
zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn get_packets() {
        let lines = INPUT.lines().map(|s| s.to_string());
        let offsets = packet_offsets(lines).collect::<Vec<_>>();
        println!("offsets: {offsets:?}");
        assert_eq!(offsets, vec![7, 5, 6, 10, 11]);
    }

    #[test]
    fn get_messages() {
        let lines = INPUT.lines().map(|s| s.to_string());
        let offsets = message_offsets(lines).collect::<Vec<_>>();
        println!("offsets: {offsets:?}");
        assert_eq!(offsets, vec![19, 23, 23, 29, 26]);
    }

    #[test]
    fn today() {
        day06();
    }
}
