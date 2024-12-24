use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn parse_input(input: &str) -> Graph {
    let mut result = Graph::new();
    
    let edges = input
        .trim()
        .lines()
        .filter_map(|line| {
            let edges: Vec<&str> = line.trim().split("-").collect();
            if edges.len() < 2 {
                None
            } else {
                Some((edges[0], edges[1]))
            }
        })
        .collect();
    
    result.build_graph(&edges);
    
    result
}

type Node<'a> = &'a str;

struct Graph<'a> {
    nodes: Vec<Node<'a>>,
    edges: Vec<(usize, usize)>,
}

impl<'a> Graph<'a> {
    fn new() -> Self {
        Graph{
            nodes: vec![],
            edges: vec![],
        }
    }

    fn node_idx(&self, node: Node) -> Option<usize> {
        self.nodes.iter().position(|i| *i == node)
    }

    fn add_unique_node(&mut self, n: Node<'a>) -> usize {
        let pos = self.node_idx(n);
        if pos.is_some() {
            return pos.unwrap();
        }

        let res = self.nodes.len();
        self.nodes.push(n);

        res
    }

    fn add_edge(&mut self, node_index1: usize, node_index2: usize) -> bool {
        let n_len = self.nodes.len();
        if node_index1 >= n_len || node_index2 >= n_len || node_index1 == node_index2 { 
            false
        } else {
            self.edges.push((node_index1, node_index2));
            true
        }
    }

    fn build_graph(&mut self, edges: &Vec<(Node<'a>, Node<'a>)>) {
        for edge in edges {
            let idx1 = self.add_unique_node(edge.0);
            let idx2 = self.add_unique_node(edge.1);

            self.add_edge(idx1, idx2);
        }
    }

    fn neighbors(&self, node_idx: usize) -> Vec<usize> {
        self.edges
            .iter()
            .filter_map(|&(idx1, idx2)| {
                if idx1 == node_idx {
                    Some(idx2)
                } else if idx2 == node_idx {
                    Some(idx1)
                } else {
                    None
                }
            })
            .collect()
    }

    fn check_clique_candidate(&self, nodes: &Vec<usize>) -> bool{
        nodes
            .iter()
            .combinations(2)
            .all(|edges| {
                edges.len() == 2 && (self.edges.contains(&(*edges[0], *edges[1])) || self.edges.contains(&(*edges[1], *edges[0])))
            })
    }

    fn cliques(&self, nodes: &Vec<usize>, clique_len: usize) -> HashSet<Vec<usize>> {
        let mut res = HashSet::new();
        for node_idx in nodes {
            let neighbors = self.neighbors(*node_idx);

            if neighbors.len() < clique_len {
                continue;
            }

            neighbors
                .iter()
                .combinations(clique_len-1)
                .filter_map(|candidates| {
                    let mut group = vec![node_idx];
                    
                    group.extend(candidates);
                    group.sort();
                    let group = group.into_iter().map(|x|x.to_owned()).collect();

                    if self.check_clique_candidate(&group) {
                        Some(group)
                    } else {
                        None
                    }
                })
                .for_each(|group| {
                    res.insert(group);
                });           
        }
        res
    }

    fn stringify(&'a self, group: &[usize]) -> Vec<&'a str> {
        group
            .iter()
            .map(|&idx| self.nodes[idx])
            .collect()
    }
}

pub fn process_part1(input: &str) -> String {
    let graph = parse_input(input);

    let mut res = vec![];
    for (idx, node) in graph.nodes.iter().enumerate() {
        if !node.starts_with("t") {
            continue
        }

        res.push(idx);
    }

    graph.cliques(&res, 3)
        .iter()
        .count()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let graph = parse_input(input);

    let nodes = (0..graph.nodes.len()).collect();

    let mut max_clique = vec![];
    for i in (1..100).rev() {
        let result = graph.cliques(&nodes, i);
        if result.len() > 0 {
            max_clique = result.iter().nth(0).unwrap().to_owned();
            break;
        }
    }
        
    let mut password = graph.stringify(&max_clique);
    password.sort();

    password.join(",").to_string()

}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn test_process_part1() {
        println!("{}", process_part1(INPUT));
    }
    
    #[test]
    fn test_process_part2() {
        println!("{}", process_part2(INPUT));
    }
}