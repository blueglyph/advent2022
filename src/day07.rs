use crate::get_file_lines;

#[allow(dead_code)]

const THRESHOLD: u32     =    100_000;
const DISK_CAPACITY: u32 = 70_000_000;
const UPDATE_SIZE: u32   = 30_000_000;

pub fn day07() {
    let total = find_small_dirs(get_file_lines("data/07_data.txt"), false);
    println!("1) Size small directories: {total}");
    let size = select_dir(get_file_lines("data/07_data.txt"), false);
    println!("2) Minimum saved size: {size}");
}

#[derive(Debug)]
struct Node {
    pub name: String,
    pub size: u32,
    pub dirs: Vec<Node>
}

impl Node {
    pub fn new(name: String) -> Self {
        Node { name, size: 0, dirs: Vec::new() }
    }

    /// Walks through all the nodes and executes `f` on each of them.
    fn walk<F>(&self, mut f: F) where F: FnMut(&Node) {
        let mut stack = Vec::<&Node>::new();
        stack.push(self);
        while !stack.is_empty() {
            let node = stack.pop().unwrap();
            f(node);
            for subnode in node.dirs.iter().rev() {
                stack.push(subnode)
            }
        }
    }
}

/// Parses the commands and builds the directory structure, returning its root.
fn parse<E>(lines: E) -> Node
    where E: Iterator<Item = String>
{
    //                     stack = (/, 0, [])
    // cd a             -> stack = (/, 0, []), (a, 0, [])
    // 1                -> stack = (/, 0, []), (a, 1, [])
    // 2                -> stack = (/, 0, []), (a, 3, [])
    // cd a1            -> stack = (/, 0, []), (a, 3, []), (a1, 0, [])
    // 4                -> stack = (/, 0, []), (a, 3, []), (a1, 4, [])
    // cd ..            -> stack = (/, 0, []), (a, 7, [a1])
    // cd ..            -> stack = (/, 7, [])
    let mut stack: Vec<Node> = Vec::new();
    for mut line in lines {
        if line.starts_with("$ cd ") {
            let name = line.split_off(5);
            match name.as_str() {
                ".." => {
                    let node = stack.pop().unwrap();
                    let parent = stack.last_mut().unwrap();
                    parent.size += node.size;
                    parent.dirs.push(node);
                }
                _ => {
                    stack.push(Node::new(name));
                }
            }
        } else if line.starts_with(|c: char| c.is_ascii_digit()) {
            let mut node = stack.last_mut().unwrap();
            node.size += line.split_whitespace().next().unwrap().parse::<u32>().unwrap();
        }
    }
    while stack.len() > 1 {
        let node = stack.pop().unwrap();
        let parent = stack.last_mut().unwrap();
        parent.size += node.size;
        parent.dirs.push(node);
    }
    stack.pop().unwrap()
}

/// Sum of all directories with a total size of at most `THRESHOLD`, including possible overlaps.
fn find_small_dirs<E>(lines: E, verbose: bool) -> u32
    where E: Iterator<Item = String>
{
    let root = parse(lines);
    let mut total = 0;
    root.walk(|n| {
        if verbose {
            println!("- {}: {}", n.name, n.size);
        }
        if n.size <= THRESHOLD {
            total += n.size
        }
    });
    total
}

fn select_dir<E>(lines: E, verbose: bool) -> u32
    where E: Iterator<Item = String>
{
    let root = parse(lines);
    let current_occupied = root.size;
    let current_free = DISK_CAPACITY - current_occupied;
    println!("Occupied:     {current_occupied:9}");
    println!("Free space:   {current_free:9}");
    if current_free > UPDATE_SIZE {
        println!("no need to delete anything");
        return 0;
    }
    let missing_space = UPDATE_SIZE - current_free;
    println!("Need to free: {missing_space:9}");
    let mut min = u32::MAX;
    root.walk(|n| {
        if n.size >= missing_space && n.size < min {
            min = n.size;
            if verbose {
                println!("- found:      {:9} occupied by {}", n.size, n.name)
            }
        }
    });
    min
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str =
"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn get_small_dirs() {
        let lines = INPUT.lines().map(|s| s.to_string());
        let size = find_small_dirs(lines, true);
        println!("size: {size}");
    }

    #[test]
    fn get_smallest_dir() {
        let lines = INPUT.lines().map(|s| s.to_string());
        let size = select_dir(lines, true);
        println!("size: {size}");
        assert_eq!(size, 24933642);
    }

    #[test]
    fn today() {
        day07();
    }
}
