use std::fs;

use fxhash::FxHashSet;
use petgraph::graph::{NodeIndex, UnGraph};

use crate::Solution;

pub fn solutions() -> Solution {
    let input = get_input("inputs/2024/day23.txt");

    Solution::evaluated(
        "Day 23".to_owned(), 
        &|| solve_first(input.clone()),
        &|| solve_second(input.clone())
    )
}

fn get_input(file: &'static str) -> UnGraph<(), (), u16> {
    UnGraph::from_edges(
        fs::read_to_string(file).expect("No file there").lines().map(|line| {
            let (left, right) = line.split_once("-").unwrap();
            let (mut cleft, mut cright) = (left.chars(), right.chars());

            ((((cleft.next().unwrap() as u16) << 8 ) | cleft.next().unwrap() as u16), (((cright.next().unwrap() as u16) << 8 ) | cright.next().unwrap() as u16))
        })
    )
}

// https://www.cs.cornell.edu/courses/cs6241/2019sp/readings/Chiba-1985-arboricity.pdf
fn solve_first(input: UnGraph<(), (), u16>) -> usize {
    let mut graph = input;

    let mut nodes: Vec<NodeIndex<u16>> = graph.node_indices().collect();
    nodes.sort_by_cached_key(|node| usize::MAX - graph.neighbors(*node).count());

    let mut triangles: Vec<(u16, u16, u16)> = Vec::new();

    for node in nodes {
        let neighbors = graph.neighbors(node);
        let mut marked = FxHashSet::from_iter(neighbors.clone());
        for neighbor in neighbors {
            for next_neighbor in graph.neighbors(neighbor) {

                if next_neighbor == node {
                    continue;
                }

                if marked.contains(&next_neighbor) {
                    triangles.push((node.index() as u16, neighbor.index() as u16, next_neighbor.index() as u16));
                }
            }
            marked.remove(&neighbor);
        }
        graph.remove_node(node);
    }

    triangles
        .iter()
        .filter(|pc| (pc.0 >> 8) as u8 == b't' || (pc.1 >> 8) as u8 == b't' || (pc.2 >> 8) as u8 == b't')
        .count()
}

// https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm
fn bron_kerboschl(graph: &UnGraph<(), (), u16>, r: FxHashSet<NodeIndex<u16>>, p: FxHashSet<NodeIndex<u16>>, x: FxHashSet<NodeIndex<u16>>) -> Option<FxHashSet<NodeIndex<u16>>> {
    if p.is_empty() && x.is_empty() {
        Some(r.clone())
    } else {
        let mut candidate: Option<FxHashSet<NodeIndex<u16>>> = None;
        let mut p = p;
        let mut x = x;

        for node in p.clone().iter() {
            let neighbors = FxHashSet::from_iter(graph.neighbors(*node));

            let mut nr = r.clone();
            nr.insert(*node);
            let np = FxHashSet::from_iter(p.intersection(&neighbors).copied());
            let nx = FxHashSet::from_iter(x.intersection(&neighbors).copied());

            if let Some(clique) = bron_kerboschl(graph, nr, np, nx) {
                if candidate.is_none() || candidate.as_ref().unwrap().len() < clique.len() {
                    candidate = Some(clique);
                }
            }

            p.remove(node);
            x.insert(*node);
        }

        candidate
    }
}

fn solve_second(input: UnGraph<(), (), u16>) -> String {
    let result = bron_kerboschl(&input, FxHashSet::default(), FxHashSet::from_iter(input.node_indices()), FxHashSet::default());

    let mut pcs: Vec<NodeIndex<u16>> = result.unwrap().iter().copied().collect();
    pcs.sort();

    let mut password = String::new();
    
    for pc in pcs {
        if !password.is_empty() {
            password.push(',');
        }

        password.push((pc.index() >> 8) as u8 as char);
        password.push((pc.index() & 0xFF) as u8 as char);
    }

    password
}
