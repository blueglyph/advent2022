use crate::get_file_lines;

#[allow(dead_code)]

pub fn day08() {
    let trees = visible_trees(get_file_lines("data/08_data.txt"), false);
    println!("1) Number of visible trees: {trees}");
    let score = scenic_score(get_file_lines("data/08_data.txt"), false);
    println!("2) Scenic score: {score}");
}

/// Parses the input lines and returns the trees.
fn parse_trees<E>(lines: E) -> Vec<Vec<(u8, bool)>>
    where E: Iterator<Item = String>
{
    lines.map(|line|
        line.into_bytes().into_iter().map(|t| (t, false)).collect::<Vec<_>>()
    ).collect::<Vec<_>>()
}

/// Number of visible trees from outside.
fn visible_trees<E>(lines: E, verbose: bool) -> usize
    where E: Iterator<Item = String>
{
    let mut visible: usize = 0;
    let mut trees = parse_trees(lines);
    let h = trees.len();
    let w = trees[0].len();
    let mut hmax: Vec<u8> = vec![0; w];
    for hline in &mut trees {
        for i in 0..w {
            if hline[i].0 >= hmax[i] {
                hline[i].1 = true;
                hmax[i] = hline[i].0 + 1;
                visible += 1;
            }
        }
    }
    let mut hmax: Vec<u8> = vec![0; w];
    for hline in &mut trees.iter_mut().rev() {
        for i in 0..w {
            if hline[i].0 >= hmax[i] {
                if !hline[i].1 {
                    hline[i].1 = true;
                    visible += 1;
                }
                hmax[i] = hline[i].0 + 1;
            }
        }
    }
    let mut vmax_l: Vec<u8> = vec![0; h];
    let mut vmax_r: Vec<u8> = vec![0; h];
    for x in 0..w {
        for y in 0..h {
            let tree = &mut trees[y][x];
            if tree.0 >= vmax_l[y] {
                if !tree.1 {
                    tree.1 = true;
                    visible += 1;
                }
                vmax_l[y] = tree.0 + 1;
            }
            let tree = &mut trees[y][w - x - 1];
            if tree.0 >= vmax_r[y] {
                if !tree.1 {
                    tree.1 = true;
                    visible += 1;
                }
                vmax_r[y] = tree.0 + 1;
            }
        }
    }
    if verbose {
        print_forest(&trees, "finished:");
    }
    visible
}

/// Shows the forest and the tagged trees (debug feature).
fn print_forest(trees: &Vec<Vec<(u8, bool)>>, x1: &str) {
    println!("{x1}");
    for hline in trees {
        for (h, done) in hline {
            let c = *h as char;
            if *done {
                print!("[{c}]");
            } else {
                print!(" {c} ");
            }
        }
        println!();
    }
}

/// Best scenic score.
fn scenic_score<E>(lines: E, verbose: bool) -> usize
    where E: Iterator<Item = String>
{
    let mut score = 0;
    let mut coord = (0, 0);
    let mut trees = parse_trees(lines);
    let h = trees.len();
    let w = trees[0].len();
    for y in 1..h-1 {
        for x in 1..w-1 {
            let tree = trees[y][x];
            let d = [(-1, 0), (1, 0), (0, -1), (0, 1)].iter()
                .map(|delta| distance(&trees, w, h, tree.0, (x, y), *delta))
                .product();
            if d > score {
                coord = (x, y);
                score = d;
            }
        }
    }
    if verbose {
        trees[coord.1][coord.0].1 = true;
        print_forest(&trees, &format!("best score: {score}"));
    }
    score
}

/// Distance to a higher tree at the `pos` position in the `dpos` direction.
fn distance(trees: &Vec<Vec<(u8, bool)>>, w: usize, h: usize, height: u8, mut pos: (usize, usize), dpos: (isize, isize)) -> usize {
    let mut d = 0;
    while 0 < pos.0 && pos.0 < w - 1 && 0 < pos.1 && pos.1 < h - 1 {
        d += 1;
        pos = ((pos.0 as isize + dpos.0) as usize, (pos.1 as isize + dpos.1) as usize);
        if trees[pos.1][pos.0].0 >= height { break }
    }
    d
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
30373
25512
65332
33549
35390";

    #[test]
    fn get_visible_trees() {
        let lines = INPUT.lines().map(|s| s.to_string());
        let trees = visible_trees(lines, true);
        assert_eq!(trees, 21);
    }

    #[test]
    fn get_scenic_score() {
        let lines = INPUT.lines().map(|s| s.to_string());
        let score = scenic_score(lines, true);
        assert_eq!(score, 8);
    }

    #[test]
    fn today() {
        day08();
    }
}
