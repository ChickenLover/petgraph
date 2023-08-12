
use std::collections::{HashMap, HashSet, VecDeque};

use crate::algo::IntoNeighbors;
use crate::stable_graph::StableGraph;
use crate::{Graph, Undirected, EdgeType};
use crate::graph::{EdgeIndex, NodeIndex};
use crate::visit::{IntoNodeIdentifiers, NodeIndexable};

fn calculate_betweenness_centrality<G>(graph: G) -> HashMap<(G::NodeId, G::NodeId), f64>
where
    G: NodeIndexable + IntoNodeIdentifiers + IntoNeighbors,
    G::NodeId: std::cmp::Eq + std::hash::Hash
{
    let mut betweenness = HashMap::new();

    // Calculate edge betweenness centrality
    for (i, node) in graph.node_identifiers().into_iter().enumerate() {
        // println!("Processing {i} node...");

        let mut stack = Vec::new();
        let mut predecessors = HashMap::new();
        let mut distance = HashMap::new();
        let mut sigma = HashMap::new();
        let mut queue = VecDeque::new();

        distance.insert(node, 0u128);
        sigma.insert(node, 1u128);
        queue.push_back(node);

        // Breadth-first search
        while let Some(current) = queue.pop_front() {
            stack.push(current);
            let current_distance = *distance.get(&current).unwrap();
            let current_sigma = *sigma.get(&current).unwrap();

            for neighbor in graph.neighbors(current) {
                if !distance.contains_key(&neighbor) {
                    queue.push_back(neighbor);
                    distance.insert(neighbor, current_distance + 1);
                }
                if let Some(neighbor_distance) = distance.get(&neighbor) {
                    if *neighbor_distance == current_distance + 1 {
                        *sigma.entry(neighbor).or_insert(0) += current_sigma;
                        predecessors.entry(neighbor).or_insert(vec![]).push(current);
                    }
                }
            }
        }

        // Accumulate edges
        let mut delta = HashMap::new();
        while let Some(current) = stack.pop() {
            if let None = predecessors.get(&current) {
                continue;
            }
            
            let coeff = (1. + delta.get(&current).unwrap_or(&0.)) / *sigma.get(&current).unwrap() as f64;
            for predecessor in predecessors.get(&current).unwrap() {
                let c = coeff * *sigma.get(predecessor).unwrap() as f64;
                if betweenness.contains_key(&(*predecessor, current)) {
                    *betweenness.get_mut(&(*predecessor, current)).unwrap() += c;
                } else if betweenness.contains_key(&(current, *predecessor)) {
                    *betweenness.get_mut(&(current, *predecessor)).unwrap() += c;
                } else {
                    betweenness.insert((*predecessor, current), c);
                }
                *delta.entry(predecessor).or_insert(0.) += c;
            }
        }

    }

    betweenness
}

/// \[Generic\] Girvan-Newman community detection algorithm.
pub fn girvan_newman<N: Clone, E: Clone, T: EdgeType>(graph: &Graph<N, E, T>, k: usize) -> (Vec<HashSet<NodeIndex>>, Vec<(NodeIndex, NodeIndex)>) {
    // let mut g = graph.clone().into_edge_type::<Undirected>();
    let mut g = StableGraph::<(), (), Undirected>::with_capacity(graph.node_count(), graph.edge_count());
    for _ in graph.node_indices() {
        g.add_node(());
    }
    let mut visited_edges = HashSet::new();
    for edge in graph.edge_indices() {
        let (source, target) = graph.edge_endpoints(edge).unwrap();
        if !visited_edges.contains(&(source, target)) && !visited_edges.contains(&(target, source)) {
            visited_edges.insert((source, target));
            g.add_edge(source, target, ());
        }
    }

    let mut edges_removed = Vec::new();
    // Repeat k times
    for _ in 0..k {
        println!("#GN# K = {}", k);
        let original_cc_count = find_connected_components(&g).len();
        let mut new_cc_count = original_cc_count;
        while new_cc_count == original_cc_count {
            println!("#GN# CC = {}, EDGES = {}", new_cc_count, edges_removed.len());
            let betweenness = calculate_betweenness_centrality(&g);
            let max_betweenness = betweenness.values().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
            let edges_to_remove: Vec<(NodeIndex, NodeIndex)> = betweenness
                .iter()
                .filter(|(_, value)| *value == max_betweenness)
                .map(|((a, b), _)| (*a, *b))
                .collect();

            for edge in edges_to_remove {
                edges_removed.push(edge);
                g.remove_edge(g.find_edge(edge.0, edge.1).unwrap());
            }
        
            new_cc_count = find_connected_components(&g).len();
        }
    }

    (find_connected_components(&g), edges_removed)
}

pub fn find_connected_components<N, E, T: EdgeType>(graph: &StableGraph<N, E, T>) -> Vec<HashSet<NodeIndex>> {
    let mut visited: HashSet<NodeIndex> = HashSet::new();
    let mut components: Vec<HashSet<NodeIndex>> = Vec::new();

    for node_index in graph.node_indices() {
        if !visited.contains(&node_index) {
            let mut component: HashSet<NodeIndex> = HashSet::new();
            let mut queue: VecDeque<NodeIndex> = VecDeque::new();
            queue.push_back(node_index);

            while let Some(current_node) = queue.pop_front() {
                if visited.insert(current_node) {
                    component.insert(current_node);

                    for neighbor in graph.neighbors(current_node) {
                        if !visited.contains(&neighbor) {
                            queue.push_back(neighbor);
                        }
                    }
                }
            }

            components.push(component);
        }
    }

    components
}