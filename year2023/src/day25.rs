use std::collections::HashSet;

use common::day::Day;
use rand::seq::SliceRandom;

pub struct Day25 {}

impl Day for Day25 {
    fn star1(&self, input: &str) -> String {
        let graph = Graph::parse(input);
        loop {
            if let Some(nodes) = karger(&graph, 3) {
                let num_components_1 = nodes[0].len() / 3;
                let num_components_2 = nodes[1].len() / 3;
                return (num_components_1 * num_components_2).to_string();
            }
        }
    }

    fn star2(&self, _input: &str) -> String {
        String::from("not implemented")
    }
}

#[derive(Clone)]
struct Graph {
    nodes: HashSet<String>,
    edges: Vec<(String, String)>,
}

impl Graph {
    fn parse(input: &str) -> Graph {
        let mut nodes = HashSet::new();
        let mut edges = vec![];

        for line in input.lines() {
            let (node, connected_nodes) = line.split_once(": ").unwrap();
            nodes.insert(node.to_string());
            for connected in connected_nodes.split_ascii_whitespace() {
                edges.push((node.to_string(), connected.to_string()));
            }
        }

        Graph { nodes, edges }
    }

    fn contract(&mut self, node1: &str, node2: &str) {
        let node_merged = format!("{}{}", node1, node2);

        self.edges
            .retain(|(n1, n2)| !((n1 == node1 && n2 == node2) || (n1 == node2 && n2 == node1)));

        for edge in self.edges.iter_mut() {
            if edge.0 == node1 || edge.0 == node2 {
                edge.0 = node_merged.clone();
            }
            if edge.1 == node1 || edge.1 == node2 {
                edge.1 = node_merged.clone();
            }
        }

        self.nodes.remove(node1);
        self.nodes.remove(node2);
        self.nodes.insert(node_merged);
    }
}

fn karger(graph: &Graph, remaining_edges: usize) -> Option<Vec<String>> {
    let mut graph = graph.clone();
    let mut rng = rand::thread_rng();

    while graph.nodes.len() > 2 {
        let cut_edge = graph.edges.choose(&mut rng).unwrap().to_owned();
        graph.contract(&cut_edge.0, &cut_edge.1);
    }

    if graph.edges.len() <= remaining_edges {
        Some(graph.nodes.into_iter().collect())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr"#;

    #[test]
    fn ex1() {
        let d = Day25 {};
        assert_eq!(d.star1(INPUT), "54");
    }

    #[test]
    fn test_contract() {
        let mut graph = Graph::parse(INPUT);
        assert!(graph
            .edges
            .contains(&("cmg".to_string(), "qnr".to_string())));

        graph.contract("cmg", "qnr");
        assert!(!graph.nodes.contains(&"cmg".to_string()));
        assert!(!graph.nodes.contains(&"qnr".to_string()));
        assert!(graph.nodes.contains(&"cmgqnr".to_string()));
        assert!(!graph
            .edges
            .contains(&("cmg".to_string(), "qnr".to_string())));
        assert!(graph
            .edges
            .contains(&("cmgqnr".to_string(), "lhk".to_string())));
        assert!(graph
            .edges
            .contains(&("frs".to_string(), "cmgqnr".to_string())));
    }
}
