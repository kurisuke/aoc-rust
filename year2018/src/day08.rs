use common::day::Day;

pub struct Day08 {}

struct Node {
    metadata: Vec<usize>,
    children: Vec<Node>,
}

fn parse_node(nums: &[usize]) -> (Node, usize) {
    let num_child_nodes = nums[0];
    let num_metadata = nums[1];
    let mut idx = 2;
    let children: Vec<_> = (0..num_child_nodes)
        .map(|_| {
            let (node, consumed) = parse_node(&nums[idx..]);
            idx += consumed;
            node
        })
        .collect();
    let metadata = nums[idx..idx + num_metadata].to_vec();
    idx += num_metadata;
    let node = Node { metadata, children };
    (node, idx)
}

fn parse_input(input: &str) -> Node {
    let nums: Vec<_> = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    parse_node(&nums).0
}

fn sum_metadata(node: &Node) -> usize {
    node.metadata.iter().sum::<usize>()
        + node.children.iter().map(|c| sum_metadata(c)).sum::<usize>()
}

fn value(node: &Node) -> usize {
    if node.children.is_empty() {
        node.metadata.iter().sum::<usize>()
    } else {
        node.metadata
            .iter()
            .map(|idx| {
                if *idx == 0 || *idx > node.children.len() {
                    0
                } else {
                    value(&node.children[idx - 1])
                }
            })
            .sum::<usize>()
    }
}

impl Day for Day08 {
    fn star1(&self, input: &str) -> String {
        let tree = parse_input(input);
        format!("{}", sum_metadata(&tree))
    }

    fn star2(&self, input: &str) -> String {
        let tree = parse_input(input);
        format!("{}", value(&tree))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day08 {};
        let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        assert_eq!(d.star1(input), "138");
        assert_eq!(d.star2(input), "66");
    }
}
