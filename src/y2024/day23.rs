use std::fs;

use rustc_hash::FxHashSet;
use petgraph::graph::{NodeIndex, UnGraph};

use crate::solutions;

solutions!{2024, 23}

type Triangles = Vec<[u16; 3]>;

fn get_input(file: &str) -> (UnGraph<(), (), u16>, Triangles) {
    let graph = UnGraph::from_edges(
        fs::read_to_string(file).expect("No file there").lines().map(|line| {
            let (left, right) = line.split_once("-").unwrap();
            let (mut cleft, mut cright) = (left.chars(), right.chars());

            ((((cleft.next().unwrap() as u16) << 8 ) | cleft.next().unwrap() as u16), (((cright.next().unwrap() as u16) << 8 ) | cright.next().unwrap() as u16))
        })
    );

    let triangles = find_triangles(&graph);

    (graph, triangles)
}

// https://www.cs.cornell.edu/courses/cs6241/2019sp/readings/Chiba-1985-arboricity.pdf
fn find_triangles(graph: &UnGraph<(), (), u16>) -> Triangles {
    let mut graph = graph.clone();

    let mut nodes: Vec<Vec<NodeIndex<u16>>> = vec![Vec::with_capacity(5); graph.node_indices().map(|n| graph.edges(n).count()).max().unwrap() + 1];
    
    for node in graph.node_indices() {
        nodes[graph.edges(node).count()].push(node);
    }
    
    let mut triangles: Triangles = Vec::new();

    let mut marked: Vec<u16> = vec![0; u16::MAX as usize + 1];

    let mut round = 1;

    for &node in nodes.iter().flatten().rev() {
        for neighbor in graph.neighbors(node) {
            marked[neighbor.index()] = round;
        }

        for neighbor in graph.neighbors(node) {
            for next_neighbor in graph.neighbors(neighbor) {

                if next_neighbor == node {
                    continue;
                }

                if marked[next_neighbor.index()] == round {
                    triangles.push([node.index() as u16, neighbor.index() as u16, next_neighbor.index() as u16]);
                }
            }
            marked[neighbor.index()] = 0;
        }
        
        graph.remove_node(node);
        round += 1;
    }

    triangles
}

fn solve_first(input: &(UnGraph<(), (), u16>, Triangles)) -> usize {
    input.1
        .iter()
        .filter(|pc| (pc[0] >> 8) as u8 == b't' || (pc[1] >> 8) as u8 == b't' || (pc[2] >> 8) as u8 == b't')
        .count()
}

// https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm
fn bron_kerboschl(graph: &UnGraph<(), (), u16>, r: FxHashSet<NodeIndex<u16>>, p: FxHashSet<NodeIndex<u16>>, x: FxHashSet<NodeIndex<u16>>) -> Option<FxHashSet<NodeIndex<u16>>> {
    if p.is_empty() && x.is_empty() {
        Some(r)
    } else {
        let mut candidate: Option<FxHashSet<NodeIndex<u16>>> = None;
        let mut p = p;
        let mut x = x;

        let pivot = p.iter().next().unwrap_or_else(|| x.iter().next().unwrap());

        for node in p.clone().difference(&FxHashSet::from_iter(graph.neighbors(*pivot))) {
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

fn solve_second(input: &(UnGraph<(), (), u16>, Triangles)) -> String {
    let p = FxHashSet::from_iter(input.1.iter().flatten().map(|num| NodeIndex::from(*num)));
    let x = FxHashSet::from_iter(FxHashSet::from_iter(input.0.node_indices()).difference(&p).copied());
    let result = bron_kerboschl(&input.0, FxHashSet::default(), p, x);

    let mut pcs: Vec<NodeIndex<u16>> = result.unwrap().drain().collect();
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
