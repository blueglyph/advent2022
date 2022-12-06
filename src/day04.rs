use crate::get_file_lines;

pub fn day04() {
    let overlap = calc_overlap(get_file_lines("data/04_data.txt"), false);
    println!("# contain: {overlap}");
    let overlap = calc_overlap(get_file_lines("data/04_data.txt"), true);
    println!("# overlap: {overlap}");
}

/// range a contains range b
fn contains(a: (u32, u32), b: (u32, u32)) -> bool {
    a.0 <= b.0 && b.1 <= a.1
}

/// ranges a and b overlap
fn overlaps(a: (u32, u32), b: (u32, u32)) -> bool {
    !(b.1 < a.0 || a.1 < b.0)
}

/// Counts the number of range couples where
/// - partial=false: one range includes another (all IDs of one are included in the other)
/// - partial=true: ranges overlap (one includes at least an ID of the other)
fn calc_overlap<E>(lines: E, partial: bool) -> u32 where E: Iterator<Item = String> {
    lines
        .map(|line| {
            let id = line.split(['-', ',']).map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>();
            assert!(id[0] <= id[1]);
            assert!(id[2] <= id[3]);
            ((id[0], id[1]), (id[2], id[3]))
        })
        .filter(|&(range1, range2)|
            if partial {
                overlaps(range1, range2)
            } else {
                contains(range1, range2) || contains(range2, range1)
            }
        )
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str =
"2-4,6-8
02-3,4-05
05-07,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn get_contain() {
        let lines = INPUT.lines().map(|s| s.to_string());
        let contain = calc_overlap(lines, false);
        assert_eq!(contain, 2);
    }

    #[test]
    fn get_overlap() {
        let lines = INPUT.lines().map(|s| s.to_string());
        let overlap = calc_overlap(lines, true);
        assert_eq!(overlap, 4);
    }

    #[test]
    fn today() {
        day04();
    }
}
