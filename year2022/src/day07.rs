use common::day::Day;

use std::collections::{HashMap, HashSet};

pub struct Day07 {}

enum Node {
    Dir { children: HashSet<String> },
    File { size: usize },
}

struct Tree {
    nodes: HashMap<String, Node>,
}

impl Tree {
    fn build(input: &str) -> Tree {
        let mut nodes = HashMap::new();
        let mut current_dir = String::from("/");
        for line in input.lines() {
            let tokens: Vec<_> = line.split_whitespace().collect();
            if tokens[0] == "$" && tokens[1] == "cd" {
                // change directory
                match tokens[2] {
                    "/" => {
                        current_dir = String::from("/");
                    }
                    ".." => {
                        if let Some((parent, _)) = current_dir.rsplit_once('/') {
                            current_dir = String::from(parent);
                        }
                    }
                    _ => {
                        current_dir = format!("{}/{}", current_dir, tokens[2]);
                    }
                }
            } else if tokens[0] == "$" && tokens[1] == "ls" {
                // directory listing starts
                // ignore
            } else {
                // directory listing entry
                let parent = nodes.entry(current_dir.clone()).or_insert(Node::Dir {
                    children: HashSet::new(),
                });
                let child_path = format!("{}/{}", current_dir, tokens[1]);

                match parent {
                    Node::Dir { children } => {
                        children.insert(child_path.clone());
                    }
                    Node::File { size: _ } => {
                        unreachable!();
                    }
                }

                if tokens[0] == "dir" {
                    nodes.insert(
                        child_path,
                        Node::Dir {
                            children: HashSet::new(),
                        },
                    );
                } else {
                    nodes.insert(
                        child_path,
                        Node::File {
                            size: tokens[0].parse().unwrap(),
                        },
                    );
                }
            }
        }
        Tree { nodes }
    }

    fn size(&self, path: &str) -> usize {
        match self.nodes.get(path).unwrap() {
            Node::Dir { children } => children.iter().map(|p| self.size(p)).sum(),
            Node::File { size } => *size,
        }
    }

    fn size_all(&self, limit: usize) -> usize {
        self.nodes
            .iter()
            .filter_map(|(path, node)| {
                if let Node::Dir { children: _ } = node {
                    let size = self.size(path);
                    if size <= limit {
                        Some(size)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .sum()
    }

    fn size_min_threshold(&self, threshold: usize) -> usize {
        self.nodes
            .iter()
            .filter_map(|(path, node)| {
                if let Node::Dir { children: _ } = node {
                    let size = self.size(path);
                    if size >= threshold {
                        Some(size)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .min()
            .unwrap()
    }
}

impl Day for Day07 {
    fn star1(&self, input: &str) -> String {
        let tree = Tree::build(input);
        format!("{}", tree.size_all(100000))
    }

    fn star2(&self, input: &str) -> String {
        let tree = Tree::build(input);
        let size_root = tree.size("/");
        let current_free = 70_000_000 - size_root;
        let required_to_free = 30_000_000 - current_free;

        format!("{}", tree.size_min_threshold(required_to_free))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"$ cd /
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
7214296 k"#;

        let d = Day07 {};
        assert_eq!(d.star1(input), "95437");
        assert_eq!(d.star2(input), "24933642");
    }
}
