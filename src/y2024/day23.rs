use std::fs;

use rustc_hash::{FxBuildHasher, FxHashSet};
use petgraph::{graph::{NodeIndex, UnGraph}, visit::EdgeRef};

use crate::solutions;

solutions!{2024, 23}

type Triangles = Vec<[u16; 3]>;
type NodeSet = FxHashSet<NodeIndex<u16>>;

fn get_input(file: &str) -> (UnGraph<(), (), u16>, NodeSet) {
    let graph = UnGraph::from_edges(
        fs::read_to_string(file).expect("No file there").lines().map(|line| {
            let mut characters = line.chars();

            // A nodes index is the first char << 8 | the second char
            let left = ((characters.next().unwrap() as u16) << 8) | characters.next().unwrap() as u16;
            characters.next();
            let right = ((characters.next().unwrap() as u16) << 8) | characters.next().unwrap() as u16;

            (left, right)
        })
    );

    // The graph contains 520 nodes, but the graph library fills in all nodes in between automatically
    // So we save the nodes here as well, so that we don't process nodes that don't have any edges
    let mut nodes = FxHashSet::with_capacity_and_hasher(520, FxBuildHasher);
    nodes.extend(graph.raw_edges().iter().flat_map(|edge| [edge.source(), edge.target()]));

    (graph, nodes)
}

/// An algorithm for finding triangles in a graph as described by [this paper](https://www.cs.cornell.edu/courses/cs6241/2019sp/readings/Chiba-1985-arboricity.pdf)
/// 
/// For each node in the graph, mark all its neighbors. For all marked nodes neighbors, check if they are marked
/// If so, then a triangle has been found. After processing a marked nodes neighbors, remove its edge to the inital node
fn find_triangles(graph: &UnGraph<(), (), u16>, nodes: &NodeSet) -> Triangles {
    let mut graph = graph.clone();

    let mut triangles: Triangles = Vec::new();

    let mut marked: Vec<_> = vec![];

    // There are 520 nodes in the graph, the last 2 can't form a triangle on their own
    for &first_node in nodes.iter().take(518) {
        // Get all neighbors of the starting node
        marked.extend(graph.edges(first_node).map(|e| (e.id(), e.target())));

        while let Some((edge, second_node)) = marked.pop() {
            for third_node in graph.neighbors(second_node) {
                // Check if the third node is also a neighbor of the first node
                for &(_, test_second_node) in &marked {
                    if test_second_node == third_node {
                        triangles.push([first_node.index() as u16, second_node.index() as u16, third_node.index() as u16]);
                        break;
                    }
                }
            }

            // Remove the connection from first to second, to avoid duplicates
            graph.remove_edge(edge);
        }
    }

    triangles
}

/// ### Triangles With T
/// 
/// Finds all triangles, that have at least one computer whose name starts with 't'
fn solve_first(input: &(UnGraph<(), (), u16>, NodeSet)) -> usize {
    find_triangles(&input.0, &input.1)
        .iter()
        .filter(|pc| (pc[0] >> 8) as u8 == b't' || (pc[1] >> 8) as u8 == b't' || (pc[2] >> 8) as u8 == b't')
        .count()
}

/// The [Bron Kerbosch Algorithm](https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm)
/// Finds the largest completely interconnected set of nodes - the largest clique - inside the graph
fn bron_kerbosch(graph: &UnGraph<(), (), u16>, clique: NodeSet, mut candidates: NodeSet, mut excluded: NodeSet) -> Option<NodeSet> {
    // If there are no more candidates, the clique can't grow anymore
    if candidates.is_empty() {
        Some(clique)
    } else {
        let mut largest_clique: Option<NodeSet> = None;

        let pivot = candidates.iter().next().unwrap();

        for node in candidates.clone().difference(&graph.neighbors(*pivot).collect()) {
            let neighbors: NodeSet = graph.neighbors(*node).collect();

            let mut new_clique = clique.clone();
            new_clique.insert(*node);
            let new_candidates = candidates.intersection(&neighbors).copied().collect();
            let new_excluded = excluded.intersection(&neighbors).copied().collect();

            if let Some(clique) = bron_kerbosch(graph, new_clique, new_candidates, new_excluded) {
                if largest_clique.is_none() || largest_clique.as_ref().unwrap().len() < clique.len() {
                    largest_clique = Some(clique);
                }
            }

            candidates.remove(node);
            excluded.insert(*node);
        }

        largest_clique
    }
}

/// ### Largest Clique
/// 
/// Finds the largest clique (interconneced group of vertices)
/// The names of the participating computers are then sorted and assembled into the password
fn solve_second(input: &(UnGraph<(), (), u16>, NodeSet)) -> String {
    let result = bron_kerbosch(&input.0, FxHashSet::default(), input.1.clone(), FxHashSet::default());

    let mut pcs: Vec<NodeIndex<u16>> = result.unwrap().drain().collect();
    pcs.sort();

    let mut password = String::with_capacity(pcs.len() * 3);
    
    for pc in pcs {
        password.push((pc.index() >> 8) as u8 as char);
        password.push((pc.index() & 0xFF) as u8 as char);
        password.push(',');
    }

    // Remove the stray comma
    password.pop().unwrap();

    password
}
